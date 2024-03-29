use serde_json::de::SliceRead;
use std::sync::Arc;

use serde_json::ser::PrettyFormatter;

use crate::{arc::ArcOrWeak, de::DeserializeForest, forest::Forest, tree::Tree};

const EXPECTED: &str = r#"{
  "id": 0,
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
}"#;

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

fn serializer(buf: &mut Vec<u8>) -> serde_json::Serializer<&mut Vec<u8>, PrettyFormatter> {
    serde_json::Serializer::with_formatter(buf, PrettyFormatter::new())
}

fn deserializer(buf: &str) -> serde_json::Deserializer<SliceRead> {
    serde_json::Deserializer::new(serde_json::de::SliceRead::new(buf.as_bytes()))
}

#[test]
fn test_ser() {
    let root = Tree::new();
    let table = Forest::new();
    {
        let table = table.read();
        table.enqueue_snapshot(root.clone());
        let mut root_lock = table.write(&root);
        root_lock.insert("x".to_string(), ArcOrWeak::Weak(Arc::downgrade(&root)));
        root_lock.insert("y".to_string(), ArcOrWeak::Arc(Tree::new()));
    }
    let mut data = vec![];
    let () = table
        .write()
        .serialize_update(&mut serializer(&mut data))
        .unwrap();
    let data = String::from_utf8(data).unwrap();
    println!("{}", data);
    assert_eq!(EXPECTED, data);
}

#[test]
fn test_de() {
    let forest = Forest::new();
    let mut de = DeserializeForest::new();
    let root = de
        .deserialize_snapshot(&*forest.read(), &mut deserializer(EXPECTED))
        .unwrap();
    assert_eq!(format!("{:#?}", root), EXPECTED_STATE);
    // de.deserialize_update(&*forest.read(), &mut deserializer(EXPECTED))
    //     .unwrap();
}
