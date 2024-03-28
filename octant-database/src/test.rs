use std::{sync::Arc, thread, time::Duration};

use parking_lot::deadlock;
use serde_json::ser::PrettyFormatter;

use crate::{arc::ArcOrWeak, de::DeserializeForest, forest::Forest, tree::Tree};

const EXPECTED: &str = r#"[
  {
    "key": 0,
    "value": {
      "x": {
        "Weak": {
          "id": 0,
          "value": null
        }
      },
      "y": {
        "Arc": {
          "id": 1,
          "value": {}
        }
      }
    }
  }
]"#;

const EXPECTED_STATE: &str = r#"Tree {
    id: $0,
    state: {
        "x": (Weak),
        "y": Tree {
            id: $1,
            state: {},
        },
    },
}"#;

#[test]
fn test_ser() {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let deadlocks = deadlock::check_deadlock();
        if deadlocks.is_empty() {
            continue;
        }

        println!("{} deadlocks detected", deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            println!("Deadlock #{}", i);
            for t in threads {
                println!("Thread Id {:#?}", t.thread_id());
                println!("{:#?}", t.backtrace());
            }
        }
    });

    let table = Forest::new();
    let root;
    {
        let table = table.read();
        root = Tree::new();
        table.enqueue(&root);
        {
            let mut root_lock = table.write(&root);
            root_lock.insert("x".to_string(), ArcOrWeak::Weak(Arc::downgrade(&root)));
            root_lock.insert("y".to_string(), ArcOrWeak::Arc(Tree::new()));
        }
    }
    let mut data = vec![];
    let () = table
        .write()
        .serialize_log(&mut serde_json::Serializer::with_formatter(
            &mut data,
            PrettyFormatter::new(),
        ))
        .unwrap();
    let data = String::from_utf8(data).unwrap();
    println!("{}", data);
    assert_eq!(EXPECTED, data);
}

#[test]
fn test_de() {
    let forest = Forest::new();
    let mut read = forest.write();
    let root = Tree::new();
    let mut de = DeserializeForest::new(Arc::downgrade(&forest), root.clone());
    de.deserialize_log(
        &mut *read,
        &mut serde_json::Deserializer::new(serde_json::de::SliceRead::new(EXPECTED.as_bytes())),
    )
    .unwrap();
    assert_eq!(format!("{:#?}", root), EXPECTED_STATE);
}
