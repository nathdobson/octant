#![deny(unused_must_use)]
#![feature(never_type)]

extern crate octant_web_sys_client;

use std::sync::Arc;
use anyhow::anyhow;
use futures::StreamExt;
use octant_runtime_client::{
    proto::{DownMessageList, UpMessageList},
    runtime::Runtime,
};
use tokio::{sync::mpsc::unbounded_channel, try_join};
use wasm_bindgen::prelude::*;
use web_sys::window;
use octant_serde::TypeMap;

use wasm_error::{log_error, WasmError};

use crate::websocket::WebSocketMessage;

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
    let (tx, mut rx) = websocket::connect(&url).await?;
    // let rx = rx.map({
    //     let runtime_once = runtime_once.clone();
    //     move |x| {
    //         let mut ctx = TypeMap::new();
    //         ctx.insert::<Arc<Runtime>>(runtime_once.try_get().unwrap().clone());
    //         return Ok(octant_serde::deserialize(&ctx, x?.as_str()?)?);
    //     }
    // });
    // let tx = Box::new(move |x: UpMessageList| {
    //     return Ok(tx.send(WebSocketMessage::Text(serde_json::to_string(&x)?))?);
    // });
    let (tx_send, mut rx_send) = unbounded_channel();
    let runtime = Runtime::new(tx_send)?;
    let recv_fut = async {
        while let Some(next) = rx.next().await {
            let next = next?;
            let text = next.as_str()?;
            let mut ctx=TypeMap::new();
            ctx.insert::<Arc<Runtime>>(runtime.clone());
            let message: DownMessageList = octant_serde::deserialize(&ctx, text)?;
            for message in message.commands{
                message.run(&runtime)?;
            }
        }
        Err(anyhow!("Websocket terminated"))
    };
    let send_fut = async {
        loop {
            let mut commands = vec![];
            if rx_send.recv_many(&mut commands, usize::MAX).await == 0 {
                break;
            }
            let message = UpMessageList { commands };
            tx.send(WebSocketMessage::Text(serde_json::to_string(&message)?))?;
        }
        Ok(())
    };
    try_join!(recv_fut, send_fut)?.0
}
