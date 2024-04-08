#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![allow(unused_variables)]
#![allow(dead_code)]

use octant_panic::register_handler;
use octant_server::{OctantServer, OctantServerOptions};

use crate::{login::LoginHandler, score::ScoreHandler};

mod login;
mod score;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_handler();
    let mut server = OctantServer::new(OctantServerOptions::from_command_line());
    server.add_handler(ScoreHandler {});
    server.add_handler(LoginHandler {});
    server.run().await?;
    Ok(())
}
