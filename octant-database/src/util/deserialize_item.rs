use serde::Deserializer;

pub trait DeserializeItem<'de> {
    type Value;
    fn deserialize<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::Value, D::Error>;
}
