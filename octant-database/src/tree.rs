use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, OnceLock, Weak},
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{
    de::{DeserializeSeed, Error},
    ser::{SerializeSeq, SerializeStruct},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    arc::{arc_try_new_cyclic, ArcOrWeak},
    de::{DeserializeContext, DeserializeSnapshotSeed, DeserializeUpdate},
    dict::Dict,
    forest::{Forest, ForestState},
    ser::{SerializeUpdate, SerializeUpdateAdapter},
    util::{
        deserialize_pair::DeserializePair, option_seed::OptionSeed,
        pair_struct_seed::PairStructSeed,
    },
};

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct TreeId(u64);

#[derive(Debug)]
struct TreeHeader {
    id: TreeId,
    forest: Weak<Forest>,
}
pub struct Tree {
    header: OnceLock<TreeHeader>,
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
            header: OnceLock::new(),
            state: RwLock::new(Dict::new()),
        })
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
    pub fn serialize_tree<S: SerializeSeq>(
        &self,
        s: &mut S,
        table: &mut ForestState,
    ) -> Result<(), S::Error> {
        let header = self.header.get_or_init(|| TreeHeader {
            id: table.next_id(),
            forest: table.get_arc(),
        });
        let ref mut dict = *self
            .try_write()
            .expect("lock should succeed because global lock is held");
        if dict.begin_update() {
            #[derive(Serialize)]
            struct Entry<A, B> {
                key: A,
                value: B,
            }
            s.serialize_element(&Entry {
                key: header.id,
                value: Some(SerializeUpdateAdapter::new(dict, table)),
            })?;
            dict.end_update();
        }
        Ok(())
    }
    pub(crate) fn mark_written(&self, forest: Weak<Forest>, id: TreeId) {
        self.header.get_or_init(|| TreeHeader { id, forest });
    }
    pub(crate) fn is_written(&self) -> bool {
        self.header.get().is_some()
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("Tree");
        if let Some(header) = self.header.get() {
            f.field("id", &header.id);
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
        let mut new = false;
        let header = self.header.get_or_init(|| {
            new = true;
            TreeHeader {
                id: state.next_id(),
                forest: state.get_arc(),
            }
        });
        let mut s = s.serialize_struct("Arc", 2)?;
        s.serialize_field("id", &header.id)?;
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
        if let Some(mut this) = self.upgrade() {
            s.serialize_some(&SerializeUpdateAdapter::new(&mut this, state))
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
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'a> {
            table: DeserializeContext<'a>,
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
                let table = &*self.table.table;
                let des = &mut *self.table.des;
                if let Some(x) = des.entries.get(&key) {
                    Option::<!>::deserialize(d)?;
                    Ok(x.upgrade_cow()
                        .expect("received update for weak row")
                        .into_owned())
                } else {
                    let row = arc_try_new_cyclic(|row: &Weak<Tree>| {
                        des.entries.insert(key, ArcOrWeak::Weak(row.clone()));
                        let dict = OptionSeed::new(DeserializeSnapshotSeed::<Dict>::new(
                            DeserializeContext { table, des },
                        ))
                        .deserialize(d)?
                        .ok_or_else(|| {
                            D::Error::custom("missing definition for uninitialized row")
                        })?;
                        let tree = Tree {
                            header: OnceLock::new(),
                            state: RwLock::new(dict),
                        };
                        tree.header.get_or_init(|| TreeHeader {
                            id: key,
                            forest: table.get_arc(),
                        });
                        Ok(tree)
                    })?;
                    des.entries.insert(key, ArcOrWeak::Arc(row.clone()));
                    Ok(row)
                }
            }
        }
        PairStructSeed::new("Arc", &["id", "value"], V { table }).deserialize(d)
    }

    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        table: DeserializeContext,
        d: D,
    ) -> Result<(), D::Error> {
        *self = Self::deserialize_snapshot(table, d)?;
        Ok(())
    }
}

impl<'de> DeserializeUpdate<'de> for Weak<Tree> {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Ok(Arc::downgrade(&Arc::<Tree>::deserialize_snapshot(
            table, d,
        )?))
    }

    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        table: DeserializeContext,
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
