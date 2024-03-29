use serde::{Serialize, Serializer};

use crate::forest::ForestState;

pub trait SerializeUpdate {
    fn begin_stream(&mut self);
    fn begin_update(&mut self) -> bool;
    fn serialize_update<S: Serializer>(
        &self,
        state: &ForestState,
        s: S,
    ) -> Result<S::Ok, S::Error>;
    fn end_update(&mut self);
}

pub struct SerializeUpdateAdapter<'a, T>(&'a T, &'a ForestState);

impl<'a, T> SerializeUpdateAdapter<'a, T> {
    pub fn new(x: &'a T, state: &'a ForestState) -> Self {
        SerializeUpdateAdapter(x, state)
    }
}

impl<'a, T: SerializeUpdate> Serialize for SerializeUpdateAdapter<'a, T> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.0.serialize_update(self.1, s)
    }
}
