use std::{any::Any, mem::MaybeUninit, rc::Rc, sync::Arc};

use crate::{
    cast::{CastObject, CastTrait, CastValue},
    rank::{Ranked, Zero},
    stackbox::{StackBox, TraitObjectStorage},
    Class,
};

pub trait Base: 'static + Any + CastValue {
    fn value(&self) -> &Value;
}

impl Class for dyn Base {
    type Value = Value;
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

impl CastValue for Value {
    fn into_leaf_rc<'a>(
        self: Rc<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject> {
        StackBox::<Rc<dyn Base>>::new(self as Rc<dyn Base>, result)
    }

    fn into_leaf_arc<'a>(
        self: Arc<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject> {
        StackBox::<Arc<dyn Base>>::new(self as Arc<dyn Base>, result)
    }

    fn into_leaf_box<'a>(
        self: Box<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject> {
        StackBox::<Box<dyn Base>>::new(self as Box<dyn Base>, result)
    }
}

impl CastTrait for dyn Base {
    fn into_parent_object(
        &self,
    ) -> for<'a> fn(this: StackBox<'a, dyn CastObject>) -> Option<StackBox<'a, dyn CastObject>>
    {
        fn into_parent_object_impl<'a>(
            _this: StackBox<'a, dyn CastObject>,
        ) -> Option<StackBox<'a, dyn crate::cast::CastObject>> {
            None
        }
        into_parent_object_impl
    }
}

impl Base for Value {
    fn value(&self) -> &Value {
        self
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
