#![allow(unused_variables, dead_code)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]

use std::{any::Any, rc::Rc};

use octant_object::cast::Cast;

trait Parent = Send + Sync + Any;

use octant_object::{base, base::Base, define_class};
trait SendSync = Send + Sync;

define_class! {
    pub class A extends Base implements SendSync {
        x: u32,
    }
}

impl AValue {
    pub fn new(x: u32) -> AValue {
        AValue {
            parent: base::Value::new(),
            x,
        }
    }
}

define_class! {
    pub class B extends A {
        y: u32,
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
        z:u32,
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
        w:u32,
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
        let x: Rc<dyn A> = x.downcast_trait().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DValue::new(1, 2, 3, 4));
        let x: Rc<dyn B> = x.downcast_trait().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DValue::new(1, 2, 3, 4));
        let x: Rc<dyn C> = x.downcast_trait().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DValue::new(1, 2, 3, 4));
        let x: Rc<dyn D> = x.downcast_trait().unwrap();
    }
}
