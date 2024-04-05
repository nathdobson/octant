use std::sync::Weak;

use serde::Serializer;
use weak_table::WeakValueHashMap;

use crate::forest::Forest;
use crate::ser::proxy::SerializerProxy;
use crate::ser::tree::SerializeTree;
use crate::ser::update::SerializeUpdate;
use crate::tree::{Tree, TreeId};

pub struct SerializeForest<SP> {
    pub(crate) trees: WeakValueHashMap<TreeId, Weak<Tree<dyn SerializeTree<SP>>>>,
}

impl<SP: SerializerProxy> SerializeForest<SP> {
    pub fn new() -> Self {
        SerializeForest {
            trees: WeakValueHashMap::new(),
        }
    }
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
}
