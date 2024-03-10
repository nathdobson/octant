use serde::{Deserialize, Deserializer};

pub trait StreamDeserialize<'de>: Deserialize<'de> {
    fn deserialize_stream<D: Deserializer<'de>>(&mut self, d: D) -> Result<(), D::Error>;
}
