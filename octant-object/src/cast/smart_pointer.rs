//! An abstraction over smart pointers.
//!
//! This is conceptually similar to an enum:
//! ```
//! # use ::std::sync::Arc;
//! # use ::std::rc::Rc;
//! enum SmartPointer<T: ?Sized>{
//!     Box(Box<T>),
//!     Rc(Rc<T>),
//!     Arc(Arc<T>),
//! }
//! ```
//! However, `SmartPointer` can be extended to support any type of smart pointer via the [IsSmartPointer] trait.
//!
//! Destructors are currently unimplemented, so callers must invoke `into_smart_pointer` instead of dropping a `SmartPointer`.

use std::{
    any::{Any, TypeId},
    fmt::{Debug, Formatter},
    marker::Unsize,
    mem,
    ops::{CoerceUnsized, Deref},
    ptr::Pointee,
    rc::Rc,
    sync::Arc,
};

use crate::cast::repr::{HasRepr, IsRepr, PtrRepr};

/// Indicates that a type is convertible to and from a raw pointer (e.g. `Box`, `Arc`, and `Rc`).
///
/// There is `unsafe` code relying on the correctness of this trait, so implementations are `unsafe`.
pub unsafe trait IsSmartPointer: Deref {
    type Kind: Any;
    /// Convert `this` into a raw pointer and grant ownership to the caller. To avoid memory leaks,
    /// the caller should eventually pass the pointer to `trusted_from_raw`. The raw pointer must be
    /// safe to dereference (before it is passed to `trusted_from_raw`).
    fn trusted_into_raw(this: Self) -> *const Self::Target;
    /// Convert a raw pointer returned by `trusted_into_raw` back into `Self`.
    /// Callers must ensure that:
    /// * `trusted_from_raw` is called at most once for any call to `trusted_into_raw`.
    /// * The pointer passed to `trusted_from_raw` is a valid pointer to the same object as the pointer returned by `trusted_into_raw`.
    ///     * The narrow pointer must be the same.
    ///     * The pointer metadata for trait objects must be valid for the referenced object.
    ///     * The pointer metadata for slices must be the same.
    /// * `Self::Kind` must be the same for the call to `trusted_into_raw` and `trusted_from_raw`.
    unsafe fn trusted_from_raw(ptr: *const Self::Target) -> Self;
}

unsafe impl<T: ?Sized> IsSmartPointer for Box<T> {
    type Kind = Box<()>;
    unsafe fn trusted_from_raw(ptr: *const Self::Target) -> Self {
        Box::from_raw(ptr as *mut Self::Target)
    }
    fn trusted_into_raw(this: Self) -> *const Self::Target {
        Box::into_raw(this)
    }
}

unsafe impl<T: ?Sized> IsSmartPointer for Arc<T> {
    type Kind = Arc<()>;
    unsafe fn trusted_from_raw(ptr: *const Self::Target) -> Self {
        Arc::from_raw(ptr as *mut Self::Target)
    }
    fn trusted_into_raw(this: Self) -> *const Self::Target {
        Arc::into_raw(this)
    }
}

unsafe impl<T: ?Sized> IsSmartPointer for Rc<T> {
    type Kind = Rc<()>;
    unsafe fn trusted_from_raw(ptr: *const Self::Target) -> Self {
        Rc::from_raw(ptr as *mut Self::Target)
    }
    fn trusted_into_raw(this: Self) -> *const Self::Target {
        Rc::into_raw(this)
    }
}

/// See [crate::cast::smart_pointer].
#[repr(C)]
pub struct SmartPointer<T: ?Sized> {
    kind: TypeId,
    ptr: *const T,
}

impl<T: ?Sized> SmartPointer<T> {
    /// ```
    /// # use std::mem;
    /// # use std::rc::Rc;
    /// # use std::sync::Arc;
    /// # use octant_object::cast::smart_pointer::SmartPointer;
    /// let ptrs: [SmartPointer<i32>; 3] = [
    ///     SmartPointer::new(Box::new(1)),
    ///     SmartPointer::new(Rc::new(2)),
    ///     SmartPointer::new(Arc::new(3))
    /// ];
    /// assert_eq!(*ptrs[0], 1);
    /// assert_eq!(*ptrs[1], 2);
    /// assert_eq!(*ptrs[2], 3);
    /// # let [p1,p2,p3]=ptrs;
    /// # p1.into_smart_pointer::<Box<i32>>();
    /// # p2.into_smart_pointer::<Rc<i32>>();
    /// # p3.into_smart_pointer::<Arc<i32>>();
    /// ```
    pub fn new<P: IsSmartPointer<Target = T>>(ptr: P) -> Self {
        SmartPointer {
            kind: TypeId::of::<P::Kind>(),
            ptr: P::trusted_into_raw(ptr),
        }
    }
    fn into_raw(self) -> (TypeId, *const T) {
        let kind = self.kind;
        let ptr = self.ptr;
        mem::forget(self);
        (kind, ptr)
    }
    /// Convert back into the original smart pointer.
    /// ```
    /// # use std::rc::Rc;
    /// # use octant_object::cast::smart_pointer::SmartPointer;
    /// let ptr: Rc<i32> = Rc::new(42);
    /// let ptr: SmartPointer<i32> = SmartPointer::new(ptr);
    /// let ptr: Rc<i32> = ptr.into_smart_pointer().ok().unwrap();
    /// assert_eq!(*ptr, 42);
    /// ```
    pub fn into_smart_pointer<P: IsSmartPointer<Target = T>>(self) -> Result<P, Self> {
        unsafe {
            if self.kind == TypeId::of::<P::Kind>() {
                let (_, ptr) = self.into_raw();
                Ok(P::trusted_from_raw(ptr))
            } else {
                Err(self)
            }
        }
    }
    pub(crate) fn try_drop(mut self) {
        self = match self.into_smart_pointer::<Arc<_>>() {
            Ok(_) => return,
            Err(x) => x,
        };
        self = match self.into_smart_pointer::<Rc<_>>() {
            Ok(_) => return,
            Err(x) => x,
        };
        self = match self.into_smart_pointer::<Box<_>>() {
            Ok(_) => return,
            Err(x) => x,
        };
        mem::forget(self);
        panic!("Cannot drop");
    }
}

