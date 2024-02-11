#![deny(unused_must_use)]

mod websocket;
mod error;

use anyhow::anyhow;
use futures::StreamExt;
use web_sys::window;
use wasm_bindgen::prelude::*;
use octant_gui_client::Renderer;
use crate::websocket::{WebSocketStream};
use wasm_error::{log_error, WasmError};

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
    let url = format!("{ws_proto}//{host}/gui/render");
    log::info!("Connecting to {:?}",url);
    let mut socket = WebSocketStream::connect(&url).await?;
    let (tx, rx) = socket.split();
    let rx =
        rx.map(|x| Ok(serde_json::from_str(x?.as_str()?)?));
    Renderer::new(Box::pin(rx)).run().await?;
    Ok(())
}