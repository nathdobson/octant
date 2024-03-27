use std::{mem, ptr};
use std::alloc::Layout;
use std::any::Any;
use std::marker::{PhantomData, Unsize};
use std::mem::MaybeUninit;
use std::ops::{CoerceUnsized, Deref, DerefMut};
use std::ptr::{DynMetadata, NonNull, Pointee};
use std::rc::Rc;
use std::sync::Arc;

#[allow(dead_code)]
pub struct TraitObjectStorage([*const (); 2]);

pub trait Storage: Sized {
    type Store: 'static;
    const VALID_SIZE: () =
        assert!(Layout::new::<Self>().size() == Layout::new::<Self::Store>().size());
    const VALID_ALIGN: () =
        assert!(Layout::new::<Self>().align() == Layout::new::<Self::Store>().align());
}

impl<T> Storage for Arc<T>
    where
        T: ?Sized + Pointee<Metadata=DynMetadata<T>>,
{
    type Store = TraitObjectStorage;
}

impl<T> Storage for Rc<T>
    where
        T: ?Sized + Pointee<Metadata=DynMetadata<T>>,
{
    type Store = TraitObjectStorage;
}

impl<T> Storage for Box<T>
    where
        T: ?Sized + Pointee<Metadata=DynMetadata<T>>,
{
    type Store = TraitObjectStorage;
}

pub struct StackBox<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a mut ()>);

impl<'a, T: ?Sized, U: ?Sized> CoerceUnsized<StackBox<'a, U>> for StackBox<'a, T> where T: Unsize<U> {}

impl<'a, T: ?Sized> StackBox<'a, T> {
    pub fn new(value: T, storage: &'a mut MaybeUninit<T::Store>) -> Self
        where
            T: Storage,
    {
        unsafe {
            let storage = storage.as_ptr() as *mut () as *mut T;
            storage.write(value);
            StackBox(NonNull::new(storage).unwrap(), PhantomData)
        }
    }
    pub fn into_inner(self) -> T
        where
            T: Sized,
    {
        unsafe {
            let result = ptr::read(self.0.as_ptr());
            mem::forget(self);
            result
        }
    }
    pub fn into_inner_with(self) -> (T, &'a mut MaybeUninit<T::Store>)
        where
            T: Storage,
    {
        unsafe {
            let result = ptr::read(self.0.as_ptr());
            let ptr = self.0.as_ptr() as *mut MaybeUninit<T::Store>;
            mem::forget(self);
            (result, &mut *ptr)
        }
    }
    pub fn into_raw(self) -> *mut T {
        let x = self.0.as_ptr();
        mem::forget(self);
        x
    }
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        StackBox(NonNull::new(ptr).unwrap(), PhantomData)
    }
}

impl<'a> StackBox<'a, dyn 'static + Any> {
    pub fn downcast<T>(self) -> Result<StackBox<'a, T>, Self>
        where
            T: 'static + Sized,
    {
        unsafe {
            if self.is::<T>() {
                Ok(self.downcast_unchecked())
            } else {
                Err(self)
            }
        }
    }
    pub unsafe fn downcast_unchecked<T>(self) -> StackBox<'a, T> {
        StackBox::from_raw(self.into_raw() as *mut T)
    }
}

impl<'a, T: ?Sized> Deref for StackBox<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.as_ptr() }
    }
}

impl<'a, T: ?Sized> DerefMut for StackBox<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.as_ptr() }
    }
}

impl<'a, T: ?Sized> Drop for StackBox<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.0.as_ptr());
        }
    }
}

#[test]
fn test_cast() {
    trait A {}
    struct X;
    trait B: Any {}
    impl B for Arc<dyn A> {}
    impl A for X {}
    let mut s: MaybeUninit<TraitObjectStorage> = MaybeUninit::uninit();
    let a: Arc<dyn A> = Arc::new(X);
    let b = StackBox::new(a, &mut s);
    let b2: StackBox<dyn B> = b;
    let b3: StackBox<dyn Any> = b2;
    let _b4 = b3.downcast::<Arc<dyn A>>().ok().unwrap();
}
