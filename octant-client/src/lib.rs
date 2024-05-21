#![deny(unused_must_use)]
#![feature(never_type)]

extern crate octant_web_sys_client;

use std::sync::Arc;

use anyhow::anyhow;
use futures::StreamExt;
use safe_once::sync::OnceLock;
use wasm_bindgen::prelude::*;
use web_sys::window;

use octant_gui_client::Runtime;
use octant_gui_core::UpMessageList;
use octant_serde::TypeMap;
use wasm_error::{log_error, WasmError};

use crate::websocket::WebSocketMessage;

mod error;
mod websocket;

#[wasm_bindgen(module = "index.js")]
extern "C" {
    #[wasm_bindgen(js_name = displayError)]
    fn display_error(message: &str);
}

#[wasm_bindgen(start)]
pub async fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    match main_impl().await {
        Ok(x) => match x {},
        Err(e) => {
            log_error(&e);
            display_error(&format!("{:?}", e));
        }
    }
}

pub async fn main_impl() -> anyhow::Result<!> {
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
    let runtime_once = Arc::new(OnceLock::<Arc<Runtime>>::new());
    let rx = rx.map({
        let runtime_once = runtime_once.clone();
        move |x| {
            let mut ctx = TypeMap::new();
            ctx.insert::<Arc<Runtime>>(runtime_once.try_get().unwrap().clone());
            return Ok(octant_serde::deserialize(&ctx, x?.as_str()?)?);
        }
    });
    let tx = Box::new(move |x: UpMessageList| {
        return Ok(tx.send(WebSocketMessage::Text(serde_json::to_string(&x)?))?);
    });
    let runtime = Runtime::new(Box::pin(rx), tx)?;
    runtime_once.lock().or_init(|| runtime.clone());
    runtime.run().await?;
}
