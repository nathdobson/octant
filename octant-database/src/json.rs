use serde_json::{de::SliceRead, ser::PrettyFormatter};

use crate::{de::proxy::DeserializerProxy, ser::proxy::SerializerProxy};

pub struct JsonProxy;

impl DeserializerProxy for JsonProxy {
    type Error = serde_json::Error;
    type DeserializerValue<'up, 'de: 'up> = &'up mut serde_json::Deserializer<SliceRead<'de>>;
}

impl SerializerProxy for JsonProxy {
    type Error = serde_json::Error;
    type SerializerValue<'up> =
        &'up mut serde_json::Serializer<&'up mut Vec<u8>, PrettyFormatter<'up>>;
}
