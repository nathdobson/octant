use std::sync::{Arc, Weak};

use pretty_assertions::assert_eq;
use serde::{
    de::{DeserializeSeed},
    Deserializer,
    ser::SerializeStruct, Serializer,
};
use serde_json::{de::SliceRead, ser::PrettyFormatter};

use crate::{
    de::{DeserializeForest, DeserializeSnapshotSeed, DeserializeUpdate, DeserializeUpdateSeed},
    field::Field,
    forest::{Forest},
    prim::Prim,
    ser::{SerializeForest, SerializeUpdate, SerializeUpdateAdapter},
    tree::{Tree, TreeId},
    util::{
        deserializer_proxy::DeserializerProxy
        ,
        serializer_proxy::SerializerProxy,
        struct_visitor::{StructAccess, StructSeed, StructVisitor},
    },
};
use crate::util::option_seed::OptionSeed;

const EXPECTED: &str = r#"{
  "id": 0,
  "value": {
    "this": 0,
    "field1": 1,
    "field2": null,
    "field3": {
      "id": 1,
      "value": 2
    }
  }
}"#;

const EXPECTED_STATE: &str = r#"Tree {
    id: $0,
    state: MyStruct {
        this: (Weak),
        field1: 1,
        field2: (Weak),
        field3: Tree {
            id: $1,
            state: 2,
        },
    },
}"#;

fn serializer(buf: &mut Vec<u8>) -> serde_json::Serializer<&mut Vec<u8>, PrettyFormatter> {
    serde_json::Serializer::with_formatter(buf, PrettyFormatter::new())
}

fn deserializer(buf: &str) -> serde_json::Deserializer<SliceRead> {
    serde_json::Deserializer::new(serde_json::de::SliceRead::new(buf.as_bytes()))
}

struct JsonProxy;

impl DeserializerProxy for JsonProxy {
    type Error = serde_json::Error;
    type DeserializerValue<'up, 'de: 'up> = &'up mut serde_json::Deserializer<SliceRead<'de>>;
}

impl SerializerProxy for JsonProxy {
    type Error = serde_json::Error;
    type SerializerValue<'up> =
        &'up mut serde_json::Serializer<&'up mut Vec<u8>, PrettyFormatter<'up>>;
}

#[derive(Debug)]
struct MyStruct {
    this: Field<Weak<Tree<MyStruct>>>,
    field1: Field<Prim<u8>>,
    field2: Field<Weak<Tree<Prim<u8>>>>,
    field3: Field<Arc<Tree<Prim<u8>>>>,
}

impl SerializeUpdate for MyStruct {
    fn begin_stream(&mut self) {
        self.this.begin_stream();
        self.field1.begin_stream();
        self.field2.begin_stream();
        self.field3.begin_stream();
    }

    fn begin_update(&mut self) -> bool {
        self.this.begin_update()
            || self.field1.begin_update()
            || self.field2.begin_update()
            || self.field3.begin_update()
    }

    fn serialize_update<S: Serializer, SP: SerializerProxy>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        let mut s = s.serialize_struct("MyStruct", 4)?;
        s.serialize_field(
            "this",
            &self
                .this
                .modified()
                .then_some(SerializeUpdateAdapter::new(&self.this, forest, ser_forest)),
        )?;
        s.serialize_field(
            "field1",
            &self
                .field1
                .modified()
                .then_some(SerializeUpdateAdapter::new(&self.field1, forest, ser_forest)),
        )?;
        s.serialize_field(
            "field2",
            &self
                .field2
                .modified()
                .then_some(SerializeUpdateAdapter::new(&self.field2, forest, ser_forest)),
        )?;
        s.serialize_field(
            "field3",
            &self
                .field3
                .modified()
                .then_some(SerializeUpdateAdapter::new(&self.field3, forest, ser_forest)),
        )?;
        Ok(s.end()?)
    }

    fn end_update(&mut self) {
        self.this.end_update();
        self.field1.end_update();
        self.field2.end_update();
        self.field3.end_update();
    }
}

impl<'de> DeserializeUpdate<'de> for MyStruct {
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'a, DP: DeserializerProxy> {
            forest: &'a mut DeserializeForest<DP>,
        }
        impl<'a, 'de, DP: DeserializerProxy> StructVisitor<'de> for V<'a, DP> {
            type Value = MyStruct;

            fn visit<A: StructAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                Ok(MyStruct {
                    this: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
                    field1: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
                    field2: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
                    field3: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
                })
            }
        }
        StructSeed::new(
            "MyStruct",
            &["this", "field1", "field2", "field3"],
            V { forest },
        )
        .deserialize(d)
    }

    fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error> {
        struct V<'a, DP: DeserializerProxy> {
            forest: &'a mut DeserializeForest<DP>,
            this: &'a mut MyStruct,
        }
        impl<'a, 'de, DP: DeserializerProxy> StructVisitor<'de> for V<'a, DP> {
            type Value = ();

            fn visit<A: StructAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(&mut self.this.this, self.forest)))?;
                a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(
                    &mut self.this.field1,
                    self.forest,
                )))?;
                a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(
                    &mut self.this.field2,
                    self.forest,
                )))?;
                a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(
                    &mut self.this.field3,
                    self.forest,
                )))?;
                Ok(())
            }
        }
        StructSeed::new(
            "MyStruct",
            &["this", "field1", "field2", "field3"],
            V { forest, this: self },
        )
        .deserialize(d)
    }
}

