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
//! define_class! {
//!     #[derive(Default)]
//!     pub class Other extends Abstract {}
//! }
//!
//! use octant_object::cast::{downcast_object};
//! {
//!     // Cast to the concrete class
//!     let base: Arc<dyn Base> = Arc::new(ConcreteValue::default());
//!     let conc: Arc<dyn Concrete> = downcast_object(base).ok().unwrap();
//! }
//! {
//!     // Cast to a parent class
//!     let base: Arc<dyn Base> = Arc::new(ConcreteValue::default());
//!     let abs: Arc<dyn Abstract> = downcast_object(base).ok().unwrap();
//! }
//! {
//!     // Fail to make an illegal cast, but recover the original pointer.
//!     let base: Arc<dyn Base> = Arc::new(ConcreteValue::default());
//!     let base: Arc<dyn Base> = downcast_object::<_, Arc<dyn Other>>(base).err().unwrap();
//! }
//! ```
//! # Performance notes
//! Casting an object works as follows:
//! 1. The object is first cast to its concrete class.
//! 1. The object is iteratively cast to its immediate parent class.
//! 1. Once the desired class is reached, the object is reached.
//! 1. If the [Base] class is reached, the original object is recovered by repeating the same process.
//!
//! While casting uses no allocations, it does extensively use dynamic dispatch. As such, compiler
//! optimizations may not be very effective. The iterative process may make casts for deep
//! hierarchies inefficient.
use std::{
    any::Any,
    marker::Unsize,
    mem,
    ops::Deref,
    ptr::{DynMetadata, Pointee},
    rc::Rc,
    sync::Arc,
};

use repr::PtrRepr;

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
    T::Dyn: CastTrait,
{
    fn into_leaf(&self) -> fn(SmartPointer<dyn Any>) -> BoxCastObject {
        |ptr| {
            let ptr: SmartPointer<T> = SmartPointer::downcast(ptr).ok().unwrap();
            let ptr: SmartPointer<T::Dyn> = ptr;
            let ptr: InlineBox<SmartPointer<T::Dyn>, _> =
                InlineBox::<SmartPointer<T::Dyn>, _>::new(ptr);
            let ptr: InlineBox<dyn CastSmartPointer, _> = ptr.unsize();
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
            let ptr: InlineBox<dyn CastSmartPointer, _> = ptr.unsize();
            Ok(ptr)
        }
    }
}

/// A trait implemented for `SmartPointer<dyn T>` where `T` is a class. `into_parent_object` casts a
/// SmartPointer to its parent class. See also: [CastTrait].
pub trait CastSmartPointer: Any {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject>;
}

impl<T: 'static + ?Sized + CastTrait> CastSmartPointer for SmartPointer<T> {
    fn into_parent_object(&self) -> fn(BoxCastObject) -> Result<BoxCastObject, BoxCastObject> {
        (**self).into_parent_object()
    }
}

/// A dynamic representation of any smart pointer to a class (e.g. `Arc<dyn Foo>` for some class `Foo`).
pub type BoxCastObject = InlineBox<dyn CastSmartPointer, SmartRepr<PtrRepr<()>>>;

fn downcast_object_impl<T1: Unsize<dyn Any> + ?Sized + CastValue, T2: 'static + ?Sized>(
    ptr: SmartPointer<T1>,
) -> Result<SmartPointer<T2>, SmartPointer<dyn Base>> {
    let into_leaf = ptr.into_leaf();
    let mut ptr = into_leaf(ptr);
    loop {
        if (&*ptr as &dyn Any).is::<SmartPointer<T2>>() {
            let ptr: InlineBox<dyn Any, _> = ptr.unsize();
            let ptr: InlineBox<SmartPointer<T2>, _> = InlineBox::downcast(ptr).ok().unwrap();
            let ptr: SmartPointer<T2> = ptr.into_inner();
            return Ok(ptr);
        } else {
            ptr = match (ptr.into_parent_object())(ptr) {
                Ok(ptr) => ptr,
                Err(ptr) => {
                    let ptr: InlineBox<dyn Any, _> = ptr.unsize();
                    let ptr: InlineBox<SmartPointer<dyn Base>, _> =
                        InlineBox::downcast(ptr).ok().unwrap();
                    let ptr: SmartPointer<dyn Base> = ptr.into_inner();
                    return Err(ptr);
                }
            }
        }
    }
}

pub fn downcast_object<P1, P2: 'static>(ptr: P1) -> Result<P2, P1>
where
    P1: IsSmartPointer,
    P1::Target: CastValue + Unsize<dyn Any>,
    P2: IsSmartPointer<Kind = P1::Kind>,
    <P1 as Deref>::Target: Pointee<Metadata = DynMetadata<<P1 as Deref>::Target>>,
{
    let ptr = SmartPointer::new(ptr);
    match downcast_object_impl::<P1::Target, P2::Target>(ptr) {
        Ok(ptr) => Ok(ptr.into_smart_pointer().ok().unwrap()),
        Err(ptr) => {
            match downcast_object_impl::<dyn Base, P1::Target>(ptr) {
                Ok(ptr) => Err(ptr.into_smart_pointer().ok().unwrap()),
                Err(ptr) => {
                    mem::forget(ptr);
                    panic!("Failed to recover original object after downcast. This causes a memory leak.");
                }
            }
        }
    }
}

pub fn downcast_arc<T1: ?Sized, T2: ?Sized>(x: Arc<T1>) -> Result<Arc<T2>, Arc<T1>>
where
    T1: CastValue + Unsize<dyn Any>,
    T1: Pointee<Metadata = DynMetadata<T1>>,
    T2: 'static,
{
    downcast_object(x)
}

pub fn downcast_rc<T1: ?Sized, T2: ?Sized>(x: Rc<T1>) -> Result<Rc<T2>, Rc<T1>>
where
    T1: CastValue + Unsize<dyn Any>,
    T1: Pointee<Metadata = DynMetadata<T1>>,
    T2: 'static,
{
    downcast_object(x)
}

pub fn downcast_box<T1: ?Sized, T2: ?Sized>(x: Box<T1>) -> Result<Box<T2>, Box<T1>>
where
    T1: CastValue + Unsize<dyn Any>,
    T1: Pointee<Metadata = DynMetadata<T1>>,
    T2: 'static,
{
    downcast_object(x)
}
