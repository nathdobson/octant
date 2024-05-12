use std::any::Any;

use crate::{
    cast::{BoxCastObject, CastTrait, CastValue},
    rank::{Ranked, Zero},
    Class, ClassValue,
};

pub trait Base: 'static + Any + CastValue {
    fn value(&self) -> &Value;
}

impl Class for dyn Base {
    type Value = Value;
}

impl ClassValue for Value {
    type Dyn = dyn Base;
}

#[derive(Debug)]
pub struct Value {}

impl Value {
    pub fn new() -> Self {
        Value {}
    }
}

impl Ranked for Value {
    type Rank = Zero;
}

//
// impl CastTrait for dyn Base {
//     // fn into_parent_object(
//     //     &self,
//     // ) -> for<'a> fn(this: StackBox<'a, dyn CastObject>) -> Option<StackBox<'a, dyn CastObject>>
//     // {
//     //     fn into_parent_object_impl<'a>(
//     //         _this: StackBox<'a, dyn CastObject>,
//     //     ) -> Option<StackBox<'a, dyn crate::cast::CastObject>> {
//     //         None
//     //     }
//     //     into_parent_object_impl
//     // }
// }

impl Base for Value {
    fn value(&self) -> &Value {
        self
    }
}
impl CastTrait for dyn Base {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject> {
        |ptr| Err(ptr)
    }
}

impl<T: ::std::ops::Deref + 'static> Base for T
where
    <T as ::std::ops::Deref>::Target: Base,
    T: CastValue,
{
    fn value(&self) -> &Value {
        T::deref(self).value()
    }
}

impl ::std::ops::Deref for dyn Base {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}
