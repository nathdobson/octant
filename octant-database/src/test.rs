use serde_json::de::SliceRead;
use std::sync::Arc;

use serde_json::ser::PrettyFormatter;

use crate::{de::DeserializeForest, forest::Forest, tree::Tree};
use crate::util::arc_or_weak::ArcOrWeak;

const EXPECTED: &str = r#"{
  "id": 0,
  "value": {
    "x": {
      "Weak": 0
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
        "x": Weak(
            $0,
        ),
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
    let forest = Forest::with_root(root.clone());
    {
        let table = forest.read();
        let mut root_lock = table.write(&root);
        root_lock.insert("x".to_string(), ArcOrWeak::Weak(Arc::downgrade(&root)));
        root_lock.insert("y".to_string(), ArcOrWeak::Arc(Tree::new()));
    }
    let mut data = vec![];
    let () = forest
        .write()
        .serialize_update(&mut serializer(&mut data))
        .unwrap();
    let data = String::from_utf8(data).unwrap();
    println!("{}", data);
    assert_eq!(EXPECTED, data);
}

#[test]
fn test_de() {
    let mut de = DeserializeForest::new();
    let root = de
        .deserialize_snapshot(&mut deserializer(EXPECTED))
        .unwrap();
    assert_eq!(format!("{:#?}", root), EXPECTED_STATE);
}
