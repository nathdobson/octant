#![allow(dead_code)]

use octant_object::base::{Base, BaseValue};
use octant_object_derive::class;

pub struct FooValue {
    parent: BaseValue,
}

#[class]
pub trait Foo: Base {}

pub struct BarValue {
    parent: FooValue,
}

#[class]
pub trait Bar: Foo {}

#[test]
fn test() {}
