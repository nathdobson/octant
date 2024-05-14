//! Utilities for downcasting objects.
//!
//! Attempt to downcast an instance of one class to an instance of a subclass. Through a combination
//! of shenanigans, arcanery, and black magic, it is possible to downcast any trait
//! object to a subclass, even if that subclass is a superclass of the actual object.
//! ```
//! # use std::sync::Arc;
//! # use octant_object::define_class;
//! # use octant_object::base::Base;
//! define_class! {
//!     #[derive(Default)]
//!     pub class Abstract extends Base {}
//! }
//! define_class! {
//!     #[derive(Default)]
//!     pub class Concrete extends Abstract {}
//! }
//!
//! use octant_object::cast::{downcast_object};
//! {
//!     let base: Arc<dyn Base> = Arc::new(ConcreteValue::default());
//!     let conc: Arc<dyn Concrete> = downcast_object(base).unwrap();
//! }
//! {
//!     let base: Arc<dyn Base> = Arc::new(ConcreteValue::default());
//!     let abs: Arc<dyn Abstract> = downcast_object(base).unwrap();
//!     let conc: Arc<dyn Concrete> = downcast_object(abs).unwrap();
//! }
//! ```

use repr::PtrRepr;
use std::{
    any::Any,
    marker::Unsize,
    ops::Deref,
    ptr::{DynMetadata, Pointee},
    sync::Arc,
};
use std::rc::Rc;

use crate::{
    base::Base,
    cast::{
        inlinebox::InlineBox,
        smart_pointer::{IsSmartPointer, SmartPointer, SmartRepr},
    },
    class::{ClassValue, Subclass},
};

pub mod inlinebox;
pub mod repr;
pub mod smart_pointer;

/// A trait implemented for `TValue` where `T` is a class. `into_leaf` returns a function that
/// converts a `SmartPointer<TValue>` to a `SmartPointer<dyn T>`.
/// ```
/// # #![feature(trait_upcasting)]
/// # use std::any::Any;
/// # use std::sync::Arc;
/// # use octant_object::base::Base;
/// # use octant_object::cast::BoxCastObject;
/// # use octant_object::cast::inlinebox::InlineBox;
/// # use octant_object::define_class;
/// # use octant_object::cast::smart_pointer::SmartPointer;
/// define_class! {
///     #[derive(Default)]
///     pub class Foo extends Base {}
/// }
/// let ptr: Arc<dyn Base> = Arc::new(FooValue::default());
/// let ptr: SmartPointer<dyn Base> = SmartPointer::new(ptr);
/// let ptr: BoxCastObject = (ptr.into_leaf())(ptr);
/// let ptr: InlineBox<dyn Any, _> = ptr.unsize();
/// let ptr: InlineBox<SmartPointer<dyn Foo>, _> = InlineBox::downcast(ptr).ok().unwrap();
/// let ptr: SmartPointer<dyn Foo> = ptr.into_inner();
/// let ptr: Arc<dyn Foo> = ptr.into_smart_pointer().ok().unwrap();
/// ```
pub trait CastValue: 'static + Any {
    fn into_leaf(&self) -> fn(SmartPointer<dyn Any>) -> BoxCastObject;
}

impl<T> CastValue for T
where
    T: ClassValue,
    <T as ClassValue>::Dyn: CastTrait,
{
    fn into_leaf(&self) -> fn(SmartPointer<dyn Any>) -> BoxCastObject {
        |ptr| {
            let ptr: SmartPointer<T> = SmartPointer::downcast::<T>(ptr).ok().unwrap();
            let ptr: SmartPointer<T::Dyn> = ptr;
            let ptr: InlineBox<SmartPointer<T::Dyn>, _> =
                InlineBox::<SmartPointer<T::Dyn>, _>::new(ptr);
            let ptr: InlineBox<dyn CastObject, _> = ptr.unsize();
            ptr
        }
    }
}

