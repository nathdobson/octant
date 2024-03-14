use std::fmt::Formatter;

use serde::de::{DeserializeSeed, Error, Visitor};
use serde::{Deserializer, Serialize, Serializer};

use crate::stream_deserialize::StreamDeserialize;
use crate::stream_serialize::StreamSerialize;

pub struct UpdateSeed<'a, T> {
    value: &'a mut T,
}

impl<'a, T> UpdateSeed<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
        UpdateSeed { value }
    }
}

impl<'a, 'de, T: StreamDeserialize<'de>> DeserializeSeed<'de> for UpdateSeed<'a, T> {
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        self.value.deserialize_stream(deserializer)?;
        Ok(())
    }
}

pub struct OptionSeed<S> {
    inner: S,
}

impl<S> OptionSeed<S> {
    pub fn new(inner: S) -> Self {
        OptionSeed { inner }
    }
}

impl<'de, S: DeserializeSeed<'de>> DeserializeSeed<'de> for OptionSeed<S> {
    type Value = Option<S::Value>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V<S>(S);
        impl<'de, S: DeserializeSeed<'de>> Visitor<'de> for V<S> {
            type Value = Option<S::Value>;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "option")
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(None)
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Some(self.0.deserialize(deserializer)?))
            }
        }
        deserializer.deserialize_option(V(self.inner))
    }
}

pub struct SerializeUpdate<'a, T>(&'a T);
impl<'a, T> SerializeUpdate<'a, T> {
    pub fn new(x: &'a T) -> Self {
        SerializeUpdate(x)
    }
}
impl<'a, T: StreamSerialize> Serialize for SerializeUpdate<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize_update(serializer)
    }
}
