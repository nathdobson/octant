use std::ops::Deref;
use std::path::Path;

use tokio::fs::{File, OpenOptions};
use tokio::io::BufReader;

use crate::database::{DatabaseReader, DatabaseWriter};
use crate::stream_deserialize::StreamDeserialize;
use crate::stream_deserializer::StreamDeserializer;
use crate::stream_serialize::StreamSerialize;
use crate::stream_serializer::StreamSerializer;
use crate::tack::Tack;

pub struct FileDatabase<S, T> {
    database: DatabaseWriter<S, File, T>,
}

impl<'de, S: StreamSerializer<File>, T: Sync + Send + StreamSerialize + StreamDeserialize<'de>>
    FileDatabase<S, T>
{
    pub async fn new<D: StreamDeserializer<'de, BufReader<File>>>(
        path: &Path,
        ser: S,
        des: D,
        or_init: impl FnOnce() -> T,
    ) -> anyhow::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .await?;
        let mut reader = DatabaseReader::new(des, BufReader::new(file));
        let value = reader.read::<T>().await?;
        let file = reader.into_inner().into_inner();
        if let Some(value) = value {
            Ok(FileDatabase {
                database: DatabaseWriter::new_append(ser, file, value),
            })
        } else {
            let value = or_init();
            Ok(FileDatabase {
                database: DatabaseWriter::new(ser, file, value).await?,
            })
        }
    }
    pub async fn write_update(&mut self) -> anyhow::Result<()> {
        self.database.write_update().await?;
        Ok(())
    }
    pub fn get_mut(&mut self) -> Tack<T> {
        self.database.get_mut()
    }
}

impl<S, T> Deref for FileDatabase<S, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.database
    }
}
