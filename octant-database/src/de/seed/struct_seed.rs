use std::fmt::Formatter;
use std::slice;

use serde::de::{DeserializeSeed, Error, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;

use crate::de::seed::identifier_seed::IdentifierSeed;

pub trait StructAccess<'de> {
    type Error: Error;
    fn next_seed<D: DeserializeSeed<'de>>(&mut self, d: D) -> Result<D::Value, Self::Error>;
}

pub trait StructVisitor<'de> {
    type Value;
    fn visit<A: StructAccess<'de>>(self, a: A) -> Result<Self::Value, A::Error>;
}

pub struct StructSeed<S> {
    name: &'static str,
    fields: &'static [&'static str],
    inner: S,
}

impl<'de, S: StructVisitor<'de>> StructSeed<S> {
    pub fn new(name: &'static str, fields: &'static [&'static str], inner: S) -> Self {
        StructSeed {
            name,
            fields,
            inner,
        }
    }
}

impl<'de, S: StructVisitor<'de>> DeserializeSeed<'de> for StructSeed<S> {
    type Value = S::Value;
    fn deserialize<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_struct(self.name, self.fields, self)
    }
}

struct SeqStructAccess<A> {
    name: &'static str,
    fields: slice::Iter<'static, &'static str>,
    seq: A,
}

struct MapStructAccess<A> {
    name: &'static str,
    fields: slice::Iter<'static, &'static str>,
    map: A,
}

impl<'de, S: StructVisitor<'de>> Visitor<'de> for StructSeed<S> {
    type Value = S::Value;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "struct")
    }

    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.inner.visit(SeqStructAccess {
            name: self.name,
            fields: self.fields.iter(),
            seq,
        })
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.inner.visit(MapStructAccess {
            name: self.name,
            fields: self.fields.iter(),
            map,
        })
    }
}

impl<'de, A: SeqAccess<'de>> StructAccess<'de> for SeqStructAccess<A> {
    type Error = A::Error;
    fn next_seed<D: DeserializeSeed<'de>>(&mut self, d: D) -> Result<D::Value, Self::Error> {
        let field = self.fields.next().expect("end of requested fields");
        Ok(self.seq.next_element_seed(d)?.ok_or_else(|| {
            A::Error::custom(format!("Missing field {} in struct {}", field, self.name))
        })?)
    }
}

impl<'de, A: MapAccess<'de>> StructAccess<'de> for MapStructAccess<A> {
    type Error = A::Error;
    fn next_seed<D: DeserializeSeed<'de>>(&mut self, d: D) -> Result<D::Value, Self::Error> {
        let field = self.fields.next().ok_or_else(|| {
            A::Error::custom(format_args!(
                "end of requested fields in struct {}",
                self.name
            ))
        })?;
        Ok(self
            .map
            .next_entry_seed(IdentifierSeed::new(field), d)?
            .ok_or_else(|| {
                A::Error::custom(format!("Missing field {} in struct {}", field, self.name))
            })?
            .1)
    }
}
