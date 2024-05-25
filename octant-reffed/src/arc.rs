use serde::{Serialize, Serializer};
use std::{
    fmt::{Debug, Formatter},
    hash::Hash,
    marker::{PhantomData, Unsize},
    ops::{CoerceUnsized, Deref, DispatchFromDyn},
    sync::{Arc, Weak},
};
use weak_table::traits::{WeakElement, WeakKey};

#[repr(transparent)]
pub struct ArcRef<T: ?Sized> {
    phantom: PhantomData<*const ()>,
    inner: T,
}

unsafe impl<T: ?Sized> Sync for ArcRef<T> where T: Sync + Send {}
unsafe impl<T: ?Sized> Send for ArcRef<T> where T: Sync + Send {}

pub struct Arc2<T: ?Sized> {
    arc: Arc<ArcRef<T>>,
}

pub struct Weak2<T: ?Sized> {
    weak: Weak<ArcRef<T>>,
}

impl<T: ?Sized> ArcRef<T> {
    pub fn arc(&self) -> Arc2<T> {
        unsafe {
            Arc::<Self>::increment_strong_count(self);
            Arc2 {
                arc: Arc::<Self>::from_raw(self),
            }
        }
    }
}

impl<T: ?Sized> Deref for ArcRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized> Deref for Arc2<T> {
    type Target = ArcRef<T>;
    fn deref(&self) -> &Self::Target {
        &*self.arc
    }
}

impl<T: ?Sized> Arc2<T> {
    pub fn new(x: T) -> Self
    where
        T: Sized,
    {
        Arc2 {
            arc: Arc::new(ArcRef {
                phantom: PhantomData,
                inner: x,
            }),
        }
    }
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        Arc2 {
            arc: Arc::from_raw(ptr as *const ArcRef<T>),
        }
    }
    pub fn into_raw(this: Self) -> *const T {
        Arc::into_raw(this.arc) as *const T
    }
    pub fn downgrade(&self) -> Weak2<T> {
        Weak2 {
            weak: Arc::downgrade(&self.arc),
        }
    }
}

impl<T: ?Sized> Weak2<T> {
    pub fn upgrade(&self) -> Option<Arc2<T>> {
        Some(Arc2 {
            arc: self.weak.upgrade()?,
        })
    }
}

impl<T: ?Sized + Serialize> Serialize for Arc2<T> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(s)
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Arc2<U>> for Arc2<T> {}

impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Arc2<U>> for Arc2<T> {}

impl<T: ?Sized> Clone for Arc2<T> {
    fn clone(&self) -> Self {
        Arc2 {
            arc: self.arc.clone(),
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

impl<T: ?Sized + Debug> Debug for Arc2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.arc.fmt(f)
    }
}

impl<T: ?Sized> WeakElement for Weak2<T> {
    type Strong = Arc2<T>;

    fn new(view: &Self::Strong) -> Self {
        Arc2::downgrade(view)
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
        todo!()
    }
}
