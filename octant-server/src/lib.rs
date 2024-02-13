#![deny(unused_must_use)]

use std::future::Future;
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
pub struct OctantServerOptions {
    #[arg(long, required = true)]
    pub bind_http: Option<SocketAddr>,
}

pub struct OctantServer<A> {
    pub options: OctantServerOptions,
    pub application: A,
}

impl OctantServerOptions {
    pub fn from_command_line() -> Self {
        Self::parse()
    }
}

pub trait Application: 'static + Sync + Send {
    fn run_handler(&self, root: Arc<Root>) -> impl Future<Output=anyhow::Result<()>> + Send;
}

impl<A: Application> OctantServer<A> {
    async fn encode(x: CommandList) -> anyhow::Result<Message> {
        Ok(Message::binary(serde_json::to_vec(&x)?))
    }
    pub async fn run_socket(self: Arc<Self>, _name: &str, tx: SplitSink<WebSocket, Message>, mut rx: SplitStream<WebSocket>) -> anyhow::Result<()> {
        let root = Root::new(Box::pin(tx.with(Self::encode)));
        let session = tokio::spawn({
            let this = self.clone();
            async move { this.application.run_handler(root).await }
        });
        while let Some(received) = rx.next().await {
            let received = received?;
            if received.is_close() {
                break;
            }
            if received.is_binary() {
                log::info!("received {:?}", received.as_bytes());
            }
        }
        session.abort();
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
        if let Some(bind_http) = self.options.bind_http {
            warp::serve(statik.or(socket)).run(bind_http).await;
        }
    }
}
