#![deny(unused_must_use)]

use std::net::SocketAddr;
use clap::Parser;
use warp::Filter;

#[derive(Parser, Debug)]
pub struct OctantServer {
    #[arg(long, required = true)]
    pub bind_http: Option<SocketAddr>,
}

impl OctantServer {
    pub fn from_command_line() -> OctantServer {
        OctantServer::parse()
    }
}

impl OctantServer {
    pub async fn run(self) {
        let routes = warp::path("static").and(warp::fs::dir("./target/www"));
        if let Some(bind_http) = self.bind_http {
            warp::serve(routes.clone()).run(bind_http).await;
        }
    }
}
