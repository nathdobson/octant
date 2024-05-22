use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    marker::{PhantomData, Unsize},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use octant_object::class::Class;


#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct HandleId(pub usize);


pub struct NewTypedHandle<T: ?Sized + Class>(
    HandleId,
    /*Mark as Send and Sync using function pointer instead of value*/ PhantomData<fn() -> T>,
);

impl<T: ?Sized + Class> Copy for NewTypedHandle<T> {}
impl<T: ?Sized + Class> Clone for NewTypedHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + Class> NewTypedHandle<T> {
    pub fn unsize<U: ?Sized + Class>(self) -> NewTypedHandle<U>
    where
        T: Unsize<U>,
    {
        NewTypedHandle(self.0, PhantomData)
    }
    pub fn raw(&self) -> HandleId {
        self.0
    }
    pub fn new(handle: HandleId) -> Self {
        NewTypedHandle(handle, PhantomData)
    }
}

impl<T: ?Sized + Class> Serialize for NewTypedHandle<T> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(s)
    }
}

impl<'de, T: ?Sized + Class> Deserialize<'de> for NewTypedHandle<T> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(NewTypedHandle(HandleId::deserialize(d)?, PhantomData))
    }
}

impl<T: ?Sized + Class> Debug for NewTypedHandle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewTypedHandle")
            .field("type", &type_name::<T>())
            .field("id", &self.0)
            .finish()
    }
}
