use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    de::{DeserializeForest, DeserializeUpdate},
    forest::Forest,
    ser::{SerializeForest, SerializeUpdate},
    util::{deserializer_proxy::DeserializerProxy, serializer_proxy::SerializerProxy},
};
use crate::util::tack::Untack;

pub struct Prim<T: ?Sized>(T);

impl<T: ?Sized> Prim<T> {
    pub fn new(value: T) -> Self
    where
        T: Sized,
    {
        Prim(value)
    }
}

impl<T: ?Sized + Serialize> SerializeUpdate for Prim<T> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer, SP: SerializerProxy>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }

    fn end_update(&mut self) {}
}

impl<'de, T: Deserialize<'de>> DeserializeUpdate<'de> for Prim<T> {
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error> {
        Ok(Prim::new(T::deserialize(d)?))
    }

    fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error> {
        self.0 = T::deserialize(d)?;
        Ok(())
    }
}

impl<T: ?Sized + Debug> Debug for Prim<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ?Sized> Deref for Prim<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for Prim<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ?Sized> Untack for Prim<T> {}
