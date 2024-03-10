use tmpdir::TmpDir;
use tokio::fs;

use crate::file::FileDatabase;
use crate::json::JsonFormat;
use crate::prim::Prim;

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let tmpdir = TmpDir::new("octant").await?;
    let path = tmpdir.as_ref().join("octant");
    {
        let mut database =
            FileDatabase::new(&path, JsonFormat, JsonFormat, || Prim("hi".to_string())).await?;
        assert_eq!(**database, "hi");
        *database.get_mut().get_prim() = "bye".to_string();
        assert_eq!(**database, "bye");
        database.write_update().await?;
    }
    println!("{:?}", fs::read_to_string(&path).await?);
    {,.
        let mut database =
            FileDatabase::<_, Prim<String>>::new(&path, JsonFormat, JsonFormat, || todo!()).await?;
        assert_eq!(**database, "bye");
    }
    Ok(())
}
