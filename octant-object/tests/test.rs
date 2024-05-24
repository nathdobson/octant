#![allow(unused_variables, dead_code)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(dispatch_from_dyn)]
#![feature(arbitrary_self_types)]

use std::rc::Rc;

use octant_object::{base, base::Base, cast::downcast_object, define_class};

trait SendSync = Send + Sync;

define_class! {
    pub class A extends Base implements SendSync {
        field x: u32;
        fn get_x<'a>(self:&'a Self) -> &'a u32{
            &self.a().x
        }
    }
}

impl AValue {
    pub fn new(x: u32) -> AValue {
        AValue {
            parent: base::BaseValue::new(),
            x,
        }
    }
}

define_class! {
    pub class B extends A {
        field y: u32;
    }
}
impl BValue {
    pub fn new(x: u32, y: u32) -> BValue {
        BValue {
            parent: AValue::new(x),
            y,
        }
    }
}

define_class! {
    pub class C extends B{
        field z:u32;
    }
}

impl CValue {
    pub fn new(x: u32, y: u32, z: u32) -> CValue {
        CValue {
            parent: BValue::new(x, y),
            z,
        }
    }
}

define_class! {
    pub class D extends C {
        field w:u32;
    }
}

impl DValue {
    pub fn new(x: u32, y: u32, z: u32, w: u32) -> DValue {
        DValue {
            parent: CValue::new(x, y, z),
            w,
        }
    }
}

fn test_a_a(x: &AValue) -> &dyn A {
    x
}

fn test_b_a(x: &BValue) -> &dyn A {
    x
}

fn test_c_a(x: &CValue) -> &dyn A {
    x
}

fn test_d_a(x: &DValue) -> &dyn A {
    x
}

fn test_b_b(x: &BValue) -> &dyn B {
    x
}

fn test_c_b(x: &CValue) -> &dyn B {
    x
}

fn test_d_b(x: &DValue) -> &dyn B {
    x
}

fn test_c_c(x: &CValue) -> &dyn C {
    x
}

fn test_d_c(x: &DValue) -> &dyn C {
    x
}

fn test_d_d(x: &DValue) -> &dyn D {
    x
}

fn test_d_b_up(x: &dyn D) -> &dyn B {
    x
}

#[test]
fn test() {
    let x = DValue::new(1, 2, 3, 4);
    let y: &dyn A = &x;
}

#[test]
fn test_downcast() {
    {
        let x: Rc<dyn A> = Rc::new(DValue::new(1, 2, 3, 4));
        let x: Rc<dyn A> = downcast_object(x).ok().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DValue::new(1, 2, 3, 4));
        let x: Rc<dyn B> = downcast_object(x).ok().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DValue::new(1, 2, 3, 4));
        let x: Rc<dyn C> = downcast_object(x).ok().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DValue::new(1, 2, 3, 4));
        let x: Rc<dyn D> = downcast_object(x).ok().unwrap();
    }
}
