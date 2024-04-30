#![feature(absolute_path)]
#![feature(arbitrary_self_types)]

use std::{
    path,
    path::Path,
    sync::{Arc, Weak},
};

use tokio::io;

use no_imports::MyStruct;
use octant_database::{file::Database, tree::Tree};
use octant_database::value::field::Field;
use octant_database::value::prim::Prim;

mod no_imports {
    #![no_implicit_prelude]
    ::octant_database::database_struct! {
        #[derive(Debug)]
        pub struct MyStruct {
            pub this: ::std::sync::Weak<::octant_database::tree::Tree<MyStruct>>,
            pub field1: ::octant_database::value::prim::Prim<u8>,
            pub field2: ::std::sync::Weak<::octant_database::tree::Tree<::octant_database::value::prim::Prim<u8>>>,
            pub field3: ::std::sync::Arc<::octant_database::tree::Tree<::octant_database::value::prim::Prim<u8>>>,
        }
    }
}

#[tokio::test]
async fn test_file() -> io::Result<()> {
    let path = path::absolute(Path::new("../target/test.db"))?;
    tokio::fs::remove_dir_all(&path).await.ok();
    tokio::fs::create_dir_all(&path).await?;
    {
        let root = Arc::new_cyclic(|this| {
            Tree::new(MyStruct {
                this: Field::new(this.clone()),
                field1: Field::new(Prim::new(1)),
                field2: Field::new(Weak::new()),
                field3: Field::new(Arc::new(Tree::new(Prim::new(2)))),
            })
        });
        let (mut db, root) = Database::new(&path, || root).await?;
        **db.forest()
            .write()
            .await
            .write(&root)
            .get_mut()
            .field1()
            .get_mut() = 2;
        db.serialize().await?;
    }
    {
        let (db, root) = Database::new::<MyStruct>(&path, || todo!()).await?;
        pretty_assertions::assert_eq!(**db.forest().read().await.read(&root).field1, 2);
    }
    Ok(())
}
