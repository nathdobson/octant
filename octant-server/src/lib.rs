#![deny(unused_must_use)]

use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use futures::SinkExt;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use warp::Filter;
use warp::ws::{Message, WebSocket};

use octant_gui::Root;
use octant_gui_core::CommandList;

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
    async fn encode(x: CommandList) -> anyhow::Result<Message> {
        Ok(Message::binary(serde_json::to_vec(&x)?))
    }
    pub async fn run_socket(&self, _name: &str, tx: SplitSink<WebSocket, Message>, mut rx: SplitStream<WebSocket>) -> anyhow::Result<()> {
        let root = Root::new(Box::pin(tx.with(Self::encode)));
        {
            let document = root.window().document();
            let body = document.body();
            let text = document.create_text_node("Lorum Ipsum Dolor Sit Amet");
            body.append_child(&text);
        }
        root.flush().await?;
        while let Some(received) = rx.next().await {
            let received = received?;
            if received.is_close() {
                break;
            }
            if received.is_binary() {
                log::info!("{:?}",received.as_bytes());
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
            warp::path("gui")
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
