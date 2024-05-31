#![allow(dead_code)]

use octant_object::base::{Base, BaseFields};
use octant_object_derive::class;

pub struct FooFields {
    parent: BaseFields,
}

#[class]
pub trait Foo: Base {}

pub struct BarFields {
    parent: FooFields,
}

#[class]
pub trait Bar: Foo {}

#[test]
fn test() {}
