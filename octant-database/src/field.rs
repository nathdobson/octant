use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use serde::{Deserializer, Serializer};

use crate::{
    de::{DeserializeForest, DeserializeUpdate},
    forest::ForestState,
    ser::{SerializeForest, SerializeUpdate},
    util::{deserializer_proxy::DeserializerProxy, serializer_proxy::SerializerProxy},
};

pub struct Field<T: ?Sized> {
    modified: bool,
    value: T,
}

impl<T: ?Sized> Field<T> {
    pub fn new(value: T) -> Self
    where
        T: Sized,
    {
        Field {
            value,
            modified: false,
        }
    }
    pub fn modified(&self) -> bool {
        self.modified
    }
}

impl<T: ?Sized + Debug> Debug for Field<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: ?Sized> Deref for Field<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: ?Sized> DerefMut for Field<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.modified = true;
        &mut self.value
    }
}

impl<T: ?Sized + SerializeUpdate> SerializeUpdate for Field<T> {
    fn begin_stream(&mut self) {
        self.modified = true;
    }

    fn begin_update(&mut self) -> bool {
        if self.modified {
            self.modified = self.value.begin_update();
        }
        self.modified
    }

    fn serialize_update<S: Serializer, SP: SerializerProxy>(
        &self,
        forest: &mut ForestState,
        ser_forest: &mut SerializeForest<SP>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        self.value.serialize_update(forest, ser_forest, s)
    }

    fn end_update(&mut self) {
        self.value.end_update();
        self.modified = false;
    }
}

impl<'de, T: DeserializeUpdate<'de>> DeserializeUpdate<'de> for Field<T> {
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error> {
        Ok(Field::new(T::deserialize_snapshot(forest, d)?))
    }

    fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error> {
        self.value.deserialize_update(forest, d)
    }
}
