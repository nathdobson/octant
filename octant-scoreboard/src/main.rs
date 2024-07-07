#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]

use parking_lot::Mutex;
use std::sync::Arc;

use crate::score::ScoreApplication;
use octant_account::{AccountModule, SessionTable};
use octant_cookies::CookieRouter;
use octant_panic::register_panic_handler;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{OctantServer, OctantServerOptions};

mod navbar;
mod score;

#[tokio::main]
async fn main() -> OctantResult<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_panic_handler();
    let options = OctantServerOptions::from_command_line();
    let mut server = OctantServer::new(options).await?;
    let cookies = CookieRouter::new();
    cookies.register(&mut server);
    let sessions = SessionTable::new();
    AccountModule::new(server.database().clone(), cookies.clone(), sessions.clone())
        .await
        .register(&mut server);
    let app = Arc::new(ScoreApplication {
        cookies: cookies.clone(),
        sessions: sessions.clone(),
        guesses: Mutex::new(vec![]),
    });
    server.run(app).await?;
    Ok(())
}
