#![deny(unused_must_use)]
#![feature(auto_traits)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(arbitrary_self_types)]
#![feature(hash_raw_entry)]
#![feature(map_try_insert)]
#![feature(new_uninit)]
#![feature(never_type)]
#![feature(unboxed_closures)]

use std::{
    mem,
    panic::{catch_unwind, resume_unwind, AssertUnwindSafe},
    sync::{Arc, Weak},
};

use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{de::DeserializeSeed, ser::SerializeSeq, Deserialize, Deserializer, Serializer};
use weak_table::PtrWeakHashSet;

use arc::ArcOrWeak;
use de::{DeserializeContext, DeserializeSnapshotAdapter, DeserializeUpdate};
use dict::Dict;
use option_combinator::OptionCombinator;
use pair_combinator::{DeserializePair, PairStructCombinator};

use crate::row::{Row, RowId};

mod arc;
mod de;
mod dict;
mod map_combinator;
mod option_combinator;
mod pair_combinator;
mod row;
mod seq_combinator;
mod ser;
pub mod tack;
#[cfg(test)]
mod test;

struct RowTableQueue {
    next_id: u64,
    map: PtrWeakHashSet<Weak<Row>>,
}

pub struct RowTableState {
    this: Weak<RowTable>,
    queue: Mutex<RowTableQueue>,
}

pub struct RowTable {
    state: RwLock<RowTableState>,
}

impl RowTableQueue {
    fn try_enqueue(&mut self, row: &Arc<Row>) {
        self.map.insert(row.clone());
    }
}

impl RowTableState {
    pub fn try_enqueue<'b>(&self, row: &Arc<Row>) {
        self.queue.lock().try_enqueue(row);
    }
    pub fn serialize_log<S: Serializer>(&mut self, s: S) -> Result<S::Ok, S::Error> {
        let mut s = s.serialize_seq(None)?;
        for row in mem::replace(&mut self.queue.get_mut().map, PtrWeakHashSet::new()) {
            row.serialize_tree(&mut s, self)?;
        }
        s.end()
    }
    pub fn read<'b>(&'b self, row: &'b Arc<Row>) -> RwLockReadGuard<'b, Dict> {
        row.read()
    }
    pub fn try_read<'b>(&'b self, row: &'b Arc<Row>) -> Option<RwLockReadGuard<'b, Dict>> {
        row.try_read()
    }
    pub fn write<'b>(&'b self, row: &'b Arc<Row>) -> RwLockWriteGuard<'b, Dict> {
        if row.is_written() {
            self.queue.lock().map.insert(row.clone());
        }
        row.write()
    }
    pub fn try_write<'b>(&'b self, row: &'b Arc<Row>) -> Option<RwLockWriteGuard<'b, Dict>> {
        if row.is_written() {
            self.queue.lock().map.insert(row.clone());
        }
        row.try_write()
    }
    pub fn add(&self) -> Arc<Row> {
        let ref mut lock = *self.queue.lock();
        let id = lock.next_id;
        lock.next_id += 1;
        Row::new(RowId::new(id))
    }
}

impl RowTable {
    pub fn new() -> Arc<Self> {
        let result = Arc::new_cyclic(|this| RowTable {
            state: RwLock::new(RowTableState {
                this: this.clone(),
                queue: Mutex::new(RowTableQueue {
                    next_id: 0,
                    map: PtrWeakHashSet::new(),
                }),
            }),
        });
        result
    }
    pub fn read<'a>(self: &'a Arc<Self>) -> RwLockReadGuard<'a, RowTableState> {
        // RowTableReadGuard {
        self.state.read()
        // }
    }
    pub fn write<'a>(self: &'a Arc<Self>) -> RwLockWriteGuard<'a, RowTableState> {
        // RowTableWriteGuard {
        self.state.write()
        // }
    }
}

fn arc_try_new_cyclic<T, E>(
    f: impl for<'a> FnOnce(&'a Weak<T>) -> Result<T, E>,
) -> Result<Arc<T>, E> {
    let mut err = None;
    match catch_unwind(AssertUnwindSafe(|| {
        Arc::new_cyclic(|x| match f(x) {
            Err(e) => {
                err = Some(e);
                panic!("unwinding from failed arc");
            }
            Ok(x) => x,
        })
    })) {
        Err(p) => {
            if let Some(err) = err {
                return Err(err);
            } else {
                resume_unwind(p)
            }
        }
        Ok(x) => Ok(x),
    }
}

impl<'de> DeserializeUpdate<'de> for Arc<Row> {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'a> {
            table: DeserializeContext<'a>,
        }
        impl<'a, 'de> DeserializePair<'de> for V<'a> {
            type First = RowId;
            type Second = Arc<Row>;

            fn deserialize_first<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::First, D::Error> {
                RowId::deserialize(d)
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
                    Ok(x.upgrade_cow().expect("uninitialized row").into_owned())
                } else {
                    let row = arc_try_new_cyclic(|row: &Weak<Row>| {
                        des.entries.insert(key, ArcOrWeak::Weak(row.clone()));
                        let _dict = OptionCombinator::new(DeserializeSnapshotAdapter::<Dict>::new(
                            DeserializeContext { table, des },
                        ))
                        .deserialize(d)?;
                        todo!();
                    })?;
                    des.entries.insert(key, ArcOrWeak::Arc(row.clone()));
                    Ok(row)
                }
            }
        }
        PairStructCombinator {
            name: "Arc",
            fields: &["id", "value"],
            inner: V { table },
        }
        .deserialize(d)
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

impl<'de> DeserializeUpdate<'de> for Weak<Row> {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Ok(Arc::downgrade(&Arc::<Row>::deserialize_snapshot(table, d)?))
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
