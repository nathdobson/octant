use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use serde::de::Error;

use crate::de::proxy::DeserializerProxy;
use crate::de::tree::DeserializeTree;
use crate::de::update::DeserializeUpdate;
use crate::tree::{Tree, TreeId};
use crate::util::unique_arc::UniqueArc;

pub struct DeserializeForest<DP: DeserializerProxy> {
    pub(crate) updaters: HashMap<TreeId, Arc<Tree<dyn DeserializeTree<DP>>>>,
    pub(crate) holes: HashMap<TreeId, UniqueArc<dyn Any + Send + Sync + 'static>>,
    pub(crate) values: HashMap<TreeId, Arc<dyn Any + Send + Sync + 'static>>,
}

impl<DP: DeserializerProxy> DeserializeForest<DP> {
    pub fn new() -> Self {
        DeserializeForest {
            updaters: HashMap::new(),
            holes: HashMap::new(),
            values: HashMap::new(),
        }
    }
    pub fn deserialize_snapshot<'up, 'de: 'up, T: DeserializeUpdate<'de>>(
        &mut self,
        d: DP::DeserializerValue<'up, 'de>,
    ) -> Result<T, DP::Error> {
        T::deserialize_snapshot(self, d)
    }
    pub fn deserialize_update<'up, 'de>(
        &mut self,
        id: TreeId,
        d: DP::DeserializerValue<'up, 'de>,
    ) -> Result<(), DP::Error> {
        let updater = self
            .updaters
            .get(&id)
            .ok_or_else(|| DP::Error::custom("received update for unknown row"))?
            .clone();
        updater
            .try_write()
            .unwrap()
            .deserialize_update_by_proxy(self, d)?;
        Ok(())
    }
}
