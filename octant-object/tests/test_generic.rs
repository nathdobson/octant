use std::fmt::Debug;
use std::hash::Hash;
use octant_object::{
    base::{Base, BaseValue},
    define_class,
};

define_class! {
    pub class A<T1> extends Base where [T1: Debug] {
        field x:T1;
    }
}

define_class! {
    pub class B<T2> extends A<T2> where [T2: Debug + Hash] {
        field y:T2;
    }
}

define_class! {
    pub class C extends B<u32> {
        field z:u32;
    }
}

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
