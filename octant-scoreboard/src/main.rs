#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]

use std::{path::Path, sync::Arc, time::Duration};

use parking_lot::Mutex;

use octant_account::{
    AccountDatabase, login::LoginHandler, register::RegisterHandler, SessionTable,
};
use octant_database::{database_struct, file::Database, tree::Tree};
use octant_error::Context;
use octant_panic::register_handler;
use octant_runtime_server::reexports::octant_error::OctantResult;
use octant_server::{OctantServer, OctantServerOptions};

use crate::score::ScoreHandler;

mod score;

database_struct! {
    #[derive(Default)]
    struct ScoreboardDatabase{
        accounts: Arc<Tree<AccountDatabase>>,
    }
}

#[tokio::main]
async fn main() -> OctantResult<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_handler();
    let options = OctantServerOptions::from_command_line();
    let (db_writer, db) = Database::new(
        Path::new(&options.db_path),
        <Arc<Tree<ScoreboardDatabase>>>::default,
    )
    .await
    .context("Opening database")?;
    let forest = db_writer.forest().clone();
    tokio::spawn(db_writer.serialize_every(Duration::from_secs(1)));
    let mut server = OctantServer::new(options);
    {
        let forest_read = forest.read().await;
        let accounts = forest_read.read(&db).accounts.clone();
        let session_table = SessionTable::new();
        server.add_handler(ScoreHandler {
            cookie_router: server.cookie_router().clone(),
            session_table: session_table.clone(),
            guesses: Mutex::new(vec![]),
        });
        server.add_handler(RegisterHandler {
            forest: forest.clone(),
            accounts: accounts.clone(),
        });
        server.add_handler(LoginHandler {
            forest: forest.clone(),
            accounts: accounts.clone(),
            cookie_router: server.cookie_router().clone(),
            session_table,
        });
    }
    server.run().await?;
    Ok(())
}
