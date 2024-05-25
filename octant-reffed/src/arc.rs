use crate::Reffed;
use std::{
    marker::Unsize,
    ops::{CoerceUnsized, Deref, DispatchFromDyn},
    sync::Arc,
};

pub struct ArcRef<'a, T: ?Sized>(&'a T);
unsafe impl<'a, T: ?Sized> Send for ArcRef<'a, T> where T: Sync + Send {}
unsafe impl<'a, T: ?Sized> Sync for ArcRef<'a, T> where T: Sync + Send {}

impl<'a, T: ?Sized> Clone for ArcRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> Copy for ArcRef<'a, T> {}

impl<'a, T: ?Sized> ArcRef<'a, T> {
    pub fn arc(&self) -> Arc<T> {
        unsafe {
            Arc::increment_strong_count(self.0);
            Arc::from_raw(self.0)
        }
    }
}

impl<'a, T: ?Sized> Reffed for &'a Arc<T> {
    type ReffedTarget = ArcRef<'a, T>;
    fn reffed(self) -> Self::ReffedTarget {
        ArcRef(&*self)
    }
}

impl<'a, T: ?Sized> Deref for ArcRef<'a, T> {
    type Target = &'a T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'b, T: ?Sized, U: ?Sized> CoerceUnsized<ArcRef<'a, U>> for ArcRef<'b, T>
where
    'b: 'a,
    T: Unsize<U>,
{
}

impl<'a, T: ?Sized, U: ?Sized> DispatchFromDyn<ArcRef<'a, U>> for ArcRef<'a, T> where T: Unsize<U> {}
