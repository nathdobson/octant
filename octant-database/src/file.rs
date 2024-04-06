use std::{io, path::Path, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::{de::SliceRead, ser::PrettyFormatter};
use tokio::{
    fs,
    fs::{File, read_dir},
    io::AsyncWriteExt,
    sync::RwLock,
};

use crate::de::forest::DeserializeForest;
use crate::de::proxy::DeserializerProxy;
use crate::de::update::DeserializeUpdate;
use crate::forest::Forest;
use crate::ser::forest::SerializeForest;
use crate::ser::proxy::SerializerProxy;
use crate::ser::update::SerializeUpdate;
use crate::tree::{Tree, TreeId};

pub struct Database {
    forest: Arc<RwLock<Forest>>,
    file: File,
    ser_forest: SerializeForest<JsonProxy>,
}

struct JsonProxy;

impl DeserializerProxy for JsonProxy {
    type Error = serde_json::Error;
    type DeserializerValue<'up, 'de: 'up> = &'up mut serde_json::Deserializer<SliceRead<'de>>;
}

impl SerializerProxy for JsonProxy {
    type Error = serde_json::Error;
    type SerializerValue<'up> =
        &'up mut serde_json::Serializer<&'up mut Vec<u8>, PrettyFormatter<'up>>;
}

impl Database {
    fn serializer(
        vec: &mut Vec<u8>,
    ) -> serde_json::Serializer<&mut Vec<u8>, PrettyFormatter> {
        serde_json::Serializer::with_formatter(vec, PrettyFormatter::new())
    }
    pub async fn new<
        T: 'static + Send + Sync + for<'de> DeserializeUpdate<'de> + SerializeUpdate,
    >(
        dir: &Path,
        def: impl FnOnce() -> Arc<Tree<T>>,
    ) -> io::Result<(Self, Arc<Tree<T>>)> {
        let ext = "json";
        let mut entries = read_dir(dir).await?;
        let mut indices: Vec<u64> = vec![];
        while let Some(next) = entries.next_entry().await? {
            let path = next.path();
            if let Some(index) = try {
                (path.extension()? == ext).then_some(())?;
                path.file_stem()?.to_str()?.parse::<u64>().ok()?
            } {
                indices.push(index);
            }
        }
        let mut root: Arc<Tree<T>>;
        let next: u64;
        if let Some(last) = indices.iter().max() {
            let path = dir.join(&format!("{}.{}", last, ext));
            let mut de_forest = DeserializeForest::<JsonProxy>::new();
            let data = fs::read(&path).await?;
            let mut d = serde_json::Deserializer::from_slice(&data);
            root = de_forest.deserialize_snapshot(&mut d)?;
            while let Ok(next) = u64::deserialize(&mut d) {
                for _ in 0..next {
                    let id = TreeId::deserialize(&mut d)?;
                    de_forest.deserialize_update(id, &mut d)?;
                }
            }
            next = *last + 1;
        } else {
            next = 0;
            root = def();
        }
        let mut file = File::create(dir.join(&format!("{}.{}", next, ext))).await?;
        let mut forest = Forest::new();
        let mut ser_forest = SerializeForest::new();
        let mut output = vec![];
        ser_forest.serialize_snapshot(
            &mut root,
            &mut forest,
            &mut Self::serializer(&mut output),
        )?;
        output.push(b'\n');
        file.write_all(&output).await?;
        Ok((
            Database {
                forest: Arc::new(RwLock::new(forest)),
                file,
                ser_forest,
            },
            root,
        ))
    }
    pub fn forest(&self) -> &Arc<RwLock<Forest>> {
        &self.forest
    }
    pub async fn serialize(&mut self) -> io::Result<()> {
        let mut output = vec![];
        {
            let ref mut forest = *self.forest.write().await;
            let queue = forest.take_queue();
            if !queue.is_empty() {
                queue.len().serialize(&mut Self::serializer(&mut output))?;
                output.push(b'\n');
                for id in queue {
                    id.serialize(&mut Self::serializer(&mut output))?;
                    output.push(b'\n');
                    self.ser_forest.serialize_update(
                        id,
                        forest,
                        &mut Self::serializer(&mut output),
                    )?;
                    output.push(b'\n');
                }
            }
        }
        self.file.write_all(&output).await?;
        Ok(())
    }
    pub async fn serialize_every(mut self, time: Duration) -> io::Result<!> {
        loop {
            tokio::time::sleep(time).await;
            self.serialize().await?;
        }
    }
}