#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]

use std::{path::Path, time::Duration};

use parking_lot::Mutex;

use octant_account::{login::LoginHandler, register::RegisterHandler, AccountTable, SessionTable};
use octant_database::{file::DatabaseFile, table::Database};
use octant_error::Context;
use octant_panic::register_handler;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{OctantServer, OctantServerOptions};

use crate::score::ScoreHandler;

mod score;

#[tokio::main]
async fn main() -> OctantResult<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_handler();
    let options = OctantServerOptions::from_command_line();
    let (db_writer, db) = DatabaseFile::<Database>::new(Path::new(&options.db_path))
        .await
        .context("Opening database")?;
    tokio::spawn(db_writer.serialize_every(Duration::from_secs(1)));
    let mut server = OctantServer::new(options);
    {
        let mut db = db.write().await;
        let accounts = db.table::<AccountTable>();
    }
    let session_table = SessionTable::new();
    server.add_handler(ScoreHandler {
        cookie_router: server.cookie_router().clone(),
        session_table: session_table.clone(),
        guesses: Mutex::new(vec![]),
    });
    server.add_handler(RegisterHandler { db: db.clone() });
    server.add_handler(LoginHandler {
        db: db.clone(),
        cookie_router: server.cookie_router().clone(),
        session_table,
    });
    server.run().await?;
    Ok(())
}
