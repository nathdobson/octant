use std::cell::Cell;

use serde::{Serialize, Serializer};

use crate::{
    forest::Forest,
    ser::{forest::SerializeForest, proxy::SerializerProxy},
};

pub trait SerializeUpdate {
    fn begin_stream(&mut self);
    fn begin_update(&mut self) -> bool;
    fn serialize_update<S: Serializer, SP: SerializerProxy>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: S,
    ) -> Result<S::Ok, S::Error>;
    fn end_update(&mut self);
}

pub struct SerializeUpdateAdapter<'a, T: ?Sized, SP: SerializerProxy> {
    value: &'a T,
    forest: Cell<Option<&'a mut Forest>>,
    ser_forest: Cell<Option<&'a mut SerializeForest<SP>>>,
}

impl<'a, T: ?Sized, SP: SerializerProxy> SerializeUpdateAdapter<'a, T, SP> {
    pub fn new(
        value: &'a T,
        forest: &'a mut Forest,
        ser_forest: &'a mut SerializeForest<SP>,
    ) -> Self {
        SerializeUpdateAdapter {
            value,
            forest: Cell::new(Some(forest)),
            ser_forest: Cell::new(Some(ser_forest)),
        }
    }
}

impl<'a, T: SerializeUpdate + ?Sized, SP: SerializerProxy> Serialize
    for SerializeUpdateAdapter<'a, T, SP>
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize_update(
            self.forest.take().unwrap(),
            self.ser_forest.take().unwrap(),
            s,
        )
    }
}
