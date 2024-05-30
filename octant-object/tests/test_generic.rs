#![feature(unsize)]

use std::fmt::Debug;

use octant_object::{
    base::{Base, BaseValue},
};
use octant_object_derive::{class, DebugClass};

#[derive(DebugClass)]
pub struct AValue<T1: 'static + Debug> {
    parent: BaseValue,
    x: T1,
}

#[class]
pub trait A<T1: 'static + Debug>: Base {}

#[derive(DebugClass)]
pub struct BValue<T2: 'static + Debug> {
    parent: AValue<T2>,
    y: T2,
}

#[class]
pub trait B<T2: 'static + Debug>: A<T2> {}

#[derive(DebugClass)]
pub struct CValue {
    parent: BValue<u32>,
    z: u32,
}

#[class]
pub trait C: B<u32> {}

#[test]
fn test() {
    assert_eq!(
        "C { x: 4, y: 8, z: 15 }",
        format!(
            "{:?}",
            CValue {
                parent: BValue {
                    parent: AValue {
                        parent: BaseValue::default(),
                        x: 4
                    },
                    y: 8,
                },
                z: 15
            }
        )
    );
}
