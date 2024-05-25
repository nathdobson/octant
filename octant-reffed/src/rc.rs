use std::{marker::PhantomData, ops::Deref};
use std::fmt::{Debug, Formatter};
use std::marker::Unsize;
use std::ops::{CoerceUnsized, DispatchFromDyn};
use std::rc::Rc;
use serde::{Serialize, Serializer};
use crate::Arc2;

#[repr(transparent)]
pub struct RcRef<T: ?Sized> {
    phantom: PhantomData<*const ()>,
    inner: T,
}

pub struct Rc2<T: ?Sized> {
    rc: Rc<RcRef<T>>,
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

impl<T: ?Sized + Debug> Debug for Rc2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rc.fmt(f)
    }
}