/// A trait implemented for `dyn T` where `T` is a class. If `T2` is the parent class of `T`,
/// `into_parent_object` returns a function that converts a `SmartPointer<dyn T>` to a
/// `SmartPointer<dyn T2>`. If `T` has no parent class, `into_parent_object` returns a function that
/// returns its own argument (as `Err`).
///
/// ```
/// # use std::any::Any;
/// # use std::sync::Arc;
/// # use octant_object::define_class;
/// # use octant_object::base::Base;
/// # use octant_object::cast::BoxCastObject;
/// # use octant_object::cast::inlinebox::InlineBox;
/// # use octant_object::cast::smart_pointer::SmartPointer;
/// define_class! {
///     #[derive(Default)]
///     pub class Foo extends Base {}
/// }
/// let ptr: Arc<dyn Foo> = Arc::new(FooValue::default());
/// let ptr: SmartPointer<dyn Foo> = SmartPointer::new(ptr);
/// let ptr: BoxCastObject = InlineBox::new(ptr).unsize();
/// let ptr: BoxCastObject = (ptr.into_parent_object())(ptr).ok().unwrap();
/// let ptr: InlineBox<dyn Any, _> = ptr.unsize();
/// let ptr: InlineBox<SmartPointer<dyn Base>, _> = InlineBox::downcast(ptr).ok().unwrap();
/// let ptr: SmartPointer<dyn Base> = ptr.into_inner();
/// let ptr: Arc<dyn Base> = ptr.into_smart_pointer().ok().unwrap();
/// ```
pub trait CastTrait {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject>;
}

impl<T: ?Sized> CastTrait for T
where
    T: Subclass,
    T::Parent: CastTrait,
{
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject> {
        |ptr| {
            let ptr: InlineBox<SmartPointer<T>, _> =
                InlineBox::downcast(ptr.unsize()).ok().unwrap();
            let ptr: SmartPointer<T> = ptr.into_inner();
            let ptr: SmartPointer<T::Parent> = ptr;
            let ptr: InlineBox<SmartPointer<T::Parent>, _> = InlineBox::new(ptr);
            let ptr: InlineBox<dyn CastObject, _> = ptr.unsize();
            Ok(ptr)
        }
    }
}

/// A trait implemented for `SmartPointer<dyn T>` where `T` is a class. `into_parent_object` casts a
/// SmartPointer to its parent class. See also: [CastTrait].
pub trait CastObject: Any {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject>;
}

impl<T: 'static + ?Sized + CastTrait> CastObject for SmartPointer<T> {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject> {
        (**self).into_parent_object()
    }
}

/// A dynamic representation of any smart pointer to a class (e.g. `Arc<dyn Foo>` for some class `Foo`).
pub type BoxCastObject = InlineBox<dyn CastObject, SmartRepr<PtrRepr<()>>>;

pub fn downcast_object<P1, P2: 'static>(x: P1) -> Option<P2>
where
    P1: IsSmartPointer,
    P1::Target: CastValue + Unsize<dyn Any>,
    P2: IsSmartPointer<Kind = P1::Kind>,
    <P1 as Deref>::Target: Pointee<Metadata = DynMetadata<<P1 as Deref>::Target>>,
{
    let into_leaf = x.into_leaf();
    let this: SmartPointer<P1::Target> = SmartPointer::new(x);
    let this: SmartPointer<dyn Any> = this;
    let mut this: BoxCastObject = into_leaf(this);
    loop {
        if (&*this as &dyn Any).is::<SmartPointer<P2::Target>>() {
            return Some(
                InlineBox::downcast::<SmartPointer<P2::Target>>(this.unsize())
                    .unwrap()
                    .into_inner()
                    .into_smart_pointer()
                    .ok()
                    .unwrap(),
            );
        } else {
            this = match (this.into_parent_object())(this) {
                Ok(this) => this,
                Err(this) => {
                    let this = InlineBox::downcast::<SmartPointer<dyn Base>>(this.unsize())
                        .ok()
                        .unwrap();
                    let this = this.into_inner();
                    this.try_drop();
                    return None;
                }
            }
        }
    }
}

pub fn downcast_arc<T1: ?Sized, T2: ?Sized>(x: Arc<T1>) -> Option<Arc<T2>>
where
    T1: CastValue + Unsize<dyn Any>,
    T1: Pointee<Metadata = DynMetadata<T1>>,
    T2: 'static,
{
    downcast_object(x)
}

pub fn downcast_rc<T1: ?Sized, T2: ?Sized>(x: Rc<T1>) -> Option<Rc<T2>>
    where
        T1: CastValue + Unsize<dyn Any>,
        T1: Pointee<Metadata = DynMetadata<T1>>,
        T2: 'static,
{
    downcast_object(x)
}

pub fn downcast_box<T1: ?Sized, T2: ?Sized>(x: Box<T1>) -> Option<Box<T2>>
    where
        T1: CastValue + Unsize<dyn Any>,
        T1: Pointee<Metadata = DynMetadata<T1>>,
        T2: 'static,
{
    downcast_object(x)
}