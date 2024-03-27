use std::cell::Cell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once, OnceLock, Weak};

use crate::object::arc::ArcOrWeak;
use crate::object::de::{
    DeserializeContext, DeserializeSnapshotAdapter, DeserializeTable, DeserializeUpdate,
};
use crate::object::dict::Dict;
use crate::object::option_combinator::OptionCombinator;
use crate::object::pair_combinator::{DeserializePair, PairCombinator, PairStructCombinator};
use crate::object::ser::{SerializeUpdate, SerializeUpdateAdapter};
use parking_lot::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::de::{DeserializeSeed, Error, Visitor};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use weak_table::{PtrWeakHashSet, PtrWeakKeyHashMap};

mod dict;
// mod hash;

mod arc;
mod de;
mod map_combinator;
mod option_combinator;
mod pair_combinator;
mod seq_combinator;
mod ser;
#[cfg(test)]
mod test;

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Debug, Copy, Clone, Serialize, Deserialize)]
struct RowId(u64);

#[derive(Debug)]
struct RowHeader {
    id: RowId,
    table: Weak<RowTable>,
}

pub struct Row {
    header: OnceLock<RowHeader>,
    state: RwLock<Dict>,
}

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

// pub struct RowTableReadGuard<'a> {
//     state: RwLockReadGuard<'a, RowTableState>,
// }
//
// pub struct RowTableWriteGuard<'a> {
//     state: RwLockWriteGuard<'a, RowTableState>,
// }

impl Row {
    pub fn new() -> Arc<Self> {
        Arc::new(Row {
            header: OnceLock::new(),
            state: RwLock::new(Dict::new()),
        })
    }
}

impl Default for Row {
    fn default() -> Self {
        Row {
            header: Default::default(),
            state: RwLock::new(Dict::new()),
        }
    }
}

impl RowHeader {
    pub fn id(&self) -> RowId {
        self.id
    }
}

impl RowTableQueue {
    fn try_init<'a>(&mut self, table: &Arc<RowTable>, row: &'a Arc<Row>) -> (&'a RowHeader, bool) {
        let mut inited = false;
        let header = row.header.get_or_init(|| {
            let next_id = self.next_id;
            self.next_id += 1;
            inited = true;
            RowHeader {
                id: RowId(next_id),
                table: Arc::downgrade(table),
            }
        });
        (header, inited)
    }
    fn try_enqueue(&mut self, row: &Arc<Row>) {
        self.map.insert(row.clone());
    }
    // fn create(&mut self, table: &Arc<RowTable>, id: RowId) -> Arc<Row> {
    //     if self.next_id <= id.0 {
    //         self.next_id = id.0 + 1;
    //     }
    //     Arc::new(Row {
    //         header: OnceLock::from(RowHeader {
    //             id,
    //             table: Arc::downgrade(table),
    //         }),
    //         state: RwLock::new(Dict::new()),
    //     })
    // }
}
impl RowTableState {
    pub fn try_init<'b>(&self, row: &'b Arc<Row>) -> (&'b RowHeader, bool) {
        self.queue
            .lock()
            .try_init(&self.this.upgrade().unwrap(), row)
    }
    pub fn try_enqueue<'b>(&self, row: &Arc<Row>) {
        self.queue.lock().try_enqueue(row);
    }
    // pub fn try_add_mut<'b>(&mut self, row: &'b Arc<Row>) -> &'b RowHeader {
    //     self.queue
    //         .get_mut()
    //         .try_add(&self.this.upgrade().unwrap(), row)
    // }
    // pub fn create(&mut self, id: RowId) -> Arc<Row> {
    //     self.queue
    //         .get_mut()
    //         .create(&self.this.upgrade().unwrap(), id)
    // }
    pub fn serialize_log<S: Serializer>(&mut self, s: S) -> Result<S::Ok, S::Error> {
        let mut s = s.serialize_seq(None)?;

        while !self.queue.get_mut().map.is_empty() {
            let frontier = mem::replace(&mut self.queue.get_mut().map, PtrWeakHashSet::new());
            for row in frontier.into_iter() {
                let ref mut row_state = *row.state.write();
                let key = row.header.get().expect("uninitialized").id;
                if row_state.begin_update() {
                    #[derive(Serialize)]
                    struct Entry<A, B> {
                        key: A,
                        value: B,
                    }
                    s.serialize_element(&Entry {
                        key,
                        value: Some(SerializeUpdateAdapter::new(row_state, self)),
                    })?;
                } else {
                    s.serialize_element(&(key, Option::<!>::None))?;
                }
            }
        }
        s.end()
    }
    // pub fn add(&self) -> Arc<Row> {
    //     let mut queue = self.state.queue.lock();
    //     let next_id = queue.next_id;
    //     queue.next_id += 1;
    //     let row = Arc::new(Row {
    //         id: RowId(next_id),
    //         table: Arc::downgrade(self.this),
    //         anchored: AtomicBool::new(false),
    //         state: RwLock::new(Dict::new()),
    //     });
    //     assert!(!queue.map.insert(row.clone()));
    //     row
    // }
    // pub fn add_root(&self) -> Arc<Row> {
    //     let root = self.add();
    //     root.anchored.store(true, Ordering::Relaxed);
    //     root
    // }
    pub fn read<'b>(&'b self, row: &'b Arc<Row>) -> RwLockReadGuard<'b, Dict> {
        row.state.read()
    }
    pub fn write<'b>(&'b self, row: &'b Arc<Row>) -> RwLockWriteGuard<'b, Dict> {
        self.queue.lock().map.insert(row.clone());
        row.state.write()
    }
}

