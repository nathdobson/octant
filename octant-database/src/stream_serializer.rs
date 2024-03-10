use std::future::Future;

use serde::Serialize;

pub trait StreamSerializer<W> {
    fn serialize_one<T: Serialize + Send>(
        &mut self,
        write: &mut W,
        x: T,
    ) -> impl Send + Future<Output = Result<(), anyhow::Error>>;
}
