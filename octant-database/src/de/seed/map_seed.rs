use std::{fmt::Formatter, marker::PhantomData};

use serde::{
    de::{DeserializeSeed, MapAccess, Visitor},
    Deserializer,
};

pub struct MapSeed<T, O>(T, PhantomData<O>);

impl<T, O> MapSeed<T, O> {
    pub fn new(t: T) -> Self {
        MapSeed(t, PhantomData)
    }
}

impl<'de, T: DeserializeEntry<'de>, O: FromIterator<T::Entry>> DeserializeSeed<'de>
    for MapSeed<T, O>
{
    type Value = O;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de, T: DeserializeEntry<'de>, O: FromIterator<T::Entry>> Visitor<'de> for MapSeed<T, O> {
    type Value = O;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "map")
    }
    fn visit_map<A>(self, seq: A) -> Result<Self::Value, A::Error>
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
    type Item = Result<T::Entry, S::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let key = self
            .seq
            .next_key_seed(DeserializePairFirst::new(&mut self.value))
            .transpose()?;
        let key = match key {
            Ok(x) => x,
            Err(x) => return Some(Err(x)),
        };
        let value = self
            .seq
            .next_value_seed(DeserializePairSecond::new(&mut self.value, key));
        Some(value)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(size) = self.seq.size_hint() {
            (size, Some(size))
        } else {
            (0, None)
        }
    }
}

pub trait DeserializeEntry<'de> {
    type Key;
    type Entry;
    fn deserialize_key<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::Key, D::Error>;
    fn deserialize_value<D: Deserializer<'de>>(
        &mut self,
        first: Self::Key,
        d: D,
    ) -> Result<Self::Entry, D::Error>;
}

struct DeserializePairFirst<'a, T>(&'a mut T);

impl<'a, T> DeserializePairFirst<'a, T> {
    pub fn new(x: &'a mut T) -> Self {
        DeserializePairFirst(x)
    }
}

impl<'de, 'a, T: DeserializeEntry<'de>> DeserializeSeed<'de> for DeserializePairFirst<'a, T> {
    type Value = T::Key;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_key(deserializer)
    }
}

struct DeserializePairSecond<'a, T, F>(&'a mut T, F);

impl<'a, T, F> DeserializePairSecond<'a, T, F> {
    pub fn new(x: &'a mut T, f: F) -> Self {
        DeserializePairSecond(x, f)
    }
}

impl<'de, 'a, T: DeserializeEntry<'de>> DeserializeSeed<'de>
    for DeserializePairSecond<'a, T, T::Key>
{
    type Value = T::Entry;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_value(self.1, deserializer)
    }
}
