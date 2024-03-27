use crate::{
    dict::Dict,
    row::{Row, RowId},
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::Serializer;
use std::{
    mem,
    sync::{Arc, Weak},
};
use serde::ser::SerializeSeq;
use weak_table::PtrWeakHashSet;

struct RowTableQueue {
    next_id: u64,
    map: PtrWeakHashSet<Weak<Row>>,
}

pub struct RowTableState {
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
        let result = Arc::new(RowTable {
            state: RwLock::new(RowTableState {
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


