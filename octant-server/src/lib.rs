#![feature(future_join)]
#![deny(unused_must_use)]

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::anyhow;
use clap::Parser;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use futures::SinkExt;
use warp::ws::{Message, WebSocket};
use warp::{Filter, Reply};

use octant_gui::event_loop::{EventLoop, Session};
use octant_gui::{Global, Runtime};
use octant_gui_core::{DownMessageList, UpMessageList};

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
    fn create_session(&self, global: Arc<Global>) -> anyhow::Result<Box<dyn Session>>;
}

impl<A: Application> OctantServer<A> {
    async fn encode(x: DownMessageList) -> anyhow::Result<Message> {
        Ok(Message::binary(serde_json::to_vec(&x)?))
    }
    fn decode(x: Message) -> anyhow::Result<UpMessageList> {
        Ok(serde_json::from_str(
            x.to_str().map_err(|_| anyhow!("not text"))?,
        )?)
    }
    pub async fn run_socket(
        self: Arc<Self>,
        _name: &str,
        tx: SplitSink<WebSocket, Message>,
        rx: SplitStream<WebSocket>,
    ) -> anyhow::Result<()> {
        let root = Runtime::new(Box::pin(tx.with(Self::encode)));
        let global = Global::new(root);
        let events = Box::pin(rx.map(|x| Self::decode(x?)));
        let session = self.application.create_session(global.clone())?;
        let mut event_loop = EventLoop::new(global, events, session);
        event_loop.handle_events().await?;
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
        let site = warp::path("site")
            .and(warp::fs::file("./target/www/index.html"))
            .map(Self::add_header);
        let socket = warp::path("socket")
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
            warp::serve(statik.or(site).or(socket)).run(bind_http).await;
        }
    }
}
