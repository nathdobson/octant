use std::future::Future;
use std::io::Cursor;

use serde::de::DeserializeSeed;
use serde::Serialize;
use serde_json::de::IoRead;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

use crate::stream_deserializer::StreamDeserializer;
use crate::stream_serializer::StreamSerializer;

pub struct JsonFormat;
impl<'de, R: Send + Unpin + AsyncBufRead> StreamDeserializer<'de, R> for JsonFormat {
    fn deserialize_one<D: Send + DeserializeSeed<'de>>(
        &mut self,
        read: &mut R,
        seed: D,
    ) -> impl Send + Future<Output = Result<Option<D::Value>, anyhow::Error>> {
        async move {
            let mut buf = String::new();
            if 0 == read.read_line(&mut buf).await? {
                return Ok(None);
            }
            let value = seed.deserialize(&mut serde_json::Deserializer::new(IoRead::new(
                Cursor::new(&buf.as_bytes()),
            )))?;
            Ok(Some(value))
        }
    }
}

impl<W: Send + Unpin + AsyncWrite> StreamSerializer<W> for JsonFormat {
    fn serialize_one<T: Serialize + Send>(
        &mut self,
        write: &mut W,
        value: T,
    ) -> impl Send + Future<Output = Result<(), anyhow::Error>> {
        async move {
            let mut buf = vec![];
            value.serialize(&mut serde_json::Serializer::new(Cursor::new(&mut buf)))?;
            buf.push(b'\n');
            write.write_all(&buf).await?;
            Ok(())
        }
    }
}
