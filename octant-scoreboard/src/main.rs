#![deny(unused_must_use)]
#![feature(trait_upcasting)]

mod app;

use crate::app::ScoreBoardApplication;
use octant_panic::register_handler;
use octant_server::{OctantServer, OctantServerOptions};

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_handler();
    let application = ScoreBoardApplication {};
    OctantServer {
        options: OctantServerOptions::from_command_line(),
        application,
    }
    .run()
    .await;
}
