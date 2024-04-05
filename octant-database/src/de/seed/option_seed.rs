use std::fmt::Formatter;

use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserializer,
};

pub struct OptionSeed<T>(T);

impl<T> OptionSeed<T> {
    pub fn new(x: T) -> Self {
        OptionSeed(x)
    }
}

impl<'de, T> DeserializeSeed<'de> for OptionSeed<T>
where
    T: DeserializeSeed<'de>,
{
    type Value = Option<T::Value>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(self)
    }
}

impl<'de, T> Visitor<'de> for OptionSeed<T>
where
    T: DeserializeSeed<'de>,
{
    type Value = Option<T::Value>;

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(self.0.deserialize(deserializer)?))
    }
    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "option")
    }
}
