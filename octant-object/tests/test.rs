#![allow(unused_variables, dead_code)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]

use std::any::Any;
use std::rc::Rc;

use octant_object::cast::Cast;

trait Parent = Send + Sync + Any;

pub mod a {
    use octant_object::base;
    use octant_object::define_class;

    trait SendSync = Send + Sync;

    define_class! {
        pub class extends base implements SendSync {
            x: u32,
        }
    }
    impl Value {
        pub fn new(x: u32) -> Self {
            Value {
                parent: base::Value::new(),
                x,
            }
        }
    }
}

pub mod b {
    use octant_object::define_class;

    use super::a;

    define_class! {
        pub class extends a {
            y: u32,
        }
    }
    impl Value {
        pub fn new(x: u32, y: u32) -> Self {
            Value {
                parent: a::Value::new(x),
                y,
            }
        }
    }
}

mod c {
    use octant_object::define_class;

    use super::b;

    define_class! {
        pub class extends b{
            z:u32,
        }
    }
    impl Value {
        pub fn new(x: u32, y: u32, z: u32) -> Self {
            Value {
                parent: b::Value::new(x, y),
                z,
            }
        }
    }
}

mod d {
    use octant_object::define_class;

    use crate::c;

    define_class! {
        pub class extends c {
            w:u32,
        }
    }
    impl Value {
        pub fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
            Value {
                parent: c::Value::new(x, y, z),
                w,
            }
        }
    }
}

fn test_a_a(x: &a::Value) -> &dyn a::Trait {
    x
}

fn test_b_a(x: &b::Value) -> &dyn a::Trait {
    x
}

fn test_c_a(x: &c::Value) -> &dyn a::Trait {
    x
}

fn test_d_a(x: &d::Value) -> &dyn a::Trait {
    x
}

fn test_b_b(x: &b::Value) -> &dyn b::Trait {
    x
}

fn test_c_b(x: &c::Value) -> &dyn b::Trait {
    x
}

fn test_d_b(x: &d::Value) -> &dyn b::Trait {
    x
}

fn test_c_c(x: &c::Value) -> &dyn c::Trait {
    x
}

fn test_d_c(x: &d::Value) -> &dyn c::Trait {
    x
}

fn test_d_d(x: &d::Value) -> &dyn d::Trait {
    x
}

fn test_d_b_up(x: &dyn d::Trait) -> &dyn b::Trait {
    x
}

#[test]
fn test() {
    let x = d::Value::new(1, 2, 3, 4);
    let y: &dyn a::Trait = &x;
}

#[test]
fn test_downcast() {
    {
        let x: Rc<dyn a::Trait> = Rc::new(d::Value::new(1, 2, 3, 4));
        let x: Rc<dyn a::Trait> = x.downcast_trait().unwrap();
    }
    {
        let x: Rc<dyn a::Trait> = Rc::new(d::Value::new(1, 2, 3, 4));
        let x: Rc<dyn b::Trait> = x.downcast_trait().unwrap();
    }
    {
        let x: Rc<dyn a::Trait> = Rc::new(d::Value::new(1, 2, 3, 4));
        let x: Rc<dyn c::Trait> = x.downcast_trait().unwrap();
    }
    {
        let x: Rc<dyn a::Trait> = Rc::new(d::Value::new(1, 2, 3, 4));
        let x: Rc<dyn d::Trait> = x.downcast_trait().unwrap();
    }
}
