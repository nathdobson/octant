use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    marker::{PhantomData, Unsize},
};

use marshal::{
    context::Context,
    de::Deserialize,
    decode::{AnyDecoder, Decoder},
    Deserialize,
    encode::{AnyEncoder, Encoder},
    ser::Serialize, Serialize,
};

use octant_object::class::Class;

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

impl<E: Encoder, T: ?Sized + Class> Serialize<E> for TypedHandle<T> {
    fn serialize<'w, 'en>(&self, e: AnyEncoder<'w, 'en, E>, ctx: Context) -> anyhow::Result<()> {
        <RawHandle as Serialize<E>>::serialize(&self.0, e, ctx)
    }
}

impl<D: Decoder, T: ?Sized + Class> Deserialize<D> for TypedHandle<T> {
    fn deserialize<'p, 'de>(d: AnyDecoder<'p, 'de, D>, ctx: Context) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(TypedHandle(
            <RawHandle as Deserialize<D>>::deserialize(d, ctx)?,
            PhantomData,
        ))
    }
}

impl<T: ?Sized + Class> Debug for TypedHandle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({:?})", &type_name::<T>(), self.0)
    }
}
