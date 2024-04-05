use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};

use serde::{de::DeserializeSeed, Deserializer, Serialize, Serializer};

use crate::{
    de::{DeserializeForest, DeserializeUpdate, DeserializeUpdateSeed},
    deserializer_proxy::DeserializerProxy,
    forest::Forest,
    ser::{SerializeForest, SerializeUpdate, SerializeUpdateAdapter},
    serializer_proxy::SerializerProxy,
    tack::Tack,
    util::option_seed::OptionSeed,
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
    pub fn get_mut<'a>(self: Tack<'a, Self>) -> Tack<'a, T> {
        Tack::new(&mut self.into_inner_unchecked().value)
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
        forest: &mut Forest,
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

pub struct SerializeFieldAdapter<'a, T, SP: SerializerProxy>(
    Option<SerializeUpdateAdapter<'a, T, SP>>,
);

impl<'a, T, SP: SerializerProxy> SerializeFieldAdapter<'a, T, SP> {
    pub fn new(
        field: &'a Field<T>,
        forest: &'a mut Forest,
        ser_forest: &'a mut SerializeForest<SP>,
    ) -> Self {
        SerializeFieldAdapter(
            field
                .modified
                .then_some(SerializeUpdateAdapter::new(field, forest, ser_forest)),
        )
    }
}

impl<'a, T: SerializeUpdate, SP: SerializerProxy> Serialize for SerializeFieldAdapter<'a, T, SP> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

pub struct DeserializeFieldSeed<'a, T, DP: DeserializerProxy>(
    OptionSeed<DeserializeUpdateSeed<'a, Field<T>, DP>>,
);

impl<'a, T, DP: DeserializerProxy> DeserializeFieldSeed<'a, T, DP> {
    pub fn new(field: &'a mut Field<T>, de_forest: &'a mut DeserializeForest<DP>) -> Self {
        DeserializeFieldSeed(OptionSeed::new(DeserializeUpdateSeed::new(
            field, de_forest,
        )))
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>, DP: DeserializerProxy> DeserializeSeed<'de>
    for DeserializeFieldSeed<'a, T, DP>
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize(deserializer)?;
        Ok(())
    }
}
