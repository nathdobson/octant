use std::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct DirtyMarker<T: ?Sized> {
    dirty: AtomicBool,
    inner: T,
}

impl<T: ?Sized> DirtyMarker<T> {
    pub fn new(inner: T) -> Self
    where
        T: Sized,
    {
        DirtyMarker {
            dirty: AtomicBool::new(false),
            inner,
        }
    }
    pub fn check_dirty(&self) -> bool {
        self.dirty.swap(false, Ordering::SeqCst)
    }
}

impl<T: ?Sized> Deref for DirtyMarker<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized> DerefMut for DirtyMarker<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        *self.dirty.get_mut() = true;
        &mut self.inner
    }
}
