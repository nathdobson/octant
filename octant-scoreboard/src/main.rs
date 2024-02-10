#![deny(unused_must_use)]

use octant_server::OctantServer;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    OctantServer::from_command_line().run().await;
}
