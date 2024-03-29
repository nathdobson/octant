use std::{
    cell::UnsafeCell,
    mem,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
};

#[repr(C)]
struct ArcInner<T: ?Sized> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    inner: UnsafeCell<T>,
}

pub struct UniqueArc<T: ?Sized>(NonNull<ArcInner<T>>);

const MAX_REFCOUNT: usize = (isize::MAX) as usize;

impl<T: ?Sized> UniqueArc<T> {
    pub fn new(value: T) -> Self
    where
        T: Sized,
    {
        UniqueArc(
            NonNull::new(Box::into_raw(Box::new(ArcInner {
                strong: AtomicUsize::new(0),
                weak: AtomicUsize::new(1),
                inner: UnsafeCell::new(value),
            })))
            .unwrap(),
        )
    }
    pub fn downgrade(this: &Self) -> Weak<T> {
        unsafe {
            this.0.as_ref().weak.fetch_add(1, Ordering::Relaxed);
            Weak::from_raw(this.0.as_ref().inner.get())
        }
    }
    pub fn into_arc(self) -> Arc<T> {
        unsafe {
            let raw: *mut T = self.0.as_ref().inner.get();
            self.0.as_ref().strong.fetch_add(1, Ordering::Release);
            mem::forget(self);
            Arc::from_raw(raw)
        }
    }
}

impl<T: ?Sized> Deref for UniqueArc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.as_ref().inner.get() }
    }
}

impl<T: ?Sized> DerefMut for UniqueArc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.as_ref().inner.get() }
    }
}

impl<T: ?Sized> Drop for UniqueArc<T> {
    fn drop(&mut self) {
        unsafe {
            self.0.as_ref().inner.get().drop_in_place();
            mem::drop(Weak::from_raw(self.0.as_ref().inner.get()));
        }
    }
}

impl<T> UniqueArc<MaybeUninit<T>> {
    pub fn new_uninit() -> Self {
        Self::new(MaybeUninit::uninit())
    }
    pub fn init(mut self, value: T) -> Arc<T> {
        unsafe {
            (*self).write(value);
            mem::transmute::<Arc<MaybeUninit<T>>, Arc<T>>(self.into_arc())
        }
    }
    pub fn downgrade_uninit(this: &Self) -> Weak<T> {
        unsafe { mem::transmute::<Weak<MaybeUninit<T>>, Weak<T>>(Self::downgrade(this)) }
    }
}

struct AssertDropped {
    dropped: bool,
}

impl AssertDropped {
    pub fn new() -> Self {
        AssertDropped { dropped: false }
    }
    pub fn check(&mut self) -> MustDrop {
        MustDrop(self)
    }
}
struct MustDrop<'a>(&'a mut AssertDropped);

impl<'a> Drop for MustDrop<'a> {
    fn drop(&mut self) {
        assert!(!self.0.dropped);
        self.0.dropped = true;
    }
}

impl Drop for AssertDropped {
    fn drop(&mut self) {
        assert!(self.dropped);
    }
}

#[test]
fn test_without_arc() {
    let mut assert = AssertDropped::new();
    let x = UniqueArc::new(assert.check());
}

#[test]
fn test_with_arc() {
    let mut assert = AssertDropped::new();
    let x = UniqueArc::new(assert.check());
    let x = x.into_arc();
}

#[test]
fn test_with_weak() {
    let mut assert = AssertDropped::new();
    let x = UniqueArc::new(assert.check());
    let w = UniqueArc::downgrade(&x);
    let x = x.into_arc();
    mem::drop(w);
}

#[test]
fn test_uninit() {
    let x = UniqueArc::<MaybeUninit<MustDrop>>::new_uninit();
}

#[test]
fn test_uninit_arc() {
    let mut assert = AssertDropped::new();
    let x = UniqueArc::<MaybeUninit<MustDrop>>::new_uninit();
    x.init(assert.check());
}

#[test]
fn test_uninit_weak() {
    let mut assert = AssertDropped::new();
    let x = UniqueArc::<MaybeUninit<MustDrop>>::new_uninit();
    let w = UniqueArc::downgrade_uninit(&x);
    assert!(w.upgrade().is_none());
    x.init(assert.check());
}

// use std::{
//     fmt,
//     marker::Unsize,
//     ops::{CoerceUnsized, DispatchFromDyn},
//     panic::{RefUnwindSafe, UnwindSafe},
//     ptr::NonNull,
//     sync::atomic::AtomicUsize,
// };
//
// struct ArcInner2<T: ?Sized> {
//     strong: AtomicUsize,
//     weak: AtomicUsize,
//     inner: T,
// }
//
// pub struct Arc2<T: ?Sized> {
//     ptr: NonNull<ArcInner2<T>>,
// }
//
// pub struct Weak2<T: ?Sized> {
//     ptr: NonNull<ArcInner2<T>>,
// }
//
// unsafe impl<T: ?Sized + Sync + Send> Send for Arc2<T> {}
//
// unsafe impl<T: ?Sized + Sync + Send> Sync for Arc2<T> {}
//
// impl<T: RefUnwindSafe + ?Sized> UnwindSafe for Arc2<T> {}
//
// impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Arc2<U>> for Arc2<T> {}
//
// impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Arc2<U>> for Arc2<T> {}
//
// unsafe impl<T: ?Sized + Sync + Send> Send for Weak2<T> {}
//
// unsafe impl<T: ?Sized + Sync + Send> Sync for Weak2<T> {}
//
// impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Weak2<U>> for Weak2<T> {}
//
// impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Weak2<U>> for Weak2<T> {}
//
// impl<T: ?Sized> fmt::Debug for Weak2<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "(Weak)")
//     }
// }
