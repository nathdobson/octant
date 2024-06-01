#![deny(unused_must_use)]
#![feature(never_type)]

extern crate octant_web_sys_client;

use std::rc::Rc;

use futures::StreamExt;
use tokio::{sync::mpsc::unbounded_channel, try_join};
use wasm_bindgen::prelude::*;
use web_sys::window;

use octant_error::{octant_error, OctantError, OctantResult};
use octant_runtime_client::{
    proto::{DownMessageList, UpMessageList},
    runtime::Runtime,
};
use octant_serde::{DeserializeContext, Format, RawEncoded};

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
            octant_error::wasm::log_error(&e);
            display_error(&format!("{:?}", e));
        }
    }
}

pub async fn main_impl() -> OctantResult<!> {
    let location = window().expect("no window").location();
    let http_proto = location.protocol().map_err(OctantError::from)?;
    let host = location.host().map_err(OctantError::from)?;
    let ws_proto = match &*http_proto {
        "http:" => "ws:",
        "https:" => "wss:",
        _ => {
            return Err(octant_error!(
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
            let encoded = match next {
                WebSocketMessage::Text(text) => RawEncoded::Text(text),
                WebSocketMessage::Binary(_) => todo!(),
            };
            let message: DownMessageList = encoded.deserialize_as::<DownMessageList>()?;
            let mut ctx = DeserializeContext::new();
            ctx.insert::<Rc<Runtime>>(runtime.clone());
            for message in message.commands {
                let message = message.deserialize_with(&ctx)?;
                message.run(&runtime)?;
            }
        }
        Err(octant_error!("Websocket terminated"))
    };
    let send_fut = async {
        loop {
            let mut commands = vec![];
            if rx_send.recv_many(&mut commands, usize::MAX).await == 0 {
                break;
            }
            let commands = commands
                .iter()
                .map(|x| Format::default().serialize(&**x))
                .collect::<OctantResult<Vec<_>>>()?;
            let message = UpMessageList { commands };
            tx.send(WebSocketMessage::Text(serde_json::to_string(&message)?))?;
        }
        Ok(())
    };
    try_join!(recv_fut, send_fut)?.0
}
