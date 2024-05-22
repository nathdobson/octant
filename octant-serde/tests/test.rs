#![allow(unused_variables)]

use std::fmt::Debug;

use serde::{Deserialize, Deserializer, Serialize};
use type_map::TypeMap;

use octant_serde::{
    define_serde_impl, define_serde_trait, deserialize, serialize, DeserializeWith, SerializeDyn,
};

trait MyTrait: Debug + SerializeDyn {}

#[no_implicit_prelude]
mod a {
    use crate::define_serde_trait;

    define_serde_trait!(crate::MyTrait);
}

#[derive(Debug, Serialize, Deserialize)]
struct Foo(u32);

impl<'de> DeserializeWith<'de> for Foo {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        Foo::deserialize(d)
    }
}

#[no_implicit_prelude]
mod b {
    use crate::define_serde_impl;

    define_serde_impl!(crate::Foo: crate::MyTrait);
}

impl MyTrait for Foo {}

#[derive(Debug, Serialize, Deserialize)]
struct Bar(String);

impl<'de> DeserializeWith<'de> for Bar {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        Bar::deserialize(d)
    }
}

define_serde_impl!(Bar: MyTrait);

impl MyTrait for Bar {}

#[test]
fn test() {
    let start: Box<dyn MyTrait> = Box::new(Foo(2));
    let encoded: String = serialize(&start).unwrap();
    assert_eq!(r#"{"type":"test::Foo","value":2}"#, encoded);
    let end: Box<dyn MyTrait> = deserialize(&TypeMap::new(), &encoded).unwrap();
    assert_eq!(r#"Foo(2)"#, format!("{:?}", end));
}
