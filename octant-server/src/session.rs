use std::any::{Any, TypeId};
use std::sync::Arc;

use memo_map::MemoMap;

use octant_gui::Global;

pub struct Session {
    global: Arc<Global>,
    data: MemoMap<TypeId, Box<dyn 'static + Any + Send + Sync>>,
}

pub trait SessionData: 'static + Sync + Send + Default {}

impl Session {
    pub fn new(global: Arc<Global>) -> Session {
        Session {
            global,
            data: MemoMap::new(),
        }
    }
    pub fn global(&self) -> &Arc<Global> {
        &self.global
    }
    pub fn data<T: SessionData>(&self) -> &T {
        self.data
            .get_or_insert(&TypeId::of::<T>(), || Box::<T>::default())
            .downcast_ref()
            .unwrap()
    }
}
