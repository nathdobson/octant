use std::{
    collections::HashSet,
    mem,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use parking_lot::{Mutex, RwLockReadGuard, RwLockWriteGuard};

use crate::tree::{Tree, TreeId};

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Debug, Copy, Clone)]
pub(crate) struct ForestId(usize);
static FOREST_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
impl ForestId {
    pub fn new() -> Self {
        ForestId(FOREST_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

struct ForestState {
    next_id: u64,
    update_queue: HashSet<TreeId>,
}

pub struct Forest {
    forest_id: ForestId,
    queue: Mutex<ForestState>,
}

impl Forest {
    pub(crate) fn enqueue_update<T>(&self, row: &Arc<Tree<T>>) {
        self.queue.lock().update_queue.insert(row.id(self));
    }
    pub(crate) fn take_queue(&mut self) -> HashSet<TreeId> {
        mem::replace(&mut self.queue.get_mut().update_queue, HashSet::new())
    }
    pub fn read<'b, T: ?Sized>(&'b self, row: &'b Arc<Tree<T>>) -> RwLockReadGuard<'b, T> {
        assert_eq!(row.forest(self.forest_id), self.forest_id);
        row.read()
    }
    pub fn try_read<'b, T: ?Sized>(
        &'b self,
        row: &'b Arc<Tree<T>>,
    ) -> Option<RwLockReadGuard<'b, T>> {
        assert_eq!(row.forest(self.forest_id), self.forest_id);
        row.try_read()
    }
    pub fn write<'b, T: ?Sized>(&'b self, row: &'b Arc<Tree<T>>) -> RwLockWriteGuard<'b, T> {
        assert_eq!(row.forest(self.forest_id), self.forest_id);
        self.queue.lock().update_queue.insert(row.id(self));
        row.write()
    }
    pub fn try_write<'b, T: ?Sized>(
        &'b self,
        row: &'b Arc<Tree<T>>,
    ) -> Option<RwLockWriteGuard<'b, T>> {
        assert_eq!(row.forest(self.forest_id), self.forest_id);
        self.queue.lock().update_queue.insert(row.id(self));
        row.try_write()
    }
    pub(crate) fn next_id(&self) -> TreeId {
        let ref mut queue = *self.queue.lock();
        let id = queue.next_id;
        queue.next_id += 1;
        TreeId::new(id)
    }
}

impl Forest {
    pub fn new() -> Forest {
        Forest {
            forest_id: ForestId::new(),
            queue: Mutex::new(ForestState {
                next_id: 0,
                update_queue: HashSet::new(),
            }),
        }
    }
}
