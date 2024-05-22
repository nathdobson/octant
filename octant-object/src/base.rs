//! The base class that all other classes extend.

use std::any::Any;

use crate::{
    cast::{BoxCastObject, CastTrait, CastValue},
    class::{Class, ClassValue, Ranked, Zero},
};

pub trait Base: 'static + Any + CastValue {
    fn value(&self) -> &BaseValue;
}

impl Class for dyn Base {
    type Value = BaseValue;
}

impl ClassValue for BaseValue {
    type Dyn = dyn Base;
}

#[derive(Debug)]
pub struct BaseValue {}

impl BaseValue {
    pub fn new() -> Self {
        BaseValue {}
    }
}

impl Ranked for BaseValue {
    type Rank = Zero;
}

impl Base for BaseValue {
    fn value(&self) -> &BaseValue {
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
    fn value(&self) -> &BaseValue {
        T::deref(self).value()
    }
}

impl ::std::ops::Deref for dyn Base {
    type Target = BaseValue;
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl Default for BaseValue {
    fn default() -> Self {
        BaseValue {}
    }
}
