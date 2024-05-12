use crate::repr::HasRepr;
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

pub struct InlineBox<T: ?Sized, S> {
    value: MaybeUninit<S>,
    metadata: <T as Pointee>::Metadata,
    phantom: PhantomData<T>,
}

impl<T: ?Sized, S> InlineBox<T, S> {
    pub fn new(value: T) -> Self
    where
        T: Sized + HasRepr<Repr = S>,
    {
        let _: () = T::VALID_SIZE;
        let _: () = T::VALID_ALIGN;
        unsafe {
            let value2 = mem::transmute_copy::<T, MaybeUninit<S>>(&value);
            mem::forget(value);
            InlineBox {
                value: value2,
                metadata: (),
                phantom: PhantomData,
            }
        }
    }
    pub fn unsize<U: ?Sized>(self) -> InlineBox<U, S>
    where
        T: Unsize<U>,
    {
        unsafe {
            let r: *const MaybeUninit<S> = &self.value;
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

impl<S> InlineBox<dyn Any, S> {
    pub fn downcast<T: Any>(this: Self) -> Result<InlineBox<T, S>, Self> {
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

impl<T: ?Sized, S> Deref for InlineBox<T, S> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*std::ptr::from_raw_parts::<T>(self.value.as_ptr() as *const (), self.metadata) }
    }
}

impl<T: ?Sized, S> DerefMut for InlineBox<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *std::ptr::from_raw_parts_mut::<T>(
                self.value.as_mut_ptr() as *mut (),
                self.metadata,
            )
        }
    }
}

impl<T: ?Sized, S> Drop for InlineBox<T, S> {
    fn drop(&mut self) {
        unsafe {
            let ptr: *mut T = self.deref_mut();
            std::ptr::drop_in_place(ptr);
        }
    }
}

impl<T: ?Sized + Debug, S> Debug for InlineBox<T, S> {
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
