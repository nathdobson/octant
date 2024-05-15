use octant_serde::{define_serde_impl, define_serde_trait, deserialize, serialize, SerializeDyn};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

trait MyTrait: Debug + SerializeDyn {}

#[no_implicit_prelude]
mod a {
    use crate::define_serde_trait;
    define_serde_trait!(crate::MyTrait);
}
#[derive(Debug, Serialize, Deserialize)]
struct Foo(u32);

#[no_implicit_prelude]
mod b {
    use crate::define_serde_impl;
    define_serde_impl!(crate::Foo: crate::MyTrait);
}

impl MyTrait for Foo {}

#[derive(Debug, Serialize, Deserialize)]
struct Bar(String);

define_serde_impl!(Bar: MyTrait);

impl MyTrait for Bar {}

#[test]
fn test() {
    let start: Box<dyn MyTrait> = Box::new(Foo(2));
    let encoded: String = serialize(&start).unwrap();
    assert_eq!(r#"{"type":"test::test::Foo","value":2}"#, encoded);
    let end: Box<dyn MyTrait> = deserialize(&encoded).unwrap();
    assert_eq!(r#"Foo(2)"#, format!("{:?}", end));
}
