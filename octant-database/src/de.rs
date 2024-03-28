use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Weak},
};

use serde::{
    de::{DeserializeSeed, Error},
    Deserialize, Deserializer,
};

use crate::{
    arc::ArcOrWeak,
    forest::{Forest, ForestState},
    tree::{Tree, TreeId},
    util::{
        deserialize_item::DeserializeItem, deserialize_pair::DeserializePair,
        pair_struct_seed::PairStructSeed, seq_seed::SeqSeed,
    },
};

pub struct DeserializeForest {
    pub entries: HashMap<TreeId, ArcOrWeak<Tree>>,
}

pub struct DeserializeContext<'t> {
    pub table: &'t ForestState,
    pub des: &'t mut DeserializeForest,
}

impl<'t> DeserializeContext<'t> {
    pub fn reborrow<'a>(&'a mut self) -> DeserializeContext<'a> {
        DeserializeContext {
            table: &*self.table,
            des: &mut *self.des,
        }
    }
}

pub trait DeserializeUpdate<'de>: Sized {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error>;
    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        table: DeserializeContext,
        d: D,
    ) -> Result<(), D::Error>;
}

impl DeserializeForest {
    pub fn new(forest: Weak<Forest>, root: Arc<Tree>) -> Self {
        let mut result = DeserializeForest {
            entries: HashMap::new(),
        };
        let id = TreeId::new(0);
        root.mark_written(forest, id);
        result.entries.insert(id, ArcOrWeak::Arc(root));
        result
    }
    pub fn deserialize_log<'de, D: Deserializer<'de>>(
        &mut self,
        table: &ForestState,
        d: D,
    ) -> Result<(), D::Error> {
        return SeqSeed::new(LogSeq(DeserializeContext { table, des: self })).deserialize(d);
        struct LogSeq<'a>(DeserializeContext<'a>);
        impl<'a, 'de, 't> DeserializeItem<'de> for LogSeq<'a> {
            type Value = ();

            fn deserialize<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::Value, D::Error> {
                PairStructSeed::new("Entry", &["key", "value"], LogEntry(self.0.reborrow()))
                    .deserialize(d)
            }
        }
        struct LogEntry<'a>(DeserializeContext<'a>);
        impl<'a, 't, 'de> DeserializePair<'de> for LogEntry<'a> {
            type First = TreeId;
            type Second = ();

            fn deserialize_first<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::First, D::Error> {
                TreeId::deserialize(d)
            }

            fn deserialize_second<D: Deserializer<'de>>(
                &mut self,
                first: Self::First,
                d: D,
            ) -> Result<Self::Second, D::Error> {
                let row = self
                    .0
                    .des
                    .entries
                    .get(&first)
                    .ok_or_else(|| {
                        D::Error::custom(format!("received update for unknown row {:?}", first))
                    })?
                    .upgrade_cow()
                    .expect("uninitialized row")
                    .into_owned();
                let table = &*self.0.table;
                let des = &mut *self.0.des;
                self.0
                    .table
                    .write(&row)
                    .deserialize_update(DeserializeContext { table, des }, d)?;
                Ok(())
            }
        }
    }
}

pub struct DeserializeUpdateSeed<'a, T>(&'a mut T, DeserializeContext<'a>);

impl<'a, T> DeserializeUpdateSeed<'a, T> {
    pub fn new(x: &'a mut T, table: DeserializeContext<'a>) -> Self {
        DeserializeUpdateSeed(x, table)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>> DeserializeSeed<'de> for DeserializeUpdateSeed<'a, T> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_update(self.1, deserializer)
    }
}

pub struct DeserializeSnapshotSeed<'a, T>(DeserializeContext<'a>, PhantomData<T>);

impl<'a, T> DeserializeSnapshotSeed<'a, T> {
    pub fn new(table: DeserializeContext<'a>) -> Self {
        DeserializeSnapshotSeed(table, PhantomData)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>> DeserializeSeed<'de> for DeserializeSnapshotSeed<'a, T> {
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_snapshot(self.0, deserializer)
    }
}