impl RowTable {
    pub fn new(root: Arc<Row>) -> Arc<Self> {
        let result = Arc::new_cyclic(|this| RowTable {
            state: RwLock::new(RowTableState {
                this: this.clone(),
                queue: Mutex::new(RowTableQueue {
                    next_id: 0,
                    map: PtrWeakHashSet::new(),
                }),
            }),
        });
        result.read().try_init(&root);
        result.read().try_enqueue(&root);

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

impl Debug for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Row")
            .field("header", &self.header)
            .field("state", &self.state)
            .finish()
    }
}

fn arc_try_new_cyclic<T: Default, E>(
    f: impl for<'a> FnOnce(&'a Weak<T>) -> Result<T, E>,
) -> Result<Arc<T>, E> {
    let mut err = None;
    let x = Arc::new_cyclic(|x| match f(x) {
        Err(e) => {
            err = Some(e);
            T::default()
        }
        Ok(x) => x,
    });
    if let Some(err) = err {
        Err(err)
    } else {
        Ok(x)
    }
}

impl<'de> DeserializeUpdate<'de> for Arc<Row> {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'a> {
            table: DeserializeContext<'a>,
        };
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
                first: Self::First,
                d: D,
            ) -> Result<Self::Second, D::Error> {
                let table = &*self.table.table;
                let des = &mut *self.table.des;
                match des.entries.entry(first) {
                    Entry::Occupied(x) => {
                        Option::<!>::deserialize(d)?;
                        Ok(x.get()
                            .upgrade_cow()
                            .expect("uninitialized row")
                            .into_owned())
                    }
                    Entry::Vacant(entry) => {
                        mem::drop(entry);
                        let row = arc_try_new_cyclic(|row: &Weak<Row>| {
                            des.entries.insert(first, ArcOrWeak::Weak(row.clone()));
                            let dict =
                                OptionCombinator::new(DeserializeSnapshotAdapter::<Dict>::new(
                                    DeserializeContext { table, des },
                                ))
                                .deserialize(d)?;
                            Ok(Row {
                                header: OnceLock::from(RowHeader {
                                    id: first,
                                    table: table.this.clone(),
                                }),
                                state: RwLock::new(
                                    dict.ok_or_else(|| D::Error::custom("missing definition"))?,
                                ),
                            })
                        })?;
                        des.entries.insert(first, ArcOrWeak::Arc(row.clone()));
                        Ok(row)
                    }
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
        todo!()
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
        todo!()
    }
}

impl SerializeUpdate for Arc<Row> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer>(
        &self,
        state: &RowTableState,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        let (header, new) = state.try_init(self);
        let mut s = s.serialize_struct("Arc", 2)?;
        s.serialize_field("id", &header.id())?;
        if new {
            s.serialize_field(
                "value",
                &Some(SerializeUpdateAdapter::new(&*state.read(self), state)),
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

impl SerializeUpdate for Weak<Row> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer>(
        &self,
        state: &RowTableState,
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
