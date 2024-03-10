use std::future::Future;

use serde::de::DeserializeSeed;

pub trait StreamDeserializer<'de, R> {
    fn deserialize_one<D: Send + DeserializeSeed<'de>>(
        &mut self,
        read: &mut R,
        seed: D,
    ) -> impl Send + Future<Output = Result<Option<D::Value>, anyhow::Error>>;
}
//
// pub trait ReadStreamDeserializer<'de, R>: StreamDeserializer<'de> {
//     fn from_write(w: R) -> Self;
//     fn into_write(self) -> R;
// }
