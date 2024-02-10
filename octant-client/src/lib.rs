#![deny(unused_must_use)]

mod websocket;
mod error;

use anyhow::anyhow;
use web_sys::window;
use wasm_bindgen::prelude::*;
use crate::error::WasmError;
use crate::websocket::{RecvError, WebSocketStream};
use crate::error::log_error;

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    if let Err(e) = main_impl().await {
        log_error(&e);
    }
}

pub async fn main_impl() -> anyhow::Result<()> {
    let location = window().expect("no window").location();
    let http_proto = location.protocol().map_err(WasmError::new)?;
    let host = location.host().map_err(WasmError::new)?;
    let ws_proto = match &*http_proto {
        "http:" => "ws:",
        "https:" => "wss:",
        _ => { return Err(anyhow!("Cannot infer websocket protocol for {:?}",http_proto)); }
    };
    let url = format!("{ws_proto}//{host}/socket/render");
    log::info!("Connecting to {:?}",url);
    let mut socket = WebSocketStream::connect(&url).await?;
    log::info!("Response before initial request is {:?}",socket.try_recv());
    socket.send(b"ping")?;
    log::info!("Sent ping");
    match socket.recv().await {
        Ok(resp) => log::info!("resp is {:?}",String::from_utf8(resp)),
        Err(RecvError::Disconnected) => log::error!("disconnected"),
        Err(RecvError::Anyhow(x)) => return Err(x),
    };
    Ok(())
}