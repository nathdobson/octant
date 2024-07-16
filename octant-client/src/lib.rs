#![deny(unused_must_use)]
#![feature(never_type)]

extern crate octant_web_sys_client;

use futures::StreamExt;
use std::rc::Rc;
use marshal_fixed::encode::full::FixedEncoderBuilder;
use marshal_json::encode::full::JsonEncoderBuilder;
use tokio::{sync::mpsc::unbounded_channel, try_join};
use wasm_bindgen::prelude::*;
use web_sys::window;

use crate::websocket::WebSocketMessage;
use octant_error::{octant_error, OctantError, OctantResult};
use octant_runtime_client::{
    proto::{DownMessage, DownMessageList, Proto, UpMessageList},
    reexports::marshal::context::OwnedContext,
    runtime::Runtime,
};

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
    let path = location.pathname()?;
    let proto = path
        .strip_prefix("/site/")
        .expect("url must start with /site/")
        .split_once("/")
        .unwrap()
        .0
        .parse::<Proto>()?;
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
    let url = format!("{ws_proto}//{host}/socket/{proto}");
    log::info!("Connecting to {:?}", url);
    let (tx, mut rx) = websocket::connect(&url).await?;

    let (tx_send, mut rx_send) = unbounded_channel();
    let runtime = Runtime::new(proto, tx_send)?;
    let recv_fut = async {
        let mut ctx = OwnedContext::new();
        ctx.insert_const::<Rc<Runtime>>(&runtime);
        while let Some(next) = rx.next().await {
            let next = next?;
            let message = proto.deserialize::<DownMessageList>(next.as_bytes(), ctx.borrow())?;
            for bytes in message.commands {
                proto
                    .deserialize::<Box<dyn DownMessage>>(&bytes, ctx.borrow())?
                    .run(&runtime)?;
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
                .map(|x| Ok(proto.serialize(x, OwnedContext::new().borrow())?))
                .collect::<anyhow::Result<Vec<_>>>()?;
            let message = UpMessageList { commands };
            let mut ctx = OwnedContext::new();
            let message = match proto {
                Proto::Json => WebSocketMessage::Text(
                    JsonEncoderBuilder::new().serialize(&message, ctx.borrow())?,
                ),
                Proto::Fixed => WebSocketMessage::Binary(
                    FixedEncoderBuilder::new().serialize(&message, ctx.borrow())?,
                ),
            };
            tx.send(message)?;
        }
        Ok(())
    };
    try_join!(recv_fut, send_fut)?.0
}
