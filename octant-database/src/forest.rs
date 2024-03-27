use crate::{
    dict::Dict,
    tree::{Tree, TreeId},
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::Serializer;
use std::{
    mem,
    sync::{Arc, Weak},
};
use serde::ser::SerializeSeq;
use weak_table::PtrWeakHashSet;

struct ForestGlobalState {
    next_id: u64,
    map: PtrWeakHashSet<Weak<Tree>>,
}

pub struct ForestState {
    queue: Mutex<ForestGlobalState>,
}

pub struct Forest {
    state: RwLock<ForestState>,
}

impl ForestGlobalState {
    fn try_enqueue(&mut self, row: &Arc<Tree>) {
        self.map.insert(row.clone());
    }
}

impl ForestState {
    pub fn enqueue<'b>(&self, row: &Arc<Tree>) {
        self.queue.lock().try_enqueue(row);
    }
    pub fn serialize_log<S: Serializer>(&mut self, s: S) -> Result<S::Ok, S::Error> {
        let mut s = s.serialize_seq(None)?;
        for row in mem::replace(&mut self.queue.get_mut().map, PtrWeakHashSet::new()) {
            row.serialize_tree(&mut s, self)?;
        }
        s.end()
    }
    pub fn read<'b>(&'b self, row: &'b Arc<Tree>) -> RwLockReadGuard<'b, Dict> {
        row.read()
    }
    pub fn try_read<'b>(&'b self, row: &'b Arc<Tree>) -> Option<RwLockReadGuard<'b, Dict>> {
        row.try_read()
    }
    pub fn write<'b>(&'b self, row: &'b Arc<Tree>) -> RwLockWriteGuard<'b, Dict> {
        if row.is_written() {
            self.queue.lock().map.insert(row.clone());
        }
        row.write()
    }
    pub fn try_write<'b>(&'b self, row: &'b Arc<Tree>) -> Option<RwLockWriteGuard<'b, Dict>> {
        if row.is_written() {
            self.queue.lock().map.insert(row.clone());
        }
        row.try_write()
    }
    pub fn add(&self) -> Arc<Tree> {
        let ref mut lock = *self.queue.lock();
        let id = lock.next_id;
        lock.next_id += 1;
        Tree::new(TreeId::new(id))
    }
}

impl Forest {
    pub fn new() -> Arc<Self> {
        let result = Arc::new(Forest {
            state: RwLock::new(ForestState {
                queue: Mutex::new(ForestGlobalState {
                    next_id: 0,
                    map: PtrWeakHashSet::new(),
                }),
            }),
        });
        result
    }
    pub fn read<'a>(self: &'a Arc<Self>) -> RwLockReadGuard<'a, ForestState> {
        self.state.read()
    }
    pub fn write<'a>(self: &'a Arc<Self>) -> RwLockWriteGuard<'a, ForestState> {
        self.state.write()
    }
}


