use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct UniqueId(u64);

pub static ID_COUNTER: AtomicU64 = AtomicU64::new(0);
impl UniqueId {
    pub fn new() -> Self {
        UniqueId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}
