use core::mem;
use tokio::sync::mpsc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CloseEvent, ErrorEvent, Event, MessageEvent, WebSocket};
use anyhow::Context;
use crate::error::WasmError;


#[derive(Debug)]
pub enum RecvError {
    Disconnected,
    Anyhow(anyhow::Error),
}

#[derive(Debug)]
pub enum TryRecvError {
    Empty,
    Disconnected,
    Anyhow(anyhow::Error),
}

impl From<anyhow::Error> for RecvError {
    fn from(value: anyhow::Error) -> Self { RecvError::Anyhow(value) }
}

impl From<anyhow::Error> for TryRecvError {
    fn from(value: anyhow::Error) -> Self { TryRecvError::Anyhow(value) }
}

impl From<RecvError> for TryRecvError {
    fn from(value: RecvError) -> Self {
        match value {
            RecvError::Disconnected => TryRecvError::Disconnected,
            RecvError::Anyhow(x) => TryRecvError::Anyhow(x),
        }
    }
}

pub struct WebSocketStream {
    socket: WebSocket,
    receiver: mpsc::UnboundedReceiver<Message>,
}

pub enum Message {
    Connect,
    Error(ErrorEvent),
    Message(Vec<u8>),
}

impl Drop for WebSocketStream {
    fn drop(&mut self) {
        if let Err(e) = self.socket.close() {
            log::error!("Error closing socket: {:?}", e);
        }
    }
}

impl WebSocketStream {
    pub async fn connect(address: &str) -> Result<Self, anyhow::Error> {
        let socket = WebSocket::new(address).map_err(WasmError::new).context("Failed to create socket.")?;
        let (recv_tx, mut recv_rx) = mpsc::unbounded_channel();
        socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let onerror_callback: Closure<dyn FnMut(ErrorEvent)> = Closure::new({
            let recv_tx = recv_tx.clone();
            move |e: ErrorEvent| {
                recv_tx.send(Message::Error(e.into())).ok();
            }
        });
        socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

        let onopen_callback: Closure<dyn FnMut(Event)> = Closure::new({
            let recv_tx = recv_tx.clone();
            move |_| {
                recv_tx.send(Message::Connect).ok();
            }
        });
        socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

        let onmessage_callback: Closure<dyn FnMut(MessageEvent)> = Closure::new({
            let recv_tx = recv_tx.clone();
            move |e: MessageEvent| {
                if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                    let array = js_sys::Uint8Array::new(&abuf);
                    let mut vec = vec![];
                    vec.resize(array.length() as usize, 0);
                    array.copy_to(&mut vec);
                    recv_tx.send(Message::Message(vec)).ok();
                } else if let Ok(_) = e.data().dyn_into::<web_sys::Blob>() {
                    unreachable!();
                } else if let Ok(abuf) = e.data().dyn_into::<js_sys::JsString>() {
                    recv_tx
                        .send(Message::Message(String::from(abuf).into_bytes()))
                        .ok();
                } else {
                    unreachable!();
                }
            }
        });
        socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        let onclose_callback = Closure::once_into_js(move |_: CloseEvent| {
            mem::drop(onerror_callback);
            mem::drop(onopen_callback);
            mem::drop(onmessage_callback);
        });
        socket.set_onclose(Some(onclose_callback.unchecked_ref()));

        match recv_rx.recv().await.unwrap() {
            Message::Connect => {}
            Message::Error(e) => {
                log::error!("receive error");
                return Err(WasmError::new_anyhow(JsValue::from(e)).context("Failed to connect."));
            }
            Message::Message(_) => unreachable!(),
        }

        Ok(WebSocketStream {
            socket,
            receiver: recv_rx,
        })
    }

    pub fn send(&mut self, message: &[u8]) -> Result<(), anyhow::Error> {
        Ok(self
            .socket
            .send_with_u8_array(&message)
            .map_err(WasmError::new)
            .context("Failed to send.")?)
    }
    pub async fn recv(&mut self) -> Result<Vec<u8>, RecvError> {
        match self.receiver.recv().await {
            None => Err(RecvError::Disconnected),
            Some(Message::Message(x)) => Ok(x),
            Some(Message::Error(e)) => Err(RecvError::Anyhow(WasmError::new_anyhow(JsValue::from(e)).context("Failed to recv."))),
            Some(Message::Connect) => unreachable!(),
        }
    }
    pub fn try_recv(&mut self) -> Result<Vec<u8>, TryRecvError> {
        match self.receiver.try_recv() {
            Ok(Message::Message(x)) => Ok(x),
            Ok(Message::Error(e)) => Err(TryRecvError::Anyhow(WasmError::new_anyhow(JsValue::from(e)).context("Failed to recv."))),
            Ok(Message::Connect) => unreachable!(),
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => Err(TryRecvError::Empty),
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                Err(TryRecvError::Disconnected)
            }
        }
    }
}
