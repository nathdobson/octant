use std::{fmt::Formatter, marker::PhantomData};

use crate::util::deserialize_pair::{DeserializePair, DeserializePairFirst, DeserializePairSecond};
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

impl<'de, T: DeserializePair<'de>, O: FromIterator<T::Second>> DeserializeSeed<'de>
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

impl<'de, T: DeserializePair<'de>, O: FromIterator<T::Second>> Visitor<'de>
    for MapSeed<T, O>
{
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

impl<'de, S: MapAccess<'de>, T: DeserializePair<'de>> Iterator for MapIterator<'de, S, T> {
    type Item = Result<T::Second, S::Error>;

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
