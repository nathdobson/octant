use octant_object::class::Class;
use octant_serde::{DeserializeWith, TypeMap};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    marker::{PhantomData, Unsize},
};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct RawHandle(pub u64);

impl Debug for RawHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl RawHandle {
    pub fn new(index: u64) -> Self {
        RawHandle(index)
    }
    pub fn index(self) -> u64 {
        self.0
    }
}

pub struct TypedHandle<T: ?Sized + Class>(RawHandle, PhantomData<fn() -> T>);

impl<T: ?Sized + Class> Copy for TypedHandle<T> {}

impl<T: ?Sized + Class> Clone for TypedHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + Class> TypedHandle<T> {
    pub fn unsize<U: ?Sized + Class>(self) -> TypedHandle<U>
    where
        T: Unsize<U>,
    {
        TypedHandle(self.0, PhantomData)
    }
    pub fn raw(&self) -> RawHandle {
        self.0
    }
    pub fn new(handle: RawHandle) -> Self {
        TypedHandle(handle, PhantomData)
    }
}

impl<T: ?Sized + Class> Serialize for TypedHandle<T> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(s)
    }
}

impl<'de, T: ?Sized + Class> Deserialize<'de> for TypedHandle<T> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(TypedHandle(RawHandle::deserialize(d)?, PhantomData))
    }
}

impl<T: ?Sized + Class> Debug for TypedHandle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({:?})", &type_name::<T>(), self.0)
    }
}

impl<'de> DeserializeWith<'de> for RawHandle {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}

impl<'de, T: ?Sized + Class> DeserializeWith<'de> for TypedHandle<T> {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}
