#![feature(future_join)]
#![deny(unused_must_use)]
#![allow(unused_variables)]
#![feature(trait_upcasting)]
#![feature(never_type)]

use std::{
    collections::HashMap, future::pending, net::SocketAddr, sync::Arc,
    thread::available_parallelism,
};
use std::rc::Rc;

use anyhow::anyhow;
use clap::Parser;
use cookie::Cookie;
use futures::{
    SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use itertools::Itertools;
use tokio::{sync::mpsc, try_join};
use url::Url;
use uuid::Uuid;
use warp::{
    Filter,
    Reply, ws::{Message, WebSocket},
};

use octant_executor::{
    event_loop::EventPool,
    local_set::{LocalSetPool, LocalSetSpawn},
};
use octant_runtime_server::{
    proto::{DownMessageList, UpMessageList},
    runtime::Runtime,
};
use octant_serde::{Format, RawEncoded};
use octant_web_sys_server::{global::Global, node::RcNode};

use crate::{cookies::CookieRouter, session::Session, sink::BufferedDownMessageSink};

pub mod cookies;
pub mod session;
mod sink;

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
    #[arg(long, required = true)]
    pub db_path: String,
}

pub struct OctantServer {
    options: OctantServerOptions,
    handlers: HashMap<String, Arc<dyn Handler>>,
    cookie_router: Arc<CookieRouter>,
    spawn: Arc<LocalSetSpawn>,
}

impl OctantServerOptions {
    pub fn from_command_line() -> Self {
        Self::parse()
    }
}

pub trait Handler: 'static + Sync + Send {
    fn prefix(&self) -> String;
    fn handle(self: Arc<Self>, url: &Url, session: Rc<Session>) -> anyhow::Result<Page>;
}

struct OctantApplication {
    server: Arc<OctantServer>,
    session: Rc<Session>,
}

impl OctantApplication {
    fn create_page(&self, url: &str, _global: Rc<Global>) -> anyhow::Result<Page> {
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
            .clone()
            .handle(&url, self.session.clone())
    }
}

impl OctantServer {
    pub fn new(options: OctantServerOptions) -> Self {
        let (spawn, pool) = LocalSetPool::new(available_parallelism().unwrap().get());
        pool.detach();
        OctantServer {
            options,
            handlers: HashMap::new(),
            cookie_router: CookieRouter::new(),
            spawn,
        }
    }
    pub fn cookie_router(&self) -> &Arc<CookieRouter> {
        &self.cookie_router
    }
    pub fn add_handler(&mut self, handler: impl Handler) {
        self.handlers.insert(handler.prefix(), Arc::new(handler));
    }
    async fn encode(x: DownMessageList) -> anyhow::Result<Message> {
        match Format::default().serialize_raw(&x)? {
            RawEncoded::Text(x) => Ok(Message::text(x)),
        }
    }
    fn decode(runtime: &Rc<Runtime>, x: Message) -> anyhow::Result<Option<UpMessageList>> {
        if x.is_close() {
            Ok(None)
        } else if x.is_text() {
            Ok(Some(
                RawEncoded::Text(x.to_str().unwrap().to_string())
                    .deserialize_as::<UpMessageList>()?,
            ))
        } else if x.is_binary() {
            todo!();
        } else {
            Ok(None)
        }
    }
    pub async fn run_socket(
        self: Arc<Self>,
        tx: SplitSink<WebSocket, Message>,
        rx: SplitStream<WebSocket>,
    ) -> anyhow::Result<()> {
        let spawn = self.spawn.clone();
        spawn
            .spawn_async(move || async move {
                self.run_socket_local(tx, rx).await?;
                Ok(())
            })
            .await?
    }
    pub async fn run_socket_local(
        self: Arc<Self>,
        tx: SplitSink<WebSocket, Message>,
        mut rx: SplitStream<WebSocket>,
    ) -> anyhow::Result<()> {
        let (tx_inner, rx_inner) = mpsc::unbounded_channel();
        let mut sink = BufferedDownMessageSink::new(rx_inner, Box::pin(tx.with(Self::encode)));
        let (spawn, mut pool) = EventPool::new(move |cx| sink.poll_flush(cx));
        let runtime = Rc::new(Runtime::new(tx_inner, spawn.clone()));
        let global = Global::new(runtime);
        let session = Rc::new(Session::new(global.clone()));
        let app = Rc::new(OctantApplication {
            server: self,
            session,
        });
        spawn.spawn({
            let runtime = global.runtime().clone();
            async move {
                while let Some(message) = rx.next().await {
                    let message = message?;
                    if let Some(message) = Self::decode(&runtime, message)? {
                        runtime.run_batch(message)?;
                    } else {
                        break;
                    }
                }
                Ok(())
            }
        });
        spawn.spawn({
            let global = global.clone();
            async move {
                let url = global.window().document().location().await?;
                log::info!("url = {}", url);
                let page = app.create_page(&url, global)?;
                pending::<!>().await;
                Ok(())
            }
        });
        log::info!("Running pool");
        pool.run().await?;
        log::info!("Done running pool");
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
        let create_cookie = warp::path("create_cookie")
            .and(warp::query::<HashMap<String, String>>())
            .map({
                let this = self.clone();
                move |q: HashMap<String, String>| {
                    let token: Uuid = q.get("token").unwrap().parse().unwrap();
                    let cookie = this.cookie_router.create_finish(token).unwrap();
                    let res = warp::reply::json(&());
                    let res = warp::reply::with_header(res, "set-cookie", format!("{}", cookie));
                    res
                }
            });
        let update_cookie = warp::path("update_cookie")
            .and(warp::query::<HashMap<String, String>>())
            .and(warp::header("Cookie"))
            .map({
                let this = self.clone();
                move |q: HashMap<String, String>, cookie: String| {
                    let cookies = Cookie::split_parse(&cookie)
                        .map_ok(|x| (x.name().to_string(), Arc::new(x.value().to_string())))
                        .collect::<Result<HashMap<_, _>, _>>()
                        .unwrap();
                    let token: Uuid = q.get("token").unwrap().parse().unwrap();
                    this.cookie_router.update_finish(token, cookies);
                    let res = warp::reply::json(&());
                    res
                }
            });
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
                        if let Err(e) = this.run_socket(tx, rx).await {
                            log::error!("Error handling websocket: {:?}", e);
                        }
                    })
                }
            });
        let routes = statik
            .or(site)
            .or(create_cookie)
            .or(update_cookie)
            .or(socket);
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

pub trait Application: 'static + Sync + Send {
    fn create_page(&self, url: &str, global: Arc<Global>) -> anyhow::Result<Page>;
}

pub struct Page {
    global: Rc<Global>,
    node: RcNode,
}

impl Page {
    pub fn new(global: Rc<Global>, node: RcNode) -> Page {
        global.window().document().body().append_child(node.clone());
        Page { global, node }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        self.global
            .window()
            .document()
            .body()
            .remove_child(self.node.clone());
    }
}
