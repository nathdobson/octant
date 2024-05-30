#![allow(dead_code)]

use octant_object::base::Base;
use octant_object_derive::class;

#[class]
pub struct Foo {
    parent: dyn Base,
}
pub trait Foo: AsFoo {}
impl<T> Foo for T where T: AsFoo {}

#[class]
pub struct Bar {
    parent: dyn Foo,
}

pub trait Bar: AsBar {}
impl<T> Bar for T where T: AsBar {}

#[test]
fn test() {}
