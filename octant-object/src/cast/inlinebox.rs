use std::{
    any::Any,
    fmt::{Debug, Formatter},
    marker::{PhantomData, Unsize},
    mem,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr,
    ptr::{null, Pointee},
};

use crate::cast::repr::HasRepr;

/// A `Box<T>` that allocates memory from a field within itself. This requires the caller to specify
/// a representation type `R` with the same layout as the stored value.
pub struct InlineBox<T: ?Sized, R> {
    value: MaybeUninit<R>,
    metadata: <T as Pointee>::Metadata,
    phantom: PhantomData<T>,
}

impl<T: ?Sized, R> InlineBox<T, R> {
    /// Construct a new `InlineBox` containing the specified value. This statically verifies that
    /// the value has the same layout as the representation.
    ///
    /// ```
    /// use octant_object::cast::inlinebox::InlineBox;
    /// let foo: InlineBox<i32, u32> = InlineBox::new(42);
    /// assert_eq!(*foo, 42);
    /// ```
    pub fn new(value: T) -> Self
    where
        T: Sized + HasRepr<Repr = R>,
    {
        let _: () = T::VALID_SIZE;
        let _: () = T::VALID_ALIGN;
        unsafe {
            let value2 = mem::transmute_copy::<T, MaybeUninit<R>>(&value);
            mem::forget(value);
            InlineBox {
                value: value2,
                metadata: (),
                phantom: PhantomData,
            }
        }
    }
    /// Perform an [`unsized coercion`](https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions):
    /// * Convert an array `InlineBox<[Elem; N]>` to a slice `InlineBox<[Elem]>`.
    /// * Convert a concrete type `InlineBox<Concrete>` to a trait object `InlineBox<dyn Trait>`.
    /// * Upcast a trait object `InlineBox<dyn ChildTrait>` to a trait object `InlineBox<dyn ParentTrait>`.
    /// ```
    /// # use std::fmt::Debug;
    /// use octant_object::cast::inlinebox::InlineBox;
    /// let foo: InlineBox<i32, u32> = InlineBox::new(42);
    /// let foo: InlineBox<dyn Debug, u32> = foo.unsize();
    /// assert_eq!(format!("{:?}", foo), "42");
    /// ```
    ///
    /// When using other pointer types, unsized coercions are implicit: Due to limitation in the
    /// behavior of the [::std::ops::CoerceUnsized] trait, unsized coercions of `InlineBox` must be
    /// explicit.   
    pub fn unsize<U: ?Sized>(self) -> InlineBox<U, R>
    where
        T: Unsize<U>,
    {
        unsafe {
            let r: *const MaybeUninit<R> = &self.value;
            let metadata = self.metadata;
            let value = ptr::read(r);
            mem::forget(self);
            let metadata = std::ptr::metadata::<U>(
                std::ptr::from_raw_parts::<T>(null(), metadata) as *const U
            );
            InlineBox {
                value,
                metadata,
                phantom: PhantomData,
            }
        }
    }
    /// Return the wrapped value
    pub fn into_inner(mut self) -> T
    where
        T: Sized,
    {
        unsafe {
            let result = ptr::read(self.value.as_mut_ptr() as *mut T);
            mem::forget(self);
            result
        }
    }
}

impl<R> InlineBox<dyn Any, R> {
    /// Attempt to downcast to a concrete type.
    pub fn downcast<T: Any>(this: Self) -> Result<InlineBox<T, R>, Self> {
        unsafe {
            let r = &*this;
            if r.is::<T>() {
                let value = ptr::read(&this.value);
                mem::forget(this);
                Ok(InlineBox {
                    value,
                    metadata: (),
                    phantom: PhantomData,
                })
            } else {
                Err(this)
            }
        }
    }
}

impl<T: ?Sized, R> Deref for InlineBox<T, R> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*std::ptr::from_raw_parts::<T>(self.value.as_ptr() as *const (), self.metadata) }
    }
}

impl<T: ?Sized, R> DerefMut for InlineBox<T, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *std::ptr::from_raw_parts_mut::<T>(
                self.value.as_mut_ptr() as *mut (),
                self.metadata,
            )
        }
    }
}

impl<T: ?Sized, R> Drop for InlineBox<T, R> {
    fn drop(&mut self) {
        unsafe {
            let ptr: *mut T = self.deref_mut();
            std::ptr::drop_in_place(ptr);
        }
    }
}

impl<T: ?Sized + Debug, R> Debug for InlineBox<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

#[test]
fn test_simple_box() {
    let foo = Box::new(3);
    let foo: InlineBox<Box<i32>, _> = InlineBox::new(foo);
    let foo: InlineBox<dyn Debug, _> = foo.unsize();
    assert_eq!("3", format!("{:?}", foo));
}

#[test]
fn test_object_box() {
    let foo = Box::new(3);
    let foo: Box<dyn Debug> = foo;
    let foo: InlineBox<Box<dyn Debug>, _> = InlineBox::new(foo);
    let foo: InlineBox<dyn Debug, _> = foo.unsize();
    assert_eq!("3", format!("{:?}", foo));
}
