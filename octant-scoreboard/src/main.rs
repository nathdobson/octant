#![deny(unused_must_use)]
#![feature(trait_upcasting)]

mod app;

use std::sync::Arc;

use octant_gui::event_loop::{Page, Session};
use octant_gui::Global;
use octant_panic::register_handler;
use octant_server::{Application, OctantServer, OctantServerOptions};
use crate::app::ScoreBoardApplication;

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
