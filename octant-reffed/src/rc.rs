use std::{marker::PhantomData, ops::Deref};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::marker::Unsize;
use std::ops::{CoerceUnsized, DispatchFromDyn};
use std::rc::{Rc, Weak};

use serde::{Serialize, Serializer};
use weak_table::traits::{WeakElement, WeakKey};

#[repr(transparent)]
pub struct RcRef<T: ?Sized> {
    phantom: PhantomData<*const ()>,
    inner: T,
}

pub struct Rc2<T: ?Sized> {
    rc: Rc<RcRef<T>>,
}

pub struct Weak2<T: ?Sized> {
    weak: Weak<RcRef<T>>,
}

impl<T: ?Sized> RcRef<T> {
    pub fn rc(&self) -> Rc2<T> {
        unsafe {
            Rc::<Self>::increment_strong_count(self);
            Rc2 {
                rc: Rc::<Self>::from_raw(self),
            }
        }
    }
}

impl<T: ?Sized> Deref for RcRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized> Deref for Rc2<T> {
    type Target = RcRef<T>;
    fn deref(&self) -> &Self::Target {
        &*self.rc
    }
}

impl<T: ?Sized> Rc2<T> {
    pub fn new(x: T) -> Self
    where
        T: Sized,
    {
        Rc2 {
            rc: Rc::new(RcRef {
                phantom: PhantomData,
                inner: x,
            }),
        }
    }
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        Rc2 {
            rc: Rc::from_raw(ptr as *const RcRef<T>),
        }
    }
    pub fn into_raw(this: Self) -> *const T {
        Rc::into_raw(this.rc) as *const T
    }
    pub fn downgrade(&self) -> Weak2<T> {
        Weak2 {
            weak: Rc::downgrade(&self.rc),
        }
    }
}


impl<T: ?Sized> Weak2<T> {
    pub fn upgrade(&self) -> Option<Rc2<T>> {
        Some(Rc2 {
            rc: self.weak.upgrade()?,
        })
    }
}

impl<T: ?Sized + Serialize> Serialize for Rc2<T> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.inner.serialize(s)
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Rc2<U>> for Rc2<T> {}

impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Rc2<U>> for Rc2<T> {}

impl<T: ?Sized> Clone for Rc2<T> {
    fn clone(&self) -> Self {
        Rc2 {
            rc: self.rc.clone(),
        }
    }
}

impl<T: ?Sized> Clone for Weak2<T> {
    fn clone(&self) -> Self {
        Weak2 {
            weak: self.weak.clone(),
        }
    }
}

impl<T: ?Sized + Debug> Debug for Rc2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rc.fmt(f)
    }
}

impl<T: ?Sized> WeakElement for Weak2<T> {
    type Strong = Rc2<T>;

    fn new(view: &Self::Strong) -> Self {
        Rc2::downgrade(view)
    }

    fn view(&self) -> Option<Self::Strong> {
        self.upgrade()
    }

    fn clone(view: &Self::Strong) -> Self::Strong {
        view.clone()
    }
}

impl<T: ?Sized + Eq + Hash> WeakKey for Weak2<T> {
    type Key = T;
    fn with_key<F, R>(view: &Self::Strong, f: F) -> R
        where
            F: FnOnce(&Self::Key) -> R,
    {
        f(&*view)
    }
}
