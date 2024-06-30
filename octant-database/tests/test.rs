#![deny(unused_must_use)]
use octant_database::file::DatabaseFile;
use octant_error::OctantResult;
use std::{mem, path, path::Path};

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
    db.terminate().await?;
    mem::drop((db, root));
    let (mut db, root) = DatabaseFile::<(u8, u8)>::new(&path).await?;
    assert_eq!(root.read().await.0, 4);
    assert_eq!(root.read().await.1, 8);
    root.write().await.0 = 15;
    db.serialize().await?;
    root.write().await.1 = 16;
    db.serialize().await?;
    db.terminate().await?;
    mem::drop((db, root));
    Ok(())
}
