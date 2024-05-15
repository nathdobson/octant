#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]
#![allow(unused_variables)]
#![deny(unused_must_use)]

use catalog::{register, Builder, BuilderFrom, Registry};
use serde::{
    de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor},
    ser::{Error, SerializeStruct},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

pub type OctantDeserializer<'a, 'de> =
    &'a mut serde_json::Deserializer<serde_json::de::SliceRead<'de>>;
pub type OctantSerializer<'a> = &'a mut serde_json::Serializer<&'a mut Vec<u8>>;

pub fn serialize<T: ?Sized + Serialize>(x: &T) -> Result<Vec<u8>, serde_json::Error> {
    let mut vec = vec![];
    let mut serializer = serde_json::Serializer::new(&mut vec);
    x.serialize(&mut serializer)?;
    Ok(vec)
}

pub fn deserialize<'de, T: Deserialize<'de>>(de: &'de [u8]) -> Result<T, serde_json::Error> {
    T::deserialize(&mut serde_json::Deserializer::new(
        serde_json::de::SliceRead::new(de),
    ))
}

type DeserializeFn = for<'a, 'de> fn(
    OctantDeserializer<'a, 'de>,
) -> Result<
    Box<dyn MyTrait>,
    <OctantDeserializer<'a, 'de> as Deserializer<'de>>::Error,
>;
struct DeserializeImp<T> {
    name: &'static str,
    deserialize: DeserializeFn,
    phantom: PhantomData<T>,
}

impl<T: 'static + MyTrait + for<'de> Deserialize<'de>> DeserializeImp<T> {
    pub fn new(name: &'static str) -> Self {
        DeserializeImp {
            name,
            deserialize: |de| Ok(Box::new(T::deserialize(de)?)),
            phantom: PhantomData,
        }
    }
}

struct DeserializeRegistry {
    deserializers: HashMap<String, DeserializeFn>,
}

static DESERIALIZE_REGISTRY: Registry<DeserializeRegistry> = Registry::new();

impl Builder for DeserializeRegistry {
    type Output = DeserializeRegistry;

    fn new() -> Self {
        DeserializeRegistry {
            deserializers: HashMap::new(),
        }
    }

    fn build(self) -> Self::Output {
        self
    }
}

impl<T> BuilderFrom<&'static DeserializeImp<T>> for DeserializeRegistry {
    fn insert(&mut self, element: &'static DeserializeImp<T>) {
        self.deserializers
            .insert(element.name.to_string(), element.deserialize);
    }
}

pub trait SerializeType {
    fn serialize_type(&self) -> &'static str;
}

pub trait SerializeDyn: SerializeType {
    fn serialize_dyn(
        &self,
        s: OctantSerializer,
    ) -> Result<<OctantSerializer as Serializer>::Ok, <OctantSerializer as Serializer>::Error>;
}

impl<T: Serialize + SerializeType> SerializeDyn for T {
    fn serialize_dyn(
        &self,
        s: OctantSerializer,
    ) -> Result<<OctantSerializer as Serializer>::Ok, <OctantSerializer as Serializer>::Error> {
        self.serialize(s)
    }
}

struct DeserializeValue(String);

trait DeserializeSpec<'de, D: Deserializer<'de>> {
    fn deserialize_spec(self, d: D) -> Result<Box<dyn MyTrait>, D::Error>;
}

impl<'de, D: Deserializer<'de>> DeserializeSpec<'de, D> for DeserializeValue {
    default fn deserialize_spec(self, d: D) -> Result<Box<dyn MyTrait>, D::Error> {
        Err(D::Error::custom("missing specialization"))
    }
}

impl<'a, 'de> DeserializeSpec<'de, OctantDeserializer<'a, 'de>> for DeserializeValue {
    fn deserialize_spec(
        self,
        d: OctantDeserializer<'a, 'de>,
    ) -> Result<Box<dyn MyTrait>, <OctantDeserializer<'a, 'de> as Deserializer<'de>>::Error> {
        let imp = DESERIALIZE_REGISTRY
            .deserializers
            .get(&self.0)
            .ok_or_else(|| {
                <<OctantDeserializer<'a, 'de> as Deserializer<'de>>::Error as serde::de::Error>::custom(
                    "Missing deserializer",
                )
            })?;
        imp(d)
    }
}

