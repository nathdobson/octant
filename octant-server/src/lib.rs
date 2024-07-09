#![feature(future_join)]
#![deny(unused_must_use)]
#![allow(unused_variables)]
#![feature(trait_upcasting)]
#![feature(never_type)]

use std::{
    future::pending, net::SocketAddr, path::Path, rc::Rc, sync::Arc, thread::available_parallelism,
    time::Duration,
};

use clap::Parser;
use futures::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use marshal::context::OwnedContext;
use marshal_json::{decode::full::JsonDecoderBuilder, encode::full::JsonEncoderBuilder};
use marshal_pointer::Rcf;
use octant_components::PathComponentBuilder;
use octant_database::{
    database::{ArcDatabase, Database},
    file::DatabaseFile,
};
use octant_error::{octant_error, Context, OctantError, OctantResult};
use octant_executor::{
    event_loop::EventPool,
    local_set::{LocalSetPool, LocalSetSpawn},
};
use octant_runtime_server::{
    proto::{DownMessageList, UpMessageList},
    runtime::Runtime,
};
use octant_web_sys_server::{global::Global};
use parking_lot::Mutex;
use tokio::{sync::mpsc, try_join};
use url::Url;
use warp::{
    filters::BoxedFilter,
    ws::{Message, WebSocket},
    Filter, Rejection, Reply,
};

use crate::{
    session::{Session, UrlPrefix},
    sink::BufferedDownMessageSink,
};

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

pub trait OctantApplication: Sync + Send {
    fn create_path_component_builder(
        self: Arc<Self>,
        session: Rc<Session>,
    ) -> OctantResult<Rcf<dyn PathComponentBuilder>>;
}

pub struct OctantServer {
    options: OctantServerOptions,
    database: ArcDatabase,
    warp_handlers: Mutex<Vec<WarpHandler>>,
    spawn: Arc<LocalSetSpawn>,
}

impl OctantServerOptions {
    pub fn from_command_line() -> Self {
        Self::parse()
    }
}

pub type WarpHandler = BoxedFilter<(Box<dyn Reply>,)>;

pub trait IntoWarpHandler {
    fn into_warp_handler(self) -> WarpHandler;
}

impl<T: 'static + Sync + Send + Filter<Extract = (R,), Error = Rejection>, R: 'static>
    IntoWarpHandler for T
where
    R: Reply,
{
    fn into_warp_handler(self) -> WarpHandler {
        self.map(|x| Box::new(x) as Box<dyn Reply>).boxed()
    }
}

