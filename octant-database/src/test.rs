use std::sync::Arc;
use std::thread;
use std::time::Duration;

use parking_lot::deadlock;
use serde_json::ser::PrettyFormatter;

use crate::arc::ArcOrWeak;
use crate::de::DeserializeTable;
use crate::RowTable;

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
    thread::spawn(move || {
        loop {
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
        }
    });

    let table = RowTable::new();
    let root;
    {
        let table = table.read();
        root = table.add();
        table.try_enqueue(&root);
        {
            let mut root_lock = table.write(&root);
            root_lock.insert("x".to_string(), ArcOrWeak::Weak(Arc::downgrade(&root)));
            root_lock.insert("y".to_string(), ArcOrWeak::Arc(table.add()));
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
    let table = RowTable::new();
    let mut read = table.write();
    let root = read.add();
    let mut de = DeserializeTable::new(&mut *read, root.clone());
    de.deserialize_log(
        &mut *read,
        &mut serde_json::Deserializer::new(serde_json::de::SliceRead::new(EXPECTED.as_bytes())),
    )
        .unwrap();
    panic!("{:#?}", root);
}
