use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use serde::{
    de::{DeserializeSeed, Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{
    arc::ArcOrWeak,
    map_combinator::DeserializeEntry,
    pair_combinator::{DeserializePair, PairStructCombinator},
    row::{Row, RowId},
    RowTableState,
    seq_combinator::{DeserializeItem, SeqCombinator},
};

pub struct DeserializeTable {
    pub entries: HashMap<RowId, ArcOrWeak<Row>>,
}

pub struct DeserializeContext<'t> {
    pub table: &'t RowTableState,
    pub des: &'t mut DeserializeTable,
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

impl DeserializeTable {
    pub fn new(table: &RowTableState, root: Arc<Row>) -> Self {
        let mut result = DeserializeTable {
            entries: HashMap::new(),
        };
        result.entries.insert(root.id(), ArcOrWeak::Arc(root));
        result
    }
    pub fn deserialize_log<'de, D: Deserializer<'de>>(
        &mut self,
        table: &RowTableState,
        d: D,
    ) -> Result<(), D::Error> {
        return SeqCombinator::new(LogSeq(DeserializeContext { table, des: self })).deserialize(d);
        struct LogSeq<'a>(DeserializeContext<'a>);
        impl<'a, 'de, 't> DeserializeItem<'de> for LogSeq<'a> {
            type Value = ();

            fn deserialize<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::Value, D::Error> {
                PairStructCombinator {
                    name: "Entry",
                    fields: &["key", "value"],
                    inner: LogEntry(self.0.reborrow()),
                }
                    .deserialize(d)
            }
        }
        struct LogEntry<'a>(DeserializeContext<'a>);
        impl<'a, 't, 'de> DeserializePair<'de> for LogEntry<'a> {
            type First = RowId;
            type Second = ();

            fn deserialize_first<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::First, D::Error> {
                RowId::deserialize(d)
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
        // return MapCombinator::new(LogMap(self)).deserialize(d);
        //
        // struct LogMap<'a, 't>(&'a mut DeserializeTable<'t>);
        // impl<'a, 'de, 't> DeserializeEntry<'de> for LogMap<'a, 't> {
        //     type Key = RowId;
        //     type Value = ();
        //
        //     fn deserialize_key<D: Deserializer<'de>>(
        //         &mut self,
        //         d: D,
        //     ) -> Result<Self::Key, D::Error> {
        //         RowId::deserialize(d)
        //     }
        //
        //     fn deserialize_value<D: Deserializer<'de>>(
        //         &mut self,
        //         key: Self::Key,
        //         value: D,
        //     ) -> Result<Self::Value, D::Error> {
        //         let row = self
        //             .0
        //             .entries
        //             .entry(key)
        //             .or_insert_with(|| self.0.table.add());
        //         self.0
        //             .table
        //             .write(row)
        //             .deserialize_update(&self.0.table, value)?;
        //         Ok(())
        //     }
        // }
    }
}

pub struct DeserializeUpdateAdapter<'a, T>(&'a mut T, DeserializeContext<'a>);

impl<'a, T> DeserializeUpdateAdapter<'a, T> {
    pub fn new(x: &'a mut T, table: DeserializeContext<'a>) -> Self {
        DeserializeUpdateAdapter(x, table)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>> DeserializeSeed<'de> for DeserializeUpdateAdapter<'a, T> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        self.0.deserialize_update(self.1, deserializer)
    }
}

pub struct DeserializeSnapshotAdapter<'a, T>(DeserializeContext<'a>, PhantomData<T>);

impl<'a, T> DeserializeSnapshotAdapter<'a, T> {
    pub fn new(table: DeserializeContext<'a>) -> Self {
        DeserializeSnapshotAdapter(table, PhantomData)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>> DeserializeSeed<'de>
for DeserializeSnapshotAdapter<'a, T>
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {
        T::deserialize_snapshot(self.0, deserializer)
    }
}
