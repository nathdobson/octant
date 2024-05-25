#![allow(unused_variables)]
#![feature(trait_upcasting)]

use std::{any::Any, fmt::Debug};

use serde::{Deserialize, Deserializer, Serialize};

use octant_serde::{
    define_serde_impl, DeserializeContext, DeserializeWith, Encoded, Format, SerializeDyn,
};

trait MyTrait: Debug + SerializeDyn + Any {}

#[derive(Debug, Serialize, Deserialize)]
struct Foo(u32);

impl<'de> DeserializeWith<'de> for Foo {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Foo::deserialize(d)
    }
}

#[no_implicit_prelude]
mod b {
    use crate::define_serde_impl;
    use crate::Foo;
    define_serde_impl!(Foo: crate::MyTrait);
}

impl MyTrait for Foo {}

#[derive(Debug, Serialize, Deserialize)]
struct Bar(String);

impl<'de> DeserializeWith<'de> for Bar {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Bar::deserialize(d)
    }
}

define_serde_impl!(Bar: MyTrait);

impl MyTrait for Bar {}

#[test]
fn test() {
    let format = Format::default();
    let start: Box<dyn MyTrait> = Box::new(Foo(2));
    let encoded = format.serialize(&*start).unwrap();
    assert_eq!(r#"2"#, encoded.as_raw().as_str().unwrap());
    let encoded = format.serialize_raw(&encoded).unwrap();
    assert_eq!(
        r#"{
  "type": "test::Foo",
  "value": "2"
}"#,
        encoded.as_str().unwrap()
    );
    let decoded = encoded.deserialize_as::<Encoded<dyn MyTrait>>().unwrap();
    assert_eq!(r#"2"#, decoded.as_raw().as_str().unwrap());
    let ctx = DeserializeContext::new();
    let decoded = decoded.deserialize_with(&ctx).unwrap();
    let decoded: Box<Foo> = Box::<dyn Any>::downcast(decoded as Box<dyn Any>).unwrap();
    assert_eq!(decoded.0, 2);
}
