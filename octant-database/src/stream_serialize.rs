use serde::{Serialize, Serializer};

pub trait StreamSerialize: Serialize {
    fn build_baseline(&mut self);
    fn build_target(&mut self) -> bool;
    fn serialize_update<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}