impl OctantServer {
    pub async fn new(options: OctantServerOptions) -> OctantResult<Self> {
        let (spawn, pool) = LocalSetPool::new(available_parallelism().unwrap().get());
        pool.detach();
        let (db_writer, db) = DatabaseFile::<Database>::new(Path::new(&options.db_path))
            .await
            .context("Opening database")?;
        tokio::spawn(db_writer.serialize_every(Duration::from_secs(1)));
        Ok(OctantServer {
            options,
            database: db,
            // handlers: HashMap::new(),
            warp_handlers: Mutex::new(vec![]),
            spawn,
        })
    }
    pub fn database(&self) -> &ArcDatabase {
        &self.database
    }
    // pub fn add_handler(&mut self, handler: impl Handler) {
    //     self.handlers.insert(handler.prefix(), Arc::new(handler));
    // }
    pub fn add_warp_handler(&mut self, handler: WarpHandler) {
        self.warp_handlers.get_mut().push(handler);
    }
    async fn encode(list: DownMessageList) -> OctantResult<Message> {
        let mut ctx = OwnedContext::new();
        Ok(Message::text(
            JsonEncoderBuilder::new().serialize(&list, ctx.borrow())?,
        ))
    }
    fn decode(runtime: &Rc<Runtime>, x: Message) -> OctantResult<Option<UpMessageList>> {
        if x.is_close() {
            Ok(None)
        } else if x.is_text() {
            let mut ctx = OwnedContext::new();
            let output =
                JsonDecoderBuilder::new(x.as_bytes()).deserialize::<UpMessageList>(ctx.borrow())?;
            Ok(Some(output))
        } else if x.is_binary() {
            todo!();
        } else {
            Ok(None)
        }
    }
    pub async fn run_socket(
        self: Arc<Self>,
        app: Arc<dyn OctantApplication>,
        tx: SplitSink<WebSocket, Message>,
        rx: SplitStream<WebSocket>,
    ) -> OctantResult<()> {
        let spawn = self.spawn.clone();
        spawn
            .spawn_async(move || async move {
                self.run_socket_local(app, tx, rx).await?;
                Ok(())
            })
            .await?
    }
    pub async fn run_socket_local(
        self: Arc<Self>,
        app: Arc<dyn OctantApplication>,
        tx: SplitSink<WebSocket, Message>,
        mut rx: SplitStream<WebSocket>,
    ) -> OctantResult<()> {
        let (tx_inner, rx_inner) = mpsc::unbounded_channel();
        let mut sink = BufferedDownMessageSink::new(rx_inner, Box::pin(tx.with(Self::encode)));
        let (spawn, mut pool) = EventPool::new(move |cx| sink.poll_flush(cx));
        let runtime = Rc::new(Runtime::new(tx_inner, spawn.clone()));
        let global = Global::new(runtime);
        let session = Rc::new(Session::new(global.clone()));
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
                let url = global.window().document().location().href().await?;
                let url = Url::parse(&url)?;
                session.insert_data(UrlPrefix::new(url.join("/")?));
                log::info!("url = {}", url);
                let component_builder = app.create_path_component_builder(session)?;
                let component = component_builder.build("/site")?;
                global
                    .window()
                    .document()
                    .body()
                    .append_child(component.node().strong());
                component.update_path(&url)?;
                global.window().history().set_push_state_handler(Box::new({
                    let component = Rcf::downgrade(&component);
                    move |url| {
                        if let Some(component) = component.upgrade() {
                            component.update_path(&Url::parse(&url)?)?;
                        }
                        Ok(())
                    }
                }));
                global.window().set_pop_state_handler({
                    let component = Rcf::downgrade(&component);
                    Box::new(move |url| {
                        if let Some(component) = component.upgrade() {
                            component.update_path(&Url::parse(&url)?)?;
                        }
                        Ok(())
                    })
                });
                pending::<!>().await;
                Ok(())
            }
        });
        log::info!("Running pool");
        pool.run().await?;
        log::info!("Done running pool");
        Ok(())
    }
    pub async fn run(self, app: Arc<dyn OctantApplication>) -> OctantResult<()> {
        Arc::new(self).run_arc(app).await?;
        Ok(())
    }
    fn add_header(reply: impl Reply) -> impl Reply {
        warp::reply::with_header(reply, "Cache-Control", "no-cache")
    }
    fn statik() -> BoxedFilter<(Box<dyn Reply>,)> {
        warp::path("static")
            .and(warp::fs::dir("./target/www"))
            .map(Self::add_header)
            .map(|x| Box::new(x) as Box<dyn Reply>)
            .boxed()
    }
    pub async fn run_arc(self: Arc<Self>, app: Arc<dyn OctantApplication>) -> OctantResult<()> {
        let statik = Self::statik();
        let site = warp::path("site")
            .and(warp::fs::file("./target/www/index.html"))
            .map(Self::add_header);
        let socket = warp::path("socket")
            .and(warp::path::param())
            .and(warp::ws())
            .map({
                let this = self.clone();
                let app = app.clone();
                move |name: String, ws: warp::ws::Ws| {
                    log::info!("Handling");
                    let this = this.clone();
                    let app = app.clone();
                    ws.on_upgrade(|websocket| async move {
                        log::info!("Upgraded");
                        let (tx, rx) = websocket.split();
                        if let Err(e) = this.run_socket(app, tx, rx).await {
                            log::error!("Error handling websocket: {:?}", e);
                        }
                    })
                }
            });
        let mut routes: WarpHandler = statik.or(site).or(socket).into_warp_handler();
        for x in self.warp_handlers.lock().drain(..) {
            routes = routes.or(x).into_warp_handler();
        }
        let http = async {
            if let Some(bind_http) = self.options.bind_http {
                warp::serve(routes.clone()).run(bind_http).await;
            }
            Result::<_, OctantError>::Ok(())
        };
        let https = async {
            if let Some(bind_https) = self.options.bind_https {
                warp::serve(routes.clone())
                    .tls()
                    .cert_path(
                        self.options
                            .cert_path
                            .as_ref()
                            .ok_or_else(|| octant_error!("missing cert_path flag"))?,
                    )
                    .key_path(
                        &self
                            .options
                            .key_path
                            .as_ref()
                            .ok_or_else(|| octant_error!("missing key_path flag:"))?,
                    )
                    .run(bind_https)
                    .await;
            }
            Result::<_, OctantError>::Ok(())
        };
        try_join!(http, https)?;
        Ok(())
    }
}