#[test]
fn test_ser() {
    let mut root = Tree::new_cyclic(|this| MyStruct {
        this: Field::new(this.clone()),
        field1: Field::new(Prim::new(1)),
        field2: Field::new(Weak::new()),
        field3: Field::new(Tree::new(Prim::new(2))),
    });
    let mut forest = Forest::new();
    let mut ser_forest = SerializeForest::<JsonProxy>::new();
    let mut data = vec![];
    ser_forest
        .serialize_snapshot(&mut root, &mut forest, &mut serializer(&mut data))
        .unwrap();
    let data = String::from_utf8(data).unwrap();
    println!("{}", data);
    assert_eq!(EXPECTED, data);
}

#[test]
fn test_de() {
    let mut de = DeserializeForest::<JsonProxy>::new();
    let root = de
        .deserialize_snapshot::<Arc<Tree<MyStruct>>>(&mut deserializer(EXPECTED))
        .unwrap();
    let state = format!("{:#?}", root);
    println!("{}", state);
    assert_eq!(state, EXPECTED_STATE);
}

#[test]
fn test_basic() {
    let root = Tree::new_cyclic(|this| MyStruct {
        this: Field::new(this.clone()),
        field1: Field::new(Prim::new(1)),
        field2: Field::new(Weak::new()),
        field3: Field::new(Tree::new(Prim::new(2))),
    });
    let forest = Forest::new();
    let mut tester = Tester::new(
        forest,
        root.clone(),
        r#"{
  "id": 0,
  "value": {
    "this": 0,
    "field1": 1,
    "field2": null,
    "field3": {
      "id": 1,
      "value": 2
    }
  }
}"#,
    );
    tester.retest(&[]);
    **tester.forest.write(&root).field1 = 2;
    tester.retest(&[(
        0,
        r#"{
  "this": null,
  "field1": 2,
  "field2": null,
  "field3": null
}"#,
    )]);
    let weakish = Tree::new(Prim::new(2u8));
    *tester.forest.write(&root).field2 = Arc::downgrade(&weakish);
    *tester.forest.write(&root).field3 = weakish.clone();
    tester.retest(&[(
        0,
        r#"{
  "this": null,
  "field1": null,
  "field2": 2,
  "field3": {
    "id": 2,
    "value": 2
  }
}"#,
    )]);
}

fn serialize_update(
    forest: &mut Forest,
    ser_forest: &mut SerializeForest<JsonProxy>,
) -> Vec<(TreeId, String)> {
    forest
        .take_queue()
        .into_iter()
        .map(|id| {
            let mut bytes = vec![];
            ser_forest
                .serialize_update(id, forest, &mut serializer(&mut bytes))
                .unwrap();
            (id, String::from_utf8(bytes).unwrap())
        })
        .collect()
}

struct Tester {
    forest: Forest,
    ser_forest: SerializeForest<JsonProxy>,
    de_forest: DeserializeForest<JsonProxy>,
    input: Arc<Tree<MyStruct>>,
    output: Arc<Tree<MyStruct>>,
}

impl Tester {
    pub fn new(mut forest: Forest, mut input: Arc<Tree<MyStruct>>, expected: &str) -> Tester {
        let mut ser_forest = SerializeForest::new();
        let mut bytes = vec![];
        ser_forest
            .serialize_snapshot(&mut input, &mut forest, &mut serializer(&mut bytes))
            .unwrap();
        let snapshot = String::from_utf8(bytes).unwrap();
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
            ser_forest,
        }
    }
    pub fn retest(&mut self, expected: &[(u64, &str)]) {
        let update = serialize_update(&mut self.forest, &mut self.ser_forest);
        assert_eq!(update.len(), expected.len());
        for ((id1, s1), (id2, s2)) in update.iter().zip(expected) {
            assert_eq!(id1.value(), *id2);
            println!("{}", s1);
            assert_eq!(s1, s2);
        }
        for (id, s) in update {
            self.de_forest
                .deserialize_update(id, &mut deserializer(&s))
                .unwrap();
        }
        assert_eq!(format!("{:?}", self.input), format!("{:?}", self.output));
    }
}
