use core::mem;
use std::{pin::Pin, rc::Rc, str, task::Poll};

use futures::Stream;
use tokio::sync::mpsc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{CloseEvent, ErrorEvent, Event, MessageEvent, WebSocket};

use octant_error::{Context, octant_error, OctantError, OctantResult};

struct WebSocketStream {
    socket: WebSocket,
}

pub struct WebSocketSender {
    stream: Rc<WebSocketStream>,
}

pub struct WebSocketReceiver {
    _stream: Rc<WebSocketStream>,
    receiver: mpsc::UnboundedReceiver<WebSocketEvent>,
}

pub enum WebSocketMessage {
    Text(String),
    Binary(Vec<u8>),
}

enum WebSocketEvent {
    Connect,
    Error(ErrorEvent),
    Message(WebSocketMessage),
    Close,
}

impl Drop for WebSocketStream {
    fn drop(&mut self) {
        if let Err(e) = self.socket.close() {
            log::error!("Error closing socket: {:?}", e);
        }
    }
}
//
// impl WebSocketMessage {
//     pub fn as_str(&self) -> Result<&str, Utf8Error> {
//         match self {
//             WebSocketMessage::Text(x) => Ok(x),
//             WebSocketMessage::Binary(x) => str::from_utf8(x),
//         }
//     }
// }

impl WebSocketStream {
    pub fn send(&self, message: WebSocketMessage) -> OctantResult<()> {
        match message {
            WebSocketMessage::Text(x) => self.socket.send_with_str(&x),
            WebSocketMessage::Binary(x) => self.socket.send_with_u8_array(&x),
        }
        .context("Failed to send.")
    }
}

impl Stream for WebSocketReceiver {
    type Item = OctantResult<WebSocketMessage>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.receiver.poll_recv(cx) {
            Poll::Ready(Some(WebSocketEvent::Connect)) => unreachable!(),
            Poll::Ready(Some(WebSocketEvent::Message(x))) => Poll::Ready(Some(Ok(x))),
            Poll::Ready(Some(WebSocketEvent::Error(e))) => {
                Poll::Ready(Some(Err(OctantError::from(JsValue::from(e)))))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(WebSocketEvent::Close)) => Poll::Ready(None),
        }
    }
}

impl WebSocketSender {
    pub fn send(&self, message: WebSocketMessage) -> OctantResult<()> {
        self.stream.send(message)
    }
}

pub async fn connect(address: &str) -> OctantResult<(WebSocketSender, WebSocketReceiver)> {
    let socket = WebSocket::new(address).context("Failed to create socket.")?;
    let (recv_tx, mut recv_rx) = mpsc::unbounded_channel();
    socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let onerror_callback: Closure<dyn FnMut(ErrorEvent)> = Closure::new({
        let recv_tx = recv_tx.clone();
        move |e: ErrorEvent| {
            recv_tx.send(WebSocketEvent::Error(e.into())).ok();
        }
    });
    socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));

    let onopen_callback: Closure<dyn FnMut(Event)> = Closure::new({
        let recv_tx = recv_tx.clone();
        move |_| {
            recv_tx.send(WebSocketEvent::Connect).ok();
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
                recv_tx
                    .send(WebSocketEvent::Message(WebSocketMessage::Binary(vec)))
                    .ok();
            } else if let Ok(_) = e.data().dyn_into::<web_sys::Blob>() {
                unreachable!();
            } else if let Ok(abuf) = e.data().dyn_into::<js_sys::JsString>() {
                recv_tx
                    .send(WebSocketEvent::Message(WebSocketMessage::Text(
                        String::from(abuf),
                    )))
                    .ok();
            } else {
                unreachable!();
            }
        }
    });
    socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

    let onclose_callback = Closure::once_into_js(move |_: CloseEvent| {
        recv_tx.send(WebSocketEvent::Close).ok();
        mem::drop(onerror_callback);
        mem::drop(onopen_callback);
        mem::drop(onmessage_callback);
    });
    socket.set_onclose(Some(onclose_callback.unchecked_ref()));

    match recv_rx.recv().await.unwrap() {
        WebSocketEvent::Connect => {}
        WebSocketEvent::Error(e) => {
            log::error!("receive error");
            return Err(OctantError::from(JsValue::from(e))
                .context("Failed to connect."));
        }
        WebSocketEvent::Message(_) => unreachable!(),
        WebSocketEvent::Close => {
            return Err(octant_error!("Connection closed."));
        }
    }

    let stream = Rc::new(WebSocketStream { socket });
    Ok((
        WebSocketSender {
            stream: stream.clone(),
        },
        WebSocketReceiver {
            _stream: stream.clone(),
            receiver: recv_rx,
        },
    ))
}
