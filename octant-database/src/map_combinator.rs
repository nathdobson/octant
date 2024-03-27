use crate::seq_combinator::DeserializeItem;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt::Formatter;
use std::marker::PhantomData;

pub struct MapCombinator<T, O>(T, PhantomData<O>);
impl<T, O> MapCombinator<T, O> {
    pub fn new(t: T) -> Self {
        MapCombinator(t, PhantomData)
    }
}

pub trait DeserializeEntry<'de> {
    type Key;
    type Value;
    fn deserialize_key<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::Key, D::Error>;
    fn deserialize_value<D: Deserializer<'de>>(
        &mut self,
        key: Self::Key,
        value: D,
    ) -> Result<Self::Value, D::Error>;
}

impl<'de, T: DeserializeEntry<'de>, O: FromIterator<T::Value>> DeserializeSeed<'de>
    for MapCombinator<T, O>
{
    type Value = O;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de, T: DeserializeEntry<'de>, O: FromIterator<T::Value>> Visitor<'de>
    for MapCombinator<T, O>
{
    type Value = O;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "map")
    }
    fn visit_map<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        MapIterator {
            value: self.0,
            seq,
            de: PhantomData,
        }
        .collect()
    }
}

struct MapIterator<'de, S, T> {
    value: T,
    seq: S,
    de: PhantomData<&'de ()>,
}

impl<'de, S: MapAccess<'de>, T: DeserializeEntry<'de>> Iterator for MapIterator<'de, S, T> {
    type Item = Result<T::Value, S::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let key = self
            .seq
            .next_key_seed(DeserializeKeySeed(&mut self.value))
            .transpose()?;
        let key = match key {
            Ok(x) => x,
            Err(x) => return Some(Err(x)),
        };
        let value = self
            .seq
            .next_value_seed(DeserializeValueSeed(&mut self.value, key));
        Some(value)
    }
}

struct DeserializeKeySeed<'a, T>(&'a mut T);

impl<'de, 'a, T: DeserializeEntry<'de>> DeserializeSeed<'de> for DeserializeKeySeed<'a, T> {
    type Value = T::Key;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_key(deserializer)
    }
}

struct DeserializeValueSeed<'a, T, K>(&'a mut T, K);

impl<'de, 'a, T: DeserializeEntry<'de>> DeserializeSeed<'de>
    for DeserializeValueSeed<'a, T, T::Key>
{
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_value(self.1, deserializer)
    }
}
