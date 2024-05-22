use std::marker::PhantomData;

use serde::{de::DeserializeSeed, Deserializer};

use crate::de::{forest::DeserializeForest, proxy::DeserializerProxy};

pub trait DeserializeUpdate<'de>: Sized {
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error>;
    fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error>;
}

pub struct DeserializeUpdateSeed<'a, T, DP: DeserializerProxy>(
    &'a mut T,
    &'a mut DeserializeForest<DP>,
);

impl<'a, T, DP: DeserializerProxy> DeserializeUpdateSeed<'a, T, DP> {
    pub fn new(x: &'a mut T, table: &'a mut DeserializeForest<DP>) -> Self {
        DeserializeUpdateSeed(x, table)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>, DP: DeserializerProxy> DeserializeSeed<'de>
    for DeserializeUpdateSeed<'a, T, DP>
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_update(self.1, deserializer)
    }
}

pub struct DeserializeSnapshotSeed<'a, T, DP: DeserializerProxy>(
    &'a mut DeserializeForest<DP>,
    PhantomData<T>,
);

impl<'a, T, DP: DeserializerProxy> DeserializeSnapshotSeed<'a, T, DP> {
    pub fn new(table: &'a mut DeserializeForest<DP>) -> Self {
        DeserializeSnapshotSeed(table, PhantomData)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>, DP: DeserializerProxy> DeserializeSeed<'de>
    for DeserializeSnapshotSeed<'a, T, DP>
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_snapshot(self.0, deserializer)
    }
}
