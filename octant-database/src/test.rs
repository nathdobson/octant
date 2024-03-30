use serde_json::de::SliceRead;
use std::sync::Arc;

use serde_json::ser::PrettyFormatter;

use crate::{de::DeserializeForest, forest::Forest, tree::Tree, util::arc_or_weak::ArcOrWeak};
use pretty_assertions::assert_eq;

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

#[test]
fn test_basic() {
    let root = Tree::new();
    let forest = Forest::with_root(root.clone());
    forest
        .read()
        .write(&root)
        .insert("x".to_string(), ArcOrWeak::Weak(Arc::downgrade(&root)));
    let mut tester = Tester::new(
        forest.clone(),
        root.clone(),
        r#"{
  "id": 0,
  "value": {
    "x": {
      "Weak": 0
    }
  }
}"#,
    );
    tester.retest("{}");
    forest
        .read()
        .write(&root)
        .insert("y".to_string(), ArcOrWeak::Arc(Tree::new()));
    tester.retest(
        r#"{
  "0": {
    "y": {
      "Arc": {
        "id": 1,
        "value": {}
      }
    }
  }
}"#,
    );
    let weakish = Tree::new();
    forest
        .read()
        .write(&root)
        .insert("a".to_string(), ArcOrWeak::Weak(Arc::downgrade(&weakish)));
    forest
        .read()
        .write(&root)
        .insert("b".to_string(), ArcOrWeak::Arc(weakish.clone()));
    tester.retest(
        r#"{
  "0": {
    "a": {
      "Weak": 2
    },
    "b": {
      "Arc": {
        "id": 2,
        "value": {}
      }
    }
  }
}"#,
    );
}

fn serialize_update(forest: &Arc<Forest>) -> String {
    let mut bytes = vec![];
    forest
        .write()
        .serialize_update(&mut serializer(&mut bytes))
        .unwrap();
    String::from_utf8(bytes).unwrap()
}

struct Tester {
    forest: Arc<Forest>,
    de_forest: DeserializeForest,
    input: Arc<Tree>,
    output: Arc<Tree>,
}

impl Tester {
    pub fn new(forest: Arc<Forest>, input: Arc<Tree>, expected: &str) -> Tester {
        let snapshot = serialize_update(&forest);
        assert_eq!(snapshot, expected);
        let mut de_forest = DeserializeForest::new();
        let output = de_forest
            .deserialize_snapshot(&mut deserializer(&snapshot))
            .unwrap();
        assert_eq!(format!("{:?}", input), format!("{:?}", output));
        Tester {
            forest,
            input,
            output,
            de_forest,
        }
    }
    pub fn retest(&mut self, expected: &str) {
        let update = serialize_update(&self.forest);
        assert_eq!(update, expected);
        self.de_forest
            .deserialize_update(&mut deserializer(&update))
            .unwrap();
        assert_eq!(format!("{:?}", self.input), format!("{:?}", self.output));
    }
}
