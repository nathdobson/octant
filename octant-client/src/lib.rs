#![deny(unused_must_use)]

use anyhow::anyhow;
use futures::StreamExt;
use wasm_bindgen::prelude::*;
use web_sys::window;

use octant_gui_client::Runtime;
use octant_gui_core::UpMessageList;
use wasm_error::log_error;
use wasm_error::WasmError;

use crate::websocket::WebSocketMessage;

mod error;
mod websocket;

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
        _ => {
            return Err(anyhow!(
                "Cannot infer websocket protocol for {:?}",
                http_proto
            ));
        }
    };
    let url = format!("{ws_proto}//{host}/socket/render");
    log::info!("Connecting to {:?}", url);
    let (tx, rx) = websocket::connect(&url).await?;
    let rx = rx.map(|x| {
        return Ok(serde_json::from_str(x?.as_str()?)?);
    });
    let tx = Box::new(move |x: UpMessageList| {
        return Ok(tx.send(WebSocketMessage::Text(serde_json::to_string(&x)?))?);
    });
    Runtime::new(Box::pin(rx), tx)?.run().await?;
    Ok(())
}
