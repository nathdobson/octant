use serde::de::{DeserializeSeed, Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;

pub struct OptionCombinator<T>(T);

impl<T> OptionCombinator<T> {
    pub fn new(x: T) -> Self {
        OptionCombinator(x)
    }
}
impl<'de, T> DeserializeSeed<'de> for OptionCombinator<T>
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

impl<'de, T> Visitor<'de> for OptionCombinator<T>
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
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}
