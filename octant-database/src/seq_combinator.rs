use std::fmt::Formatter;
use std::marker::PhantomData;

use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::Deserializer;

pub struct SeqCombinator<T, O>(T, PhantomData<O>);

pub trait DeserializeItem<'de> {
    type Value;
    fn deserialize<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::Value, D::Error>;
}

impl<T, O> SeqCombinator<T, O> {
    pub fn new(t: T) -> Self {
        SeqCombinator(t, PhantomData)
    }
}

impl<'de, T: DeserializeItem<'de>, O: FromIterator<T::Value>> DeserializeSeed<'de>
for SeqCombinator<T, O>
{
    type Value = O;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

impl<'de, T: DeserializeItem<'de>, O: FromIterator<T::Value>> Visitor<'de> for SeqCombinator<T, O> {
    type Value = O;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "seq")
    }
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        SeqIterator {
            value: self.0,
            seq,
            de: PhantomData,
        }
            .collect()
    }
}


struct SeqIterator<'de, S, T> {
    value: T,
    seq: S,
    de: PhantomData<&'de ()>,
}

impl<'de, S: SeqAccess<'de>, T: DeserializeItem<'de>> Iterator for SeqIterator<'de, S, T> {
    type Item = Result<T::Value, S::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.seq
            .next_element_seed(DeserializeItemSeed(&mut self.value))
            .transpose()
    }
}

struct DeserializeItemSeed<'a, T>(&'a mut T);

impl<'de, 'a, T: DeserializeItem<'de>> DeserializeSeed<'de> for DeserializeItemSeed<'a, T> {
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        self.0.deserialize(deserializer)
    }
}
