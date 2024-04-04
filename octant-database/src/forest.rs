use std::{
    collections::HashSet,
    mem,
    sync::{Arc, Weak},
};

use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::tree::{Tree, TreeId};

struct ForestGlobalState {
    next_id: u64,
    update_queue: HashSet<TreeId>,
    // snapshot_queue: Option<Arc<Tree<dyn TreeInner>>>,
    // update_queue: PtrWeakHashSet<Weak<Tree<dyn TreeInner>>>,
}

pub struct ForestState {
    this: Weak<Forest>,
    queue: Mutex<ForestGlobalState>,
}

pub struct Forest {
    state: RwLock<ForestState>,
}

impl ForestGlobalState {
    // fn enqueue_snapshot(&mut self, root: Arc<Tree<dyn TreeInner>>) {
    //     self.snapshot_queue = Some(root);
    // }
    fn enqueue_update<T: ?Sized>(&mut self, forest: &ForestState, row: &Arc<Tree<T>>) {
        self.update_queue.insert(row.id(forest));
    }
}

impl ForestState {
    pub fn enqueue_update<T>(&self, row: &Arc<Tree<T>>) {
        self.queue.lock().enqueue_update(self, row);
    }
    pub fn take_queue(&mut self) -> HashSet<TreeId> {
        mem::replace(&mut self.queue.get_mut().update_queue, HashSet::new())
    }
    // pub fn enqueue_snapshot<'b>(&self, row: Arc<Tree<dyn TreeInner>>) {
    //     self.queue.lock().enqueue_snapshot(row);
    // }

    pub fn read<'b, T: ?Sized>(&'b self, row: &'b Arc<Tree<T>>) -> RwLockReadGuard<'b, T> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        row.read()
    }
    pub fn try_read<'b, T: ?Sized>(
        &'b self,
        row: &'b Arc<Tree<T>>,
    ) -> Option<RwLockReadGuard<'b, T>> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        row.try_read()
    }
    pub fn write<'b, T: ?Sized>(&'b self, row: &'b Arc<Tree<T>>) -> RwLockWriteGuard<'b, T> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        self.queue.lock().update_queue.insert(row.id(self));
        row.write()
    }
    pub fn try_write<'b, T: ?Sized>(
        &'b self,
        row: &'b Arc<Tree<T>>,
    ) -> Option<RwLockWriteGuard<'b, T>> {
        assert!(row.forest(&self.this).ptr_eq(&self.this));
        self.queue.lock().update_queue.insert(row.id(self));
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
                    update_queue: HashSet::new(),
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
