use std::{
    fmt,
    fmt::{Debug, Formatter},
    sync::{Arc, OnceLock, Weak},
};

use parking_lot::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{
    de::{DeserializeSeed, Error},
    ser::{SerializeMap, SerializeStruct},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    de::{DeserializeForest, DeserializeSnapshotSeed, DeserializeUpdate},
    dict::Dict,
    forest::{Forest, ForestState},
    ser::{SerializeUpdate, SerializeUpdateAdapter},
    util::{
        arc_or_empty::ArcOrEmpty, deserialize_pair::DeserializePair, option_seed::OptionSeed,
        pair_struct_seed::PairStructSeed, unique_arc::UniqueArc,
    },
};

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct TreeId(u64);

pub struct Tree {
    id: OnceLock<TreeId>,
    forest: OnceLock<Weak<Forest>>,
    written: Once,
    state: RwLock<Dict>,
}

impl TreeId {
    pub fn new(x: u64) -> Self {
        TreeId(x)
    }
}

impl Tree {
    pub fn new() -> Arc<Self> {
        Arc::new(Tree {
            id: OnceLock::new(),
            forest: OnceLock::new(),
            written: Once::new(),
            state: RwLock::new(Dict::new()),
        })
    }
    pub(crate) fn id(&self, forest: &ForestState) -> TreeId {
        *self.id.get_or_init(|| forest.next_id())
    }
    pub(crate) fn forest(&self, forest: &Weak<Forest>) -> &Weak<Forest> {
        self.forest.get_or_init(|| forest.clone())
    }
    pub(crate) fn write(&self) -> RwLockWriteGuard<Dict> {
        self.state.write()
    }
    pub(crate) fn read(&self) -> RwLockReadGuard<Dict> {
        self.state.read()
    }
    pub(crate) fn try_write(&self) -> Option<RwLockWriteGuard<Dict>> {
        self.state.try_write()
    }
    pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<Dict>> {
        self.state.try_read()
    }
    pub fn serialize_tree<S: SerializeMap>(
        &self,
        s: &mut S,
        table: &mut ForestState,
    ) -> Result<(), S::Error> {
        let id = self.id(table);
        let ref mut dict = *self
            .try_write()
            .expect("lock should succeed because global lock is held");
        if dict.begin_update() {
            s.serialize_entry(&id, &SerializeUpdateAdapter::new(dict, table))?;
            dict.end_update();
        }
        Ok(())
    }
    pub fn fmt_weak(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(id) = self.id.get() {
            Debug::fmt(id, f)?;
        }
        Ok(())
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Tree");
        if let Some(id) = self.id.get() {
            f.field("id", &id);
        }
        if let Some(state) = self.state.try_read() {
            f.field("state", &*state);
        }
        f.finish()
    }
}

impl SerializeUpdate for Arc<Tree> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer>(
        &self,
        state: &ForestState,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        let id = self.id(state);
        let mut new = false;
        self.written.call_once(|| new = true);
        let mut s = s.serialize_struct("Arc", 2)?;
        s.serialize_field("id", &id)?;
        if new {
            s.serialize_field(
                "value",
                &Some(SerializeUpdateAdapter::new(
                    &*state.try_read(self).expect("global lock should be held"),
                    state,
                )),
            )?;
        } else {
            s.serialize_field("value", &Option::<()>::None)?;
        }
        s.end()
    }

    fn end_update(&mut self) {
        todo!()
    }
}

impl SerializeUpdate for Weak<Tree> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer>(
        &self,
        state: &ForestState,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(this) = self.upgrade() {
            s.serialize_some(&this.id(state))
        } else {
            s.serialize_none()
        }
    }

    fn end_update(&mut self) {
        todo!()
    }
}

impl<'de> DeserializeUpdate<'de> for Arc<Tree> {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        forest: &mut DeserializeForest,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'a> {
            forest: &'a mut DeserializeForest,
        }
        impl<'a, 'de> DeserializePair<'de> for V<'a> {
            type First = TreeId;
            type Second = Arc<Tree>;

            fn deserialize_first<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::First, D::Error> {
                TreeId::deserialize(d)
            }

            fn deserialize_second<D: Deserializer<'de>>(
                &mut self,
                key: Self::First,
                d: D,
            ) -> Result<Self::Second, D::Error> {
                match self
                    .forest
                    .entries
                    .entry(key)
                    .or_insert_with(|| ArcOrEmpty::Empty(UniqueArc::new_uninit()))
                {
                    ArcOrEmpty::Arc(v) => {
                        Option::<!>::deserialize(d)?;
                        Ok(v.clone())
                    }
                    ArcOrEmpty::Empty(_) => {
                        let dict =
                            OptionSeed::new(DeserializeSnapshotSeed::<Dict>::new(self.forest))
                                .deserialize(d)?
                                .ok_or_else(|| {
                                    D::Error::custom("missing definition for uninitialized row")
                                })?;
                        match self.forest.entries.remove(&key).unwrap() {
                            ArcOrEmpty::Arc(_) => unreachable!(),
                            ArcOrEmpty::Empty(v) => {
                                let v = v.init(Tree {
                                    id: OnceLock::from(key),
                                    forest: OnceLock::new(),
                                    written: Once::new(),
                                    state: RwLock::new(dict),
                                });
                                self.forest.entries.insert(key, ArcOrEmpty::Arc(v.clone()));
                                Ok(v)
                            }
                        }
                    }
                }
            }
        }
        PairStructSeed::new("Arc", &["id", "value"], V { forest }).deserialize(d)
    }

    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        table: &mut DeserializeForest,
        d: D,
    ) -> Result<(), D::Error> {
        *self = Self::deserialize_snapshot(table, d)?;
        Ok(())
    }
}

impl<'de> DeserializeUpdate<'de> for Weak<Tree> {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        forest: &mut DeserializeForest,
        d: D,
    ) -> Result<Self, D::Error> {
        let key = TreeId::deserialize(d)?;
        Ok(forest
            .entries
            .entry(key)
            .or_insert_with(|| ArcOrEmpty::Empty(UniqueArc::new_uninit()))
            .weak())
    }

    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        table: &mut DeserializeForest,
        d: D,
    ) -> Result<(), D::Error> {
        *self = Self::deserialize_snapshot(table, d)?;
        Ok(())
    }
}

impl Debug for TreeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}
