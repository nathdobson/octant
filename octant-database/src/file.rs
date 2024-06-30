use marshal::{context::OwnedContext, decode::DecodeHint};
use marshal_json::{
    decode::full::{JsonDecoder, JsonDecoderBuilder},
    encode::full::{JsonEncoder, JsonEncoderBuilder},
};
use marshal_update::{
    de::DeserializeUpdate,
    ser::{SerializeStream, SerializeUpdate},
};
use octant_error::OctantResult;
use std::{path::Path, sync::Arc, time::Duration};
use tokio::{
    fs,
    fs::{read_dir, File},
    io::AsyncWriteExt,
    sync::RwLock,
};

pub struct DatabaseFile<T: SerializeStream> {
    state: Arc<RwLock<T>>,
    stream: T::Stream,
    file: File,
}

impl<T: SerializeUpdate<JsonEncoder> + DeserializeUpdate<JsonDecoder> + Default> DatabaseFile<T> {
    pub async fn new(dir: &Path) -> OctantResult<(Self, Arc<RwLock<T>>)> {
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
        let mut state: T;
        let next: u64;
        if let Some(last) = indices.iter().max() {
            let path = dir.join(&format!("{}.{}", last, ext));

            let data = fs::read(&path).await?;
            let mut ctx = OwnedContext::new();
            let mut d = JsonDecoderBuilder::new(&data);
            state = T::deserialize(d.build(), ctx.borrow())?;
            while let Some(mut d) = d.build().decode(DecodeHint::Option)?.try_into_option()? {
                state.deserialize_update(d.decode_some()?, ctx.borrow())?;
                d.decode_end()?;
            }
            next = *last + 1;
        } else {
            next = 0;
            state = T::default();
        }
        let mut file = File::create(dir.join(&format!("{}.{}", next, ext))).await?;
        let mut ctx = OwnedContext::new();
        let mut output = JsonEncoderBuilder::new().serialize(&state, ctx.borrow())?;
        output.push('\n');
        file.write_all(output.as_bytes()).await?;
        let stream = state.start_stream(ctx.borrow())?;
        let state = Arc::new(RwLock::new(state));
        Ok((
            DatabaseFile {
                state: state.clone(),
                stream,
                file,
            },
            state,
        ))
    }
    pub async fn serialize(&mut self) -> OctantResult<()> {
        let mut output: String;
        {
            let state = self.state.read().await;
            {
                let mut ctx = OwnedContext::new();
                output = JsonEncoderBuilder::new().with(|e| {
                    let mut e = e.encode_some()?;
                    state.serialize_update(&mut self.stream, e.encode_some()?, ctx.borrow())?;
                    e.end()?;
                    Ok(())
                })?;
            }
        }
        output.push('\n');
        self.file.write_all(output.as_bytes()).await?;
        Ok(())
    }
    pub async fn terminate(&mut self) -> OctantResult<()> {
        let mut ctx = OwnedContext::new();
        let mut output = JsonEncoderBuilder::new().with(|e| e.encode_none())?;
        output.push('\n');
        self.file.write_all(output.as_bytes()).await?;
        Ok(())
    }

    pub async fn serialize_every(mut self, time: Duration) -> OctantResult<!> {
        loop {
            tokio::time::sleep(time).await;
            self.serialize().await?;
        }
    }
}
