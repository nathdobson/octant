use std::{cell::Cell, sync::Weak};

use serde::{Serialize, Serializer};
use weak_table::WeakValueHashMap;

use crate::{
    forest::Forest,
    tree::{SerializeTree, Tree, TreeId},
    util::{ serializer_proxy::SerializerProxy},
};

pub struct SerializeForest<SP> {
    pub(crate) trees: WeakValueHashMap<TreeId, Weak<Tree<dyn SerializeTree<SP>>>>,
}

impl<SP: SerializerProxy> SerializeForest<SP> {
    pub fn new() -> Self {
        SerializeForest {
            trees: WeakValueHashMap::new(),
        }
    }
}

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

impl<SP: SerializerProxy> SerializeForest<SP> {
    pub fn serialize_snapshot<'up, T: SerializeUpdate>(
        &mut self,
        mut value: &mut T,
        forest: &mut Forest,
        s: SP::SerializerValue<'up>,
    ) -> Result<<SP::SerializerValue<'up> as Serializer>::Ok, SP::Error> {
        value.begin_stream();
        assert!(value.begin_update());
        let result = value.serialize_update(forest, self, s)?;
        value.end_update();
        Ok(result)
    }
    pub fn serialize_update<'up>(
        &mut self,
        id: TreeId,
        forest: &mut Forest,
        s: SP::SerializerValue<'up>,
    ) -> Result<Option<<SP::SerializerValue<'up> as Serializer>::Ok>, SP::Error> {
        if let Some(tree) = self.trees.get(&id) {
            let ref mut value = *tree
                .try_write()
                .expect("lock should succeed because global lock is held");
            if value.tree_begin_update() {
                let result = value.tree_serialize_update(forest, self, s)?;
                value.tree_end_update();
                return Ok(Some(result));
            }
        }
        Ok(None)
    }
    pub fn next_id(&mut self) -> TreeId {
        todo!()
    }
}
