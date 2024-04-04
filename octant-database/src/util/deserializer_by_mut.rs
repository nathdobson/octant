use std::marker::PhantomData;

use serde::de::{Error, Visitor};
use serde::Deserializer;

pub struct DeserializerByMut<T, E>(T, PhantomData<E>);

impl<T, E> DeserializerByMut<T, E> {
    pub fn new(x: T) -> Self {
        DeserializerByMut(x, PhantomData)
    }
}

impl<'de, T, E: Error> Deserializer<'de> for DeserializerByMut<T, E>
    where
            for<'a> &'a mut T: Deserializer<'de, Error = E>,
{
    type Error = E;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_any(visitor)
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_i64(visitor)
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_u16(visitor)
    }

    fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_u64(visitor)
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_f32(visitor)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_f64(visitor)
    }

    fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_char(visitor)
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_str(visitor)
    }

    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_string(visitor)
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_option(visitor)
    }

    fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_unit(visitor)
    }

    fn deserialize_unit_struct<V>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_seq(visitor)
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        mut self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_map(visitor)
    }

    fn deserialize_struct<V>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V>(
        mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
    {
        self.0.deserialize_ignored_any(visitor)
    }
}
