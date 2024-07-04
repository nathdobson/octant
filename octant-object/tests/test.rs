
#![allow(unused_variables, dead_code)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(dispatch_from_dyn)]
#![feature(arbitrary_self_types)]

use std::rc::Rc;
use marshal_pointer::rc_ref::RcRef;
use octant_object::{
    base,
    base::{Base, BaseFields},
    cast::downcast_object,
};
use octant_object_derive::class;

pub struct AFields {
    parent: BaseFields,
    x: u32,
}

#[class]
pub trait A: Base + Sync + Send {
    fn get_x(&self) -> &u32 {
        &self.a().x
    }
    fn fooble<'a>(self: &'a RcRef<Self>) -> &'a u32 {
        &self.x
    }
}

impl AFields {
    pub fn new(x: u32) -> AFields {
        AFields {
            parent: base::BaseFields::new(),
            x,
        }
    }
}

pub struct BFields {
    parent: AFields,
    y: u32,
}

#[class]
pub trait B: A {
    fn get_y(&self) -> &u32 {
        &self.b().y
    }
}

impl BFields {
    pub fn new(x: u32, y: u32) -> BFields {
        BFields {
            parent: AFields::new(x),
            y,
        }
    }
}

pub struct CFields {
    parent: BFields,
    z: u32,
}

#[class]
pub trait C: B {
    fn get_z(&self) -> &u32 {
        &self.c().z
    }
}

impl CFields {
    pub fn new(x: u32, y: u32, z: u32) -> CFields {
        CFields {
            parent: BFields::new(x, y),
            z,
        }
    }
}

pub struct DFields {
    parent: CFields,
    w: u32,
}

#[class]
pub trait D: C {
    fn get_w(&self) -> &u32 {
        &self.d().w
    }
}

impl DFields {
    pub fn new(x: u32, y: u32, z: u32, w: u32) -> DFields {
        DFields {
            parent: CFields::new(x, y, z),
            w,
        }
    }
}

fn test_a_a(x: &AFields) -> &dyn A {
    x
}

fn test_b_a(x: &BFields) -> &dyn A {
    x
}

fn test_c_a(x: &CFields) -> &dyn A {
    x
}

fn test_d_a(x: &DFields) -> &dyn A {
    x
}

fn test_b_b(x: &BFields) -> &dyn B {
    x
}

fn test_c_b(x: &CFields) -> &dyn B {
    x
}

fn test_d_b(x: &DFields) -> &dyn B {
    x
}

fn test_c_c(x: &CFields) -> &dyn C {
    x
}

fn test_d_c(x: &DFields) -> &dyn C {
    x
}

fn test_d_d(x: &DFields) -> &dyn D {
    x
}

fn test_d_b_up(x: &dyn D) -> &dyn B {
    x
}

#[test]
fn test() {
    let x = DFields::new(1, 2, 3, 4);
    let y: &dyn A = &x;
}

#[test]
fn test_downcast() {
    {
        let x: Rc<dyn A> = Rc::new(DFields::new(1, 2, 3, 4));
        let x: Rc<dyn A> = downcast_object(x).ok().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DFields::new(1, 2, 3, 4));
        let x: Rc<dyn B> = downcast_object(x).ok().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DFields::new(1, 2, 3, 4));
        let x: Rc<dyn C> = downcast_object(x).ok().unwrap();
    }
    {
        let x: Rc<dyn A> = Rc::new(DFields::new(1, 2, 3, 4));
        let x: Rc<dyn D> = downcast_object(x).ok().unwrap();
    }
}
