#![feature(unsize)]

use std::fmt::Debug;

use octant_object::base::{Base, BaseFields};
use octant_object_derive::{class, DebugClass};

#[derive(DebugClass)]
pub struct AFields<T1: 'static + Debug> {
    parent: BaseFields,
    x: T1,
}

#[class]
pub trait A<T1: 'static + Debug>: Base {}

#[derive(DebugClass)]
pub struct BFields<T2: 'static + Debug> {
    parent: AFields<T2>,
    y: T2,
}

#[class]
pub trait B<T2: 'static + Debug>: A<T2> {}

#[derive(DebugClass)]
pub struct CFields {
    parent: BFields<u32>,
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
            CFields {
                parent: BFields {
                    parent: AFields {
                        parent: BaseFields::default(),
                        x: 4
                    },
                    y: 8,
                },
                z: 15
            }
        )
    );
}
