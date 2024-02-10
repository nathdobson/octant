#![deny(unused_must_use)]

use std::net::SocketAddr;
use std::sync::Arc;
use clap::Parser;
use futures::SinkExt;
use warp::{Filter};
use futures::stream::{SplitSink, SplitStream, StreamExt};
use warp::ws::{Message, WebSocket};

#[derive(Parser, Debug)]
pub struct OctantServer {
    #[arg(long, required = true)]
    pub bind_http: Option<SocketAddr>,
}

impl OctantServer {
    pub fn from_command_line() -> OctantServer {
        OctantServer::parse()
    }
}

impl OctantServer {
    pub async fn run_socket(&self, _name: &str, mut tx: SplitSink<WebSocket, Message>, mut rx: SplitStream<WebSocket>) -> anyhow::Result<()> {
        while let Some(received) = rx.next().await {
            let received = received?;
            if received.is_close() {
                break;
            }
            if received.is_binary() {
                if received.as_bytes() == b"ping" {
                    tx.send(Message::binary(b"pong")).await?;
                }
            }
        }
        Ok(())
    }
    pub async fn run(self) {
        Arc::new(self).run_arc().await
    }
    pub async fn run_arc(self: Arc<Self>) {
        let statik =
            warp::path("static").and(warp::fs::dir("./target/www"));
        let socket =
            warp::path("socket")
                .and(warp::path::param())
                .and(warp::ws())
                .map({
                    let this = self.clone();
                    move |name: String, ws: warp::ws::Ws| {
                        log::info!("Handling");
                        let this = this.clone();
                        ws.on_upgrade(|websocket| async move {
                            log::info!("Upgraded");
                            let (tx, rx) = websocket.split();
                            if let Err(e) = this.run_socket(&name, tx, rx).await {
                                log::error!("Websocket error: {:?}", e);
                            }
                        })
                    }
                })
            ;
        if let Some(bind_http) = self.bind_http {
            warp::serve(statik.or(socket)).run(bind_http).await;
        }
    }
}
