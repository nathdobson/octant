use std::fmt::Formatter;

use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserializer,
};

pub struct IdentifierSeed(&'static str);

impl IdentifierSeed {
    pub fn new(name: &'static str) -> Self {
        IdentifierSeed(name)
    }
}

impl<'de> DeserializeSeed<'de> for IdentifierSeed {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de> Visitor<'de> for IdentifierSeed {
    type Value = ();

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "a field key matching `{}'", self.0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == self.0 {
            Ok(())
        } else {
            Err(E::custom(format_args!(
                "expected a field matching `{}' but found {}",
                self.0, v
            )))
        }
    }
}
