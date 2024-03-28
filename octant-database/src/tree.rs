use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Weak},
};

use parking_lot::{Once, OnceState, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{
    de::{DeserializeSeed, Error},
    ser::{SerializeSeq, SerializeStruct},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    arc::{arc_try_new_cyclic, ArcOrWeak},
    de::{DeserializeContext, DeserializeSnapshotSeed, DeserializeUpdate},
    dict::Dict,
    forest::ForestState,
    ser::{SerializeUpdate, SerializeUpdateAdapter},
    util::{
        deserialize_pair::DeserializePair, option_seed::OptionSeed,
        pair_struct_seed::PairStructSeed,
    },
};

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TreeId(u64);

pub struct Tree {
    id: TreeId,
    written: Once,
    state: RwLock<Dict>,
}

impl TreeId {
    pub fn new(x: u64) -> Self {
        TreeId(x)
    }
}

impl Tree {
    pub fn new(id: TreeId) -> Self {
        Tree {
            id,
            written: Once::new(),
            state: RwLock::new(Dict::new()),
        }
    }
    pub fn id(&self) -> TreeId {
        self.id
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
        self.written.call_once(|| ());
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
                key: self.id(),
                value: Some(SerializeUpdateAdapter::new(dict, table)),
            })?;
            dict.end_update();
        }
        Ok(())
    }
    pub fn is_written(&self) -> bool {
        match self.written.state() {
            OnceState::New => false,
            OnceState::Poisoned => panic!(),
            OnceState::InProgress => panic!(),
            OnceState::Done => true,
        }
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Row")
            .field("id", &self.id)
            .field("state", &self.state)
            .finish()
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
        let mut s = s.serialize_struct("Arc", 2)?;
        s.serialize_field("id", &self.id())?;
        let mut new = false;
        self.written.call_once(|| new = true);
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
                        Ok(Tree::new(key))
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
