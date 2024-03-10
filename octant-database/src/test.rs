use std::fmt::Formatter;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use tmpdir::TmpDir;
use tokio::fs;

use crate::field::Field;
use crate::file::FileDatabase;
use crate::json::JsonFormat;
use crate::prim::Prim;
use crate::seed::{OptionSeed, UpdateSeed};
use crate::stream_deserialize::StreamDeserialize;
use crate::stream_serialize::StreamSerialize;
use crate::tack::Tack;

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let tmpdir = TmpDir::new("octant").await?;
    let path = tmpdir.as_ref().join("octant");
    {
        let mut database = FileDatabase::new(&path, JsonFormat, JsonFormat, || MyStruct {
            a: Field::new(Prim(1)),
            b: Field::new(Prim(2)),
        })
        .await?;
        assert_eq!(**database.a, 1);
        assert_eq!(**database.b, 2);
        *database.get_mut().a_mut() = 10;
        assert_eq!(**database.a, 10);
        assert_eq!(**database.b, 2);

        database.write_update().await?;
    }
    assert_eq!(
r#"{"a":1,"b":2}
{"a":10,"b":null}
"#, fs::read_to_string(&path).await?);
    {
        let mut database =
            FileDatabase::<_, MyStruct>::new(&path, JsonFormat, JsonFormat, || todo!()).await?;
        assert_eq!(**database.a, 10);
        assert_eq!(**database.b, 2);
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct MyStruct {
    a: Field<Prim<u8>>,
    b: Field<Prim<u8>>,
}

impl MyStruct {
    pub fn new(a: u8, b: u8) -> Self {
        MyStruct {
            a: Field::new(Prim(a)),
            b: Field::new(Prim(b)),
        }
    }
    pub fn a(&self) -> u8 {
        **self.a
    }
    pub fn a_mut<'a>(self: Tack<'a, Self>) -> &'a mut u8 {
        Tack::new(&mut self.into_inner_unchecked().a)
            .as_mut()
            .get_prim()
    }
    pub fn b_mut<'a>(self: Tack<'a, Self>) -> &'a mut u8 {
        Tack::new(&mut self.into_inner_unchecked().b)
            .as_mut()
            .get_prim()
    }
}

impl StreamSerialize for MyStruct {
    fn build_baseline(&mut self) {
        self.a.build_baseline();
        self.b.build_baseline();
    }

    fn build_target(&mut self) -> bool {
        self.a.build_target() || self.b.build_target()
    }

    fn serialize_update<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut serializer = serializer.serialize_struct("MyStruct", 2)?;
        serializer.serialize_field("a", &self.a.modified().then_some(&self.a))?;
        serializer.serialize_field("b", &self.b.modified().then_some(&self.b))?;
        Ok(serializer.end()?)
    }
}

impl<'de> StreamDeserialize<'de> for MyStruct {
    fn deserialize_stream<D: Deserializer<'de>>(&mut self, d: D) -> Result<(), D::Error> {
        struct V<'a>(&'a mut MyStruct);
        impl<'a, 'de> Visitor<'de> for V<'a> {
            type Value = ();

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "struct")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                todo!()
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<String>()? {
                    if key == "a" {
                        map.next_value_seed(OptionSeed::new(UpdateSeed::new(&mut self.0.a)))?;
                    } else if key == "b" {
                        map.next_value_seed(OptionSeed::new(UpdateSeed::new(&mut self.0.b)))?;
                    } else {
                        return Err(A::Error::custom(format_args!("unknown field {}", key)));
                    }
                }
                Ok(())
            }
        }
        d.deserialize_struct("MyStruct", &["a", "b"], V(self))
    }
}
