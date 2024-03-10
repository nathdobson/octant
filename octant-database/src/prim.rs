use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::stream_deserialize::StreamDeserialize;
use crate::stream_serialize::StreamSerialize;
use crate::tack::Tack;

pub struct Prim<T>(pub T);

impl<T> Prim<T> {
    pub fn get_prim<'a>(self: Tack<'a, Self>) -> &'a mut T {
        &mut self.into_inner_unchecked().0
    }
}

impl<T: Serialize> Serialize for Prim<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<T: Serialize> StreamSerialize for Prim<T> {
    fn build_baseline(&mut self) {}
    fn build_target(&mut self) -> bool {
        true
    }
    fn serialize_update<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Prim<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Prim(T::deserialize(deserializer)?))
    }
}
impl<'de, T: Deserialize<'de>> StreamDeserialize<'de> for Prim<T> {
    fn deserialize_stream<D: Deserializer<'de>>(&mut self, d: D) -> Result<(), D::Error> {
        self.0 = T::deserialize(d)?;
        Ok(())
    }
}

impl<T> Deref for Prim<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
