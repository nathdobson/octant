use crate::{DeserializeContext, DeserializeWith, Error};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub enum RawEncoded {
    Text(String),
}

impl RawEncoded {
    pub fn deserialize_as<'c, 'de, T: Deserialize<'de>>(&'de self) -> Result<T, Error> {
        match self {
            RawEncoded::Text(text) => Ok(T::deserialize(&mut serde_json::Deserializer::new(
                &mut serde_json::de::StrRead::new(&text),
            ))?),
        }
    }
    pub fn deserialize_as_with<'c, 'de, T: DeserializeWith<'de>>(
        &'de self,
        ctx: &'c DeserializeContext,
    ) -> Result<T, Error> {
        match self {
            RawEncoded::Text(text) => Ok(T::deserialize_with(
                ctx,
                &mut serde_json::Deserializer::new(&mut serde_json::de::StrRead::new(&text)),
            )?),
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            RawEncoded::Text(x) => Some(x),
        }
    }
}

impl<'de> DeserializeWith<'de> for RawEncoded {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}

impl Serialize for RawEncoded {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RawEncoded::Text(x) => x.serialize(s),
        }
    }
}

impl<'de> Deserialize<'de> for RawEncoded {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(RawEncoded::Text(String::deserialize(d)?))
    }
}
