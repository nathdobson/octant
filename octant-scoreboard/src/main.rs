#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]

use std::sync::Arc;

use parking_lot::Mutex;

use octant_account::{ SessionTable};
use octant_cookies::CookieRouter;
use octant_panic::register_panic_handler;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{OctantServer, OctantServerOptions};
use crate::app::ScoreApplication;

// mod navbar;
mod app;
mod puzzle;

#[tokio::main]
async fn main() -> OctantResult<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_panic_handler();
    let options = OctantServerOptions::from_command_line();
    let mut server = OctantServer::new(options).await?;
    let cookies = CookieRouter::new();
    cookies.register(&mut server);
    let sessions = SessionTable::new();
    let app = Arc::new(ScoreApplication {
        db: server.database().clone(),
        cookies: cookies.clone(),
        sessions: sessions.clone(),
        guesses: Mutex::new(vec![]),
    });
    server.run(app).await?;
    Ok(())
}
