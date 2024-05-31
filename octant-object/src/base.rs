//! The base class that all other classes extend.

use std::{any::Any, fmt::DebugStruct};

use crate::{
    cast::{BoxCastObject, CastTrait, CastValue},
    class::{Class, ClassValue, DebugClass, Ranked, Zero},
};

pub trait Base: 'static + Any + CastValue {
    fn value(&self) -> &BaseFields;
}

impl Class for dyn Base {
    type Fields = BaseFields;
}

impl ClassValue for BaseFields {
    type Dyn = dyn Base;
}

#[derive(Debug)]
pub struct BaseFields {}

impl BaseFields {
    pub fn new() -> Self {
        BaseFields {}
    }
}

impl Ranked for BaseFields {
    type Rank = Zero;
}

impl Base for BaseFields {
    fn value(&self) -> &BaseFields {
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
    fn value(&self) -> &BaseFields {
        T::deref(self).value()
    }
}

impl ::std::ops::Deref for dyn Base {
    type Target = BaseFields;
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl Default for BaseFields {
    fn default() -> Self {
        BaseFields {}
    }
}

impl DebugClass for BaseFields {
    fn fmt_class(&self, _: &mut DebugStruct) {}
}
