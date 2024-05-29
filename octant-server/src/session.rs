use std::any::{Any, TypeId};
use std::rc::Rc;

use memo_map::MemoMap;

use octant_web_sys_server::global::Global;

pub struct Session {
    global: Rc<Global>,
    data: MemoMap<TypeId, Box<dyn 'static + Any + Send + Sync>>,
}

pub trait SessionData: 'static + Sync + Send + Default {}

impl Session {
    pub fn new(global: Rc<Global>) -> Session {
        Session {
            global,
            data: MemoMap::new(),
        }
    }
    pub fn global(&self) -> &Rc<Global> {
        &self.global
    }
    pub fn data<T: SessionData>(&self) -> &T {
        self.data
            .get_or_insert(&TypeId::of::<T>(), || Box::<T>::default())
            .downcast_ref()
            .unwrap()
    }
}
