#![deny(unused_must_use)]
#![feature(trait_upcasting)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(arbitrary_self_types)]

use std::{path::Path, sync::Arc, time::Duration};

use anyhow::Context;

use octant_account::{AccountDatabase, login::LoginHandler, register::RegisterHandler};
use octant_database::{database_struct, file::Database, tree::Tree};
use octant_panic::register_handler;
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
async fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    register_handler();
    let options = OctantServerOptions::from_command_line();
    let (db_writer, db) = Database::new(Path::new(&options.db_path), <Arc<Tree<ScoreboardDatabase>>>::default)
        .await
        .context("Opening database")?;
    let forest = db_writer.forest().clone();
    tokio::spawn(db_writer.serialize_every(Duration::from_secs(1)));
    let mut server = OctantServer::new(options);
    {
        let forest = forest.read().await;
        server.add_handler(ScoreHandler {});
        server.add_handler(RegisterHandler {});
        server.add_handler(LoginHandler {
            database: forest.read(&db).accounts.clone(),
        });
    }
    server.run().await?;
    Ok(())
}
