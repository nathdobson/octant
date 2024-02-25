use std::any::Any;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::sync::Arc;

use crate::cast::{CastObject, CastTrait, CastValue};
use crate::rank::{Ranked, Zero};
use crate::stackbox::{StackBox, TraitObjectStorage};

pub trait Trait: 'static + Any + CastValue {
    fn value(&self) -> &Value;
}

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
        StackBox::<Rc<dyn Trait>>::new(self as Rc<dyn Trait>, result)
    }

    fn into_leaf_arc<'a>(
        self: Arc<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject> {
        StackBox::<Arc<dyn Trait>>::new(self as Arc<dyn Trait>, result)
    }

    fn into_leaf_box<'a>(
        self: Box<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject> {
        StackBox::<Box<dyn Trait>>::new(self as Box<dyn Trait>, result)
    }
}

impl CastTrait for dyn Trait {
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
impl Trait for Value {
    fn value(&self) -> &Value {
        self
    }
}
impl<T: ::std::ops::Deref + 'static> Trait for T
where
    <T as ::std::ops::Deref>::Target: Trait,
    T: CastValue,
{
    fn value(&self) -> &Value {
        T::deref(self).value()
    }
}

impl ::std::ops::Deref for dyn Trait {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}
