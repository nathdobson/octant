use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserializer,
};
use std::fmt::Formatter;

pub struct FieldKeySeed(&'static str);

impl FieldKeySeed {
    pub fn new(name: &'static str) -> Self {
        FieldKeySeed(name)
    }
}

impl<'de> DeserializeSeed<'de> for FieldKeySeed {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(self)
    }
}

impl<'de> Visitor<'de> for FieldKeySeed {
    type Value = ();

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "a field key matching {}", self.0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == self.0 {
            Ok(())
        } else {
            Err(E::custom(format_args!(
                "expected a field matching {} but found {}",
                self.0, v
            )))
        }
    }
}