impl<'de> DeserializeSeed<'de> for DeserializeValue {
    type Value = Box<dyn MyTrait>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Self as DeserializeSpec<D>>::deserialize_spec(self, deserializer)
    }
}

impl<'de> Deserialize<'de> for Box<dyn MyTrait> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V {}
        impl<'de> Visitor<'de> for V {
            type Value = Box<dyn MyTrait>;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "MyTrait")
            }
            fn visit_seq<A>(self, mut d: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let typ = d
                    .next_element::<String>()?
                    .ok_or_else(|| A::Error::custom("missing type"))?;
                let value = d
                    .next_element_seed(DeserializeValue(typ))?
                    .ok_or_else(|| A::Error::custom("missing value"))?;
                Ok(value)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut d: A) -> Result<Self::Value, A::Error> {
                let typ = d
                    .next_key::<String>()?
                    .ok_or_else(|| A::Error::custom("missing type"))?;
                if typ != "type" {
                    return Err(A::Error::custom("first field should be `type`"));
                }
                let typ = d.next_value::<String>()?;
                let value = d
                    .next_key::<String>()?
                    .ok_or_else(|| A::Error::custom("missing type"))?;
                if value != "value" {
                    return Err(A::Error::custom("second field should be `value`"));
                }
                let value = d.next_value_seed(DeserializeValue(typ))?;
                Ok(value)
            }
        }
        d.deserialize_struct("MyTrait", &["type", "value"], V {})
    }
}

trait SerializeSpec<S: Serializer> {
    fn serialize_spec(&self, s: S) -> Result<S::Ok, S::Error>;
}

impl<T: ?Sized + SerializeDyn, S: Serializer> SerializeSpec<S> for T {
    default fn serialize_spec(&self, s: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("Specialization failed."))
    }
}

impl<T: ?Sized + SerializeDyn> SerializeSpec<OctantSerializer<'_>> for T {
    fn serialize_spec(
        &self,
        s: OctantSerializer,
    ) -> Result<<OctantSerializer as Serializer>::Ok, <OctantSerializer as Serializer>::Error> {
        self.serialize_dyn(s)
    }
}

impl Serialize for dyn SerializeDyn {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <dyn SerializeDyn as SerializeSpec<S>>::serialize_spec(self, s)
    }
}

impl Serialize for dyn MyTrait {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = s.serialize_struct("MyTrait", 2)?;
        s.serialize_field("type", self.serialize_type())?;
        s.serialize_field("value", self as &dyn SerializeDyn)?;
        s.end()
    }
}

trait MyTrait: Debug + SerializeDyn {}

#[derive(Debug, Serialize, Deserialize)]
struct Foo(u32);

#[register(DESERIALIZE_REGISTRY, lazy = true)]
static FOO_REGISTER: DeserializeImp<Foo> = DeserializeImp::new("octant_serde::test::Foo");
impl SerializeType for Foo {
    fn serialize_type(&self) -> &'static str {
        FOO_REGISTER.name
    }
}
impl MyTrait for Foo {}

#[derive(Debug, Serialize, Deserialize)]
struct Bar(String);

#[register(DESERIALIZE_REGISTRY, lazy = true)]
static BAR_REGISTER: DeserializeImp<Bar> = DeserializeImp::new("octant_serde::test::Bar");
impl SerializeType for Bar {
    fn serialize_type(&self) -> &'static str {
        BAR_REGISTER.name
    }
}

impl MyTrait for Bar {}

#[test]
fn test() {
    let start: Box<dyn MyTrait> = Box::new(Foo(2));
    let encoded: Vec<u8> = serialize(&start).unwrap();
    assert_eq!(
        r#"{"type":"octant_serde::test::Foo","value":2}"#,
        std::str::from_utf8(&encoded).unwrap()
    );
    let end: Box<dyn MyTrait> = deserialize(&encoded).unwrap();
    assert_eq!(r#"Foo(2)"#, format!("{:?}", end));
}
