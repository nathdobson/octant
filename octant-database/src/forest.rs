use std::{
    mem,
    sync::{Arc, Weak},
};

use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{ser::SerializeMap, Serializer};
use weak_table::PtrWeakHashSet;

use crate::{
    dict::Dict,
    ser::SerializeUpdate,
    tree::{Tree, TreeId},
};

struct ForestGlobalState {
    next_id: u64,
    snapshot_queue: Option<Arc<Tree>>,
    update_queue: PtrWeakHashSet<Weak<Tree>>,
}

pub struct ForestState {
    this: Weak<Forest>,
    queue: Mutex<ForestGlobalState>,
}

pub struct Forest {
    state: RwLock<ForestState>,
}

impl ForestGlobalState {
    fn enqueue_snapshot(&mut self, root: Arc<Tree>) {
        self.snapshot_queue = Some(root);
    }
    fn enqueue_update(&mut self, row: &Arc<Tree>) {
        self.update_queue.insert(row.clone());
    }
}

impl ForestState {
    pub fn enqueue_update<'b>(&self, row: &Arc<Tree>) {
        self.queue.lock().enqueue_update(row);
    }
    pub fn enqueue_snapshot<'b>(&self, row: Arc<Tree>) {
        self.queue.lock().enqueue_snapshot(row);
    }
    pub fn serialize_update<S: Serializer>(&mut self, s: S) -> Result<S::Ok, S::Error> {
        if let Some(mut snapshot) = self.queue.get_mut().snapshot_queue.take() {
            self.queue.get_mut().update_queue.clear();
            assert!(snapshot.begin_update());
            let result = snapshot.serialize_update(self, s)?;
            snapshot.end_update();
            Ok(result)
        } else {
            let mut s = s.serialize_map(None)?;
            for row in mem::replace(
                &mut self.queue.get_mut().update_queue,
                PtrWeakHashSet::new(),
            ) {
                row.serialize_tree(&mut s, self)?;
            }
            s.end()
        }
    }
    pub fn read<'b>(&'b self, row: &'b Arc<Tree>) -> RwLockReadGuard<'b, Dict> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        row.read()
    }
    pub fn try_read<'b>(&'b self, row: &'b Arc<Tree>) -> Option<RwLockReadGuard<'b, Dict>> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        row.try_read()
    }
    pub fn write<'b>(&'b self, row: &'b Arc<Tree>) -> RwLockWriteGuard<'b, Dict> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        self.queue.lock().update_queue.insert(row.clone());
        row.write()
    }
    pub fn try_write<'b>(&'b self, row: &'b Arc<Tree>) -> Option<RwLockWriteGuard<'b, Dict>> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        self.queue.lock().update_queue.insert(row.clone());
        row.try_write()
    }
    pub fn next_id(&self) -> TreeId {
        let ref mut queue = *self.queue.lock();
        let id = queue.next_id;
        queue.next_id += 1;
        TreeId::new(id)
    }
    pub fn get_arc(&self) -> Weak<Forest> {
        self.this.clone()
    }
}

impl Forest {
    pub fn new() -> Arc<Self> {
        let result = Arc::new_cyclic(|this| Forest {
            state: RwLock::new(ForestState {
                this: this.clone(),
                queue: Mutex::new(ForestGlobalState {
                    next_id: 0,
                    snapshot_queue: None,
                    update_queue: PtrWeakHashSet::new(),
                }),
            }),
        });
        result
    }
    pub fn with_root(root: Arc<Tree>) -> Arc<Self> {
        let this = Self::new();
        this.state.write().enqueue_snapshot(root);
        this
    }
    pub fn read<'a>(self: &'a Arc<Self>) -> RwLockReadGuard<'a, ForestState> {
        self.state.read()
    }
    pub fn write<'a>(self: &'a Arc<Self>) -> RwLockWriteGuard<'a, ForestState> {
        self.state.write()
    }
}
