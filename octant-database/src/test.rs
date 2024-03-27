use crate::arc::ArcOrWeak;
use crate::de::DeserializeTable;
use serde_json::ser::PrettyFormatter;
use std::sync::Arc;
use crate::{Row, RowTable};

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
#[test]
fn test_ser() {
    let root = Row::new();
    let table = RowTable::new(root.clone());
    {
        let table = table.read();
        {
            let mut root_lock = table.write(&root);
            root_lock.insert("x".to_string(), ArcOrWeak::Weak(Arc::downgrade(&root)));
            root_lock.insert("y".to_string(), ArcOrWeak::Arc(Row::new()));
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
    let root = Row::new();
    let table = RowTable::new(root.clone());
    let mut read = table.write();
    let mut de = DeserializeTable::new(&mut *read, root.clone());
    de.deserialize_log(
        &mut *read,
        &mut serde_json::Deserializer::new(serde_json::de::SliceRead::new(EXPECTED.as_bytes())),
    )
    .unwrap();
    panic!("{:#?}", root);
}
