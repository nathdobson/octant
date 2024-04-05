use std::fmt::Formatter;

use serde::{
    de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor},
    Deserializer,
};

use crate::util::{
    identifier_seed::IdentifierSeed,
};
use crate::util::deserialize_pair::{DeserializePair, DeserializePairFirst, DeserializePairSecond};

pub struct PairStructSeed<T> {
    name: &'static str,
    fields: &'static [&'static str; 2],
    inner: T,
}

impl<T> PairStructSeed<T> {
    pub fn new(name: &'static str, fields: &'static [&'static str; 2], inner: T) -> Self {
        PairStructSeed {
            name,
            fields,
            inner,
        }
    }
}

impl<T> PairStructSeed<T> {
    pub fn missing<E: Error>(&self, index: usize) -> E {
        E::custom(format_args!(
            "missing field {} (field index {}) in struct {}",
            self.fields[index], index, self.name
        ))
    }
}

impl<'de, T: DeserializePair<'de>> DeserializeSeed<'de> for PairStructSeed<T> {
    type Value = T::Second;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(self.name, self.fields, self)
    }
}

impl<'de, T: DeserializePair<'de>> Visitor<'de> for PairStructSeed<T> {
    type Value = T::Second;
    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "a struct named {} with fields {} and {}",
            self.name, self.fields[0], self.fields[1]
        )
    }
    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let first = seq
            .next_element_seed(DeserializePairFirst::new(&mut self.inner))?
            .ok_or_else(|| self.missing(0))?;
        let second = seq
            .next_element_seed(DeserializePairSecond::new(&mut self.inner, first))?
            .ok_or_else(|| self.missing(1))?;
        Ok(second)
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let ((), first) = map
            .next_entry_seed(
                IdentifierSeed::new(self.fields[0]),
                DeserializePairFirst::new(&mut self.inner),
            )?
            .ok_or_else(|| self.missing(0))?;
        let ((), second) = map
            .next_entry_seed(
                IdentifierSeed::new(self.fields[1]),
                DeserializePairSecond::new(&mut self.inner, first),
            )?
            .ok_or_else(|| self.missing(1))?;
        Ok(second)
    }
}
