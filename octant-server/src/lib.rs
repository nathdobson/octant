#![feature(future_join)]
#![deny(unused_must_use)]

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use anyhow::anyhow;
use clap::Parser;
use futures::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use tokio::try_join;
use url::Url;
use warp::{
    Filter,
    Reply, ws::{Message, WebSocket},
};

use octant_gui::{
    event_loop::{Application, EventLoop, Page},
    Global, Runtime,
};
use octant_gui_core::{DownMessageList, UpMessageList};

use crate::session::Session;

pub mod session;

#[derive(Parser, Debug)]
pub struct OctantServerOptions {
    #[arg(long, required = true)]
    pub bind_http: Option<SocketAddr>,
    #[arg(long)]
    pub bind_https: Option<SocketAddr>,
    #[arg(long)]
    pub cert_path: Option<String>,
    #[arg(long)]
    pub key_path: Option<String>,
}

pub struct OctantServer {
    options: OctantServerOptions,
    handlers: HashMap<String, Box<dyn Handler>>,
}

impl OctantServerOptions {
    pub fn from_command_line() -> Self {
        Self::parse()
    }
}

pub trait Handler: 'static + Sync + Send {
    fn prefix(&self) -> String;
    fn handle(&self, url: &Url, session: Arc<Session>) -> anyhow::Result<Page>;
}

struct OctantApplication {
    server: Arc<OctantServer>,
    session: Arc<Session>,
}

impl Application for OctantApplication {
    fn create_page(&self, url: &str, _global: Arc<Global>) -> anyhow::Result<Page> {
        let url = Url::parse(url)?;
        let prefix = url
            .path_segments()
            .map(|mut x| {
                x.next();
                x.next()
            })
            .flatten()
            .ok_or_else(|| anyhow::Error::msg("Cannot find path prefix"))?;
        self.server
            .handlers
            .get(prefix)
            .ok_or_else(|| anyhow::Error::msg("Cannot find handler"))?
            .handle(&url, self.session.clone())
    }
}

impl OctantServer {
    pub fn new(options: OctantServerOptions) -> Self {
        OctantServer {
            options,
            handlers: HashMap::new(),
        }
    }
    pub fn add_handler(&mut self, handler: impl Handler) {
        self.handlers.insert(handler.prefix(), Box::new(handler));
    }
    async fn encode(x: DownMessageList) -> anyhow::Result<Message> {
        Ok(Message::binary(serde_json::to_vec(&x)?))
    }
    fn decode(x: Message) -> anyhow::Result<Option<UpMessageList>> {
        if x.is_close() {
            Ok(None)
        } else {
            Ok(serde_json::from_str(
                x.to_str().map_err(|_| anyhow!("not text"))?,
            )?)
        }
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
        let session = Arc::new(Session::new(global.clone()));
        let mut event_loop = EventLoop::new(
            global,
            events,
            Arc::new(OctantApplication {
                server: self,
                session,
            }),
        );
        event_loop.handle_events().await?;
        Ok(())
    }
    pub async fn run(self) -> anyhow::Result<()> {
        Arc::new(self).run_arc().await?;
        Ok(())
    }
    fn add_header(reply: impl Reply) -> impl Reply {
        warp::reply::with_header(reply, "Cache-Control", "no-cache")
    }
    pub async fn run_arc(self: Arc<Self>) -> anyhow::Result<()> {
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
                            log::error!("Error handling websocket: {:?}", e);
                        }
                    })
                }
            });
        let routes = statik.or(site).or(socket);
        let http = async {
            if let Some(bind_http) = self.options.bind_http {
                warp::serve(routes.clone()).run(bind_http).await;
            }
            Result::<_, anyhow::Error>::Ok(())
        };
        let https = async {
            if let Some(bind_https) = self.options.bind_https {
                warp::serve(routes.clone())
                    .tls()
                    .cert_path(
                        self.options
                            .cert_path
                            .as_ref()
                            .ok_or_else(|| anyhow!("missing cert_path flag"))?,
                    )
                    .key_path(
                        &self
                            .options
                            .key_path
                            .as_ref()
                            .ok_or_else(|| anyhow!("missing key_path flag:"))?,
                    )
                    .run(bind_https)
                    .await;
            }
            Result::<_, anyhow::Error>::Ok(())
        };
        try_join!(http, https)?;
        Ok(())
    }
}
