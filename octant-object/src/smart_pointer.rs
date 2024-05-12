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

use crate::{
    repr::{FatRepr, HasRepr, IsRepr},
};

pub unsafe trait IsSmartPointer: Deref {
    type Kind: Any;
    unsafe fn trusted_from_raw(ptr: *const Self::Target) -> Self;
    fn trusted_into_raw(this: Self) -> *const Self::Target;
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

#[repr(C)]
pub struct SmartPointer<T: ?Sized> {
    kind: TypeId,
    ptr: *const T,
}

#[repr(C)]
pub struct SmartRepr<T> {
    kind: TypeId,
    ptr: FatRepr<T>,
}

impl<T: IsRepr> IsRepr for SmartRepr<T> {}

impl<T: ?Sized> SmartPointer<T> {
    pub fn new<P: IsSmartPointer<Target = T>>(ptr: P) -> Self {
        SmartPointer {
            kind: TypeId::of::<P::Kind>(),
            ptr: P::trusted_into_raw(ptr),
        }
    }
    pub fn from_raw(kind: TypeId, ptr: *const T) -> Self {
        SmartPointer { kind, ptr }
    }
    pub fn into_raw(self) -> (TypeId, *const T) {
        let kind = self.kind;
        let ptr = self.ptr;
        mem::forget(self);
        (kind, ptr)
    }
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
    pub fn try_drop(mut self) {
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
        todo!();
    }
}

impl<T: ?Sized> Deref for SmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> HasRepr for SmartPointer<T>
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = SmartRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized + Debug> Debug for SmartPointer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, f)
    }
}

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
    let r = &*ptr;
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
