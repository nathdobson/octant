use std::any::Any;
use std::marker::Unsize;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ptr::{DynMetadata, Pointee};
use std::rc::Rc;
use std::sync::Arc;

use crate::stackbox::{StackBox, TraitObjectStorage};

trait A: 'static + Any {}

trait B: A {}
trait C: B {}

struct X;
impl A for X {}

struct Y;
impl A for Y {}
impl B for Y {}

struct Z;
impl A for Z {}
impl B for Z {}
impl C for Z {}

pub trait CastValue: 'static + Any {
    fn into_leaf_rc<'a>(
        self: Rc<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject>;
    fn into_leaf_arc<'a>(
        self: Arc<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject>;
    fn into_leaf_box<'a>(
        self: Box<Self>,
        result: &'a mut MaybeUninit<TraitObjectStorage>,
    ) -> StackBox<'a, dyn CastObject>;
}

pub trait CastTrait {
    fn into_parent_object(
        &self,
    ) -> for<'a> fn(this: StackBox<'a, dyn CastObject>) -> Option<StackBox<'a, dyn CastObject>>;
}

pub trait CastObject: Any {
    fn into_parent_object(
        &self,
    ) -> for<'a> fn(this: StackBox<'a, dyn CastObject>) -> Option<StackBox<'a, dyn CastObject>>;
}

impl<T: 'static + Deref> CastObject for T
where
    T::Target: CastTrait,
{
    fn into_parent_object(
        &self,
    ) -> for<'a> fn(this: StackBox<'a, dyn CastObject>) -> Option<StackBox<'a, dyn CastObject>>
    {
        (**self).into_parent_object()
    }
}

// impl<'a> StackBox<'a, dyn CastObject> {
//     pub fn into_parent_object(self) -> Option<Self> {
//         ((*self).into_parent_object_impl())(self)
//     }
// }

pub trait Cast<O: 'static> {
    fn downcast_trait(self) -> Option<O>;
}

pub fn coerce_unsized<
    'a,
    A: ?Sized + 'static + Pointee<Metadata = DynMetadata<A>>,
    B: ?Sized + 'static + Pointee<Metadata = DynMetadata<B>> + CastTrait,
>(
    this: StackBox<'a, dyn CastObject>,
) -> StackBox<'a, dyn CastObject>
where
    A: Unsize<B>,
{
    let this = this as StackBox<'a, dyn Any>;
    let this = match this.downcast::<Rc<A>>() {
        Ok(this) => {
            let (this, space) = this.into_inner_with();
            let this: Rc<B> = this;
            let this: StackBox<Rc<B>> = StackBox::new(this, space);
            let this: StackBox<dyn CastObject> = this;
            return this;
        }
        Err(this) => this,
    };
    let this = match this.downcast::<Arc<A>>() {
        Ok(this) => {
            let (this, space) = this.into_inner_with();
            let this: Arc<B> = this;
            let this: StackBox<Arc<B>> = StackBox::new(this, space);
            let this: StackBox<dyn CastObject> = this;
            return this;
        }
        Err(this) => this,
    };
    match this.downcast::<Box<A>>() {
        Ok(this) => {
            let (this, space) = this.into_inner_with();
            let this: Box<B> = this;
            let this: StackBox<Box<B>> = StackBox::new(this, space);
            let this: StackBox<dyn CastObject> = this;
            return this;
        }
        Err(this) => this,
    };
    panic!("Did not find the expected trait object");
}

impl<T: ?Sized + CastValue, S: 'static + ?Sized> Cast<Rc<S>> for Rc<T> {
    fn downcast_trait(self) -> Option<Rc<S>> {
        let mut space = MaybeUninit::uninit();
        let mut x = self.into_leaf_rc(&mut space);
        loop {
            if ((&*x) as &dyn Any).is::<Rc<S>>() {
                return Some(
                    (x as StackBox<dyn Any>)
                        .downcast::<Rc<S>>()
                        .ok()
                        .unwrap()
                        .into_inner(),
                );
            }
            x = ((*x).into_parent_object())(x)?;
        }
    }
}

impl<T: ?Sized + CastValue, S: 'static + ?Sized> Cast<Box<S>> for Box<T> {
    fn downcast_trait(self) -> Option<Box<S>> {
        let mut space = MaybeUninit::uninit();
        let mut x = self.into_leaf_box(&mut space);
        loop {
            if ((&*x) as &dyn Any).is::<Box<S>>() {
                return Some(
                    (x as StackBox<dyn Any>)
                        .downcast::<Box<S>>()
                        .ok()
                        .unwrap()
                        .into_inner(),
                );
            }
            x = ((*x).into_parent_object())(x)?;
        }
    }
}

impl<T: ?Sized + CastValue, S: 'static + ?Sized> Cast<Arc<S>> for Arc<T> {
    fn downcast_trait(self) -> Option<Arc<S>> {
        let mut space = MaybeUninit::uninit();
        let mut x = self.into_leaf_arc(&mut space);
        loop {
            if ((&*x) as &dyn Any).is::<Arc<S>>() {
                return Some(
                    (x as StackBox<dyn Any>)
                        .downcast::<Arc<S>>()
                        .ok()
                        .unwrap()
                        .into_inner(),
                );
            }
            x = ((*x).into_parent_object())(x)?;
        }
    }
}