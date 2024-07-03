#![feature(arbitrary_self_types)]
#![deny(unused_must_use)]
use marshal::{Deserialize, Serialize};
use marshal_object::derive_variant;
use marshal_update::{DeserializeUpdate, SerializeStream, SerializeUpdate};
use octant_database::{
    file::DatabaseFile,
    table::{BoxTable,  Table},
};
use octant_error::OctantResult;
use std::{mem, path, path::Path};
use octant_database::database::Database;

#[tokio::test]
async fn test() -> OctantResult<()> {
    let path = path::absolute(Path::new("../target/test.db"))?;
    tokio::fs::remove_dir_all(&path).await.ok();
    tokio::fs::create_dir_all(&path).await?;
    let (mut db, root) = DatabaseFile::<(u8, u8)>::new(&path).await?;
    root.write().await.0 = 4;
    db.serialize().await?;
    root.write().await.1 = 8;
    db.serialize().await?;
    mem::drop((db, root));
    let (mut db, root) = DatabaseFile::<(u8, u8)>::new(&path).await?;
    assert_eq!(root.read().await.0, 4);
    assert_eq!(root.read().await.1, 8);
    root.write().await.0 = 15;
    db.serialize().await?;
    root.write().await.1 = 16;
    db.serialize().await?;
    mem::drop((db, root));
    Ok(())
}

#[tokio::test]
async fn test2() -> OctantResult<()> {
    #[derive(
        Serialize, Deserialize, SerializeUpdate, DeserializeUpdate, SerializeStream, Default,
    )]
    struct MyTable {
        x: u8,
        y: u16,
    }

    derive_variant!(BoxTable, MyTable);
    impl Table for MyTable {}

    let path = path::absolute(Path::new("../target/test2.db"))?;
    tokio::fs::remove_dir_all(&path).await.ok();
    tokio::fs::create_dir_all(&path).await?;
    let (mut db, root) = DatabaseFile::<Database>::new(&path).await?;
    assert_eq!(root.write().await.table::<MyTable>().x, 0);
    assert_eq!(root.write().await.table::<MyTable>().y, 0);
    root.write().await.table_mut::<MyTable>().x = 4;
    db.serialize().await?;
    root.write().await.table_mut::<MyTable>().y = 8;
    db.serialize().await?;
    mem::drop((db, root));
    let (mut db, root) = DatabaseFile::<Database>::new(&path).await?;
    assert_eq!(root.write().await.table::<MyTable>().x, 4);
    assert_eq!(root.write().await.table::<MyTable>().y, 8);
    root.write().await.table_mut::<MyTable>().x = 15;
    db.serialize().await?;
    root.write().await.table_mut::<MyTable>().x = 16;
    db.serialize().await?;
    mem::drop((db, root));
    Ok(())
}
