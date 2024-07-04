use std::ops::{Deref, DerefMut};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::dirty::DirtyMarker;

pub struct DbLock<T: ?Sized>(RwLock<DirtyMarker<T>>);

pub struct DbLockReadGuard<'a, T: ?Sized>(RwLockReadGuard<'a, DirtyMarker<T>>);

pub struct DbLockWriteGuard<'a, T: ?Sized>(RwLockWriteGuard<'a, DirtyMarker<T>>);

impl<T: ?Sized> DbLock<T> {
    pub fn new(inner: T) -> Self
    where
        T: Sized,
    {
        DbLock(RwLock::new(DirtyMarker::new(inner)))
    }
    pub async fn read(&self) -> DbLockReadGuard<T> {
        DbLockReadGuard(self.0.read().await)
    }
    pub async fn write(&self) -> DbLockWriteGuard<T> {
        DbLockWriteGuard(self.0.write().await)
    }
}

impl<'a, T> Deref for DbLockReadGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, T> Deref for DbLockWriteGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, T> DerefMut for DbLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<'a, T> DbLockReadGuard<'a, T> {
    pub fn check_dirty(&self) -> bool {
        self.0.check_dirty()
    }
}
