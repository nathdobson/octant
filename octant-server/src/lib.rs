#![deny(unused_must_use)]

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::anyhow;

use clap::Parser;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use futures::SinkExt;
use warp::ws::{Message, WebSocket};
use warp::{Filter, Reply};

use octant_gui::Root;
use octant_gui_core::{CommandList, RemoteEvent};

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
    fn run_handler(&self, root: Arc<Root>) -> impl Future<Output = anyhow::Result<()>> + Send;
}

impl<A: Application> OctantServer<A> {
    async fn encode(x: CommandList) -> anyhow::Result<Message> {
        Ok(Message::binary(serde_json::to_vec(&x)?))
    }
    fn decode(x: Message) -> anyhow::Result<RemoteEvent> {
        Ok(serde_json::from_str(
            x.to_str().map_err(|_| anyhow!("not text"))?,
        )?)
    }
    pub async fn run_socket(
        self: Arc<Self>,
        _name: &str,
        tx: SplitSink<WebSocket, Message>,
        mut rx: SplitStream<WebSocket>,
    ) -> anyhow::Result<()> {
        let root = Root::new(
            Box::pin(rx.map(|x| Self::decode(x?))),
            Box::pin(tx.with(Self::encode)),
        );
        self.application.run_handler(root).await?;
        Ok(())
    }
    pub async fn run(self) {
        Arc::new(self).run_arc().await
    }
    fn add_header(reply: impl Reply) -> impl Reply {
        warp::reply::with_header(reply, "Cache-Control", "no-cache")
    }
    pub async fn run_arc(self: Arc<Self>) {
        let statik = warp::path("static")
            .and(warp::fs::dir("./target/www"))
            .map(Self::add_header);
        let socket = warp::path("gui")
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
            });
        if let Some(bind_http) = self.options.bind_http {
            warp::serve(statik.or(socket)).run(bind_http).await;
        }
    }
}
