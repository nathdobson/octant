use std::fmt::Formatter;

use serde::de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;



pub struct PairCombinator<T>(T);

pub struct PairStructCombinator<T> {
    pub name: &'static str,
    pub fields: &'static [&'static str; 2],
    pub inner: T,
}

impl<T> PairCombinator<T> {
    pub fn new(x: T) -> Self {
        PairCombinator(x)
    }
}

impl<'de, T: DeserializePair<'de>> DeserializeSeed<'de> for PairCombinator<T> {
    type Value = T::Second;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, self)
    }
}

impl<'de, T: DeserializePair<'de>> Visitor<'de> for PairCombinator<T> {
    type Value = T::Second;
    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "pair")
    }
    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        let first = seq
            .next_element_seed(DeserializeFirstSeed(&mut self.0))?
            .ok_or_else(|| A::Error::custom("missing first element"))?;
        let second = seq
            .next_element_seed(DeserializeSecondSeed(&mut self.0, first))?
            .ok_or_else(|| A::Error::custom("missing second element"))?;
        Ok(second)
    }
}

impl<'de, T: DeserializePair<'de>> DeserializeSeed<'de> for PairStructCombinator<T> {
    type Value = T::Second;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        println!("A");
        deserializer.deserialize_struct(self.name, self.fields, self)
    }
}

impl<'de, T: DeserializePair<'de>> Visitor<'de> for PairStructCombinator<T> {
    type Value = T::Second;
    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "pair struct")
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
    {
        if map.next_key::<String>()?.as_deref() != Some(self.fields[0]) {
            return Err(A::Error::custom("first field name is incorrect"));
        }
        let first = map.next_value_seed(DeserializeFirstSeed(&mut self.inner))?;
        if map.next_key::<String>()?.as_deref() != Some(self.fields[1]) {
            return Err(A::Error::custom("second field name is incorrect"));
        }
        let second = map.next_value_seed(DeserializeSecondSeed(&mut self.inner, first))?;
        map.next_entry::<!, !>()?;
        Ok(second)
    }
    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        println!("B");
        let first = seq
            .next_element_seed(DeserializeFirstSeed(&mut self.inner))?
            .ok_or_else(|| A::Error::custom("missing first element"))?;
        let second = seq
            .next_element_seed(DeserializeSecondSeed(&mut self.inner, first))?
            .ok_or_else(|| A::Error::custom("missing second element"))?;
        Ok(second)
    }
}

pub trait DeserializePair<'de> {
    type First;
    type Second;
    fn deserialize_first<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::First, D::Error>;
    fn deserialize_second<D: Deserializer<'de>>(
        &mut self,
        first: Self::First,
        d: D,
    ) -> Result<Self::Second, D::Error>;
}

struct DeserializeFirstSeed<'a, T>(&'a mut T);

impl<'de, 'a, T: DeserializePair<'de>> DeserializeSeed<'de> for DeserializeFirstSeed<'a, T> {
    type Value = T::First;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        self.0.deserialize_first(deserializer)
    }
}

struct DeserializeSecondSeed<'a, T, F>(&'a mut T, F);

impl<'de, 'a, T: DeserializePair<'de>> DeserializeSeed<'de>
for DeserializeSecondSeed<'a, T, T::First>
{
    type Value = T::Second;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        self.0.deserialize_second(self.1, deserializer)
    }
}
