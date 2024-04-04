use serde::{
    de

    , Deserializer,
};

pub trait DeserializerProxy {
    type Error: de::Error;
    type DeserializerValue<'up, 'de: 'up>: Deserializer<'de, Error = Self::Error>;
}

