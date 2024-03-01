#![deny(unused_must_use)]
#![feature(trait_upcasting)]

mod app;

use octant_panic::register_handler;
use octant_server::{OctantServer, OctantServerOptions};
use crate::app::ScoreHandler;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_handler();
    let handler = ScoreHandler {};
    OctantServer {
        options: OctantServerOptions::from_command_line(),
        handler,
    }
    .run()
    .await;
}