impl SmartPointer<dyn Any> {
    /// Attempt to downcast `this` to a concrete type `T2`.
    pub fn downcast<T2: Any>(this: Self) -> Result<SmartPointer<T2>, Self> {
        if ((&*this) as &dyn Any).is::<T2>() {
            let (kind, ptr) = this.into_raw();
            Ok(SmartPointer {
                kind,
                ptr: ptr as *const T2,
            })
        } else {
            Err(this)
        }
    }
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<SmartPointer<U>> for SmartPointer<T> where T: Unsize<U> {}

impl<T: ?Sized> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        todo!("SmartPointer::drop is not implemented");
    }
}

impl<T: ?Sized> Deref for SmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized + Debug> Debug for SmartPointer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, f)
    }
}
/// The [representation](crate::cast::repr) of a `SmartPointer`.
#[repr(C)]
pub struct SmartRepr<T> {
    kind: TypeId,
    ptr: PtrRepr<T>,
}

impl<T: IsRepr> IsRepr for SmartRepr<T> {}

impl<T: ?Sized> HasRepr for SmartPointer<T>
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = SmartRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

#[cfg(test)]
mod test {
    use std::{
        any::{Any, TypeId},
        fmt::Debug,
        mem::size_of,
    };

    use crate::cast::inlinebox::InlineBox;
    use crate::cast::smart_pointer::SmartPointer;

    #[test]
    fn test_smart_pointer() {
        let foo = Box::new(3);
        let foo: SmartPointer<i32> = SmartPointer::new(foo);
        println!("{:?}", foo);
        let _foo: Box<_> = foo.into_smart_pointer().ok().unwrap();
    }

    #[test]
    fn test_smart_pointer_object() {
        let ptr = Box::new(3);
        let ptr: Box<dyn Debug> = ptr;
        let ptr: SmartPointer<dyn Debug> = SmartPointer::new(ptr);
        assert_eq!(format!("{:?}", ptr), "3");
        let _foo: Box<_> = ptr.into_smart_pointer().ok().unwrap();
    }

    #[test]
    fn test_trait_object() {
        trait Boop: Any + Debug {}
        assert_eq!(size_of::<TypeId>(), 16);
        assert_eq!(size_of::<*const dyn Boop>(), 16);
        assert_eq!(size_of::<SmartPointer<dyn Boop>>(), 32);
        impl<T: ?Sized + Any + Debug> Boop for T {}
        let ptr = Box::new(3);
        assert_eq!(format!("{:?}", ptr), "3");
        let ptr: Box<dyn Boop> = ptr;
        assert_eq!(format!("{:?}", ptr), "3");
        let ptr: SmartPointer<dyn Boop> = SmartPointer::new(ptr);
        assert_eq!(format!("{:?}", ptr), "3");
        let ptr = InlineBox::<SmartPointer<dyn Boop>, _>::new(ptr);
        assert_eq!(format!("{:?}", ptr), "3");
        let ptr: InlineBox<dyn Boop, _> = ptr.unsize();
        assert_eq!(format!("{:?}", ptr), "3");
        let ptr: InlineBox<dyn Any, _> = ptr.unsize();
        let ptr: InlineBox<SmartPointer<dyn Boop>, _> = InlineBox::downcast(ptr).ok().unwrap();
        let ptr: SmartPointer<dyn Boop> = ptr.into_inner();
        let ptr: Box<dyn Boop> = ptr.into_smart_pointer().ok().unwrap();
        let ptr: Box<dyn Any> = ptr;
        let ptr: Box<i32> = Box::<dyn Any>::downcast(ptr).ok().unwrap();
        let ptr: i32 = *ptr;
        assert_eq!(ptr, 3);
    }
}
