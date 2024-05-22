use std::fmt::Debug;
use crate::{
    handle::{RawHandle, TypedHandle},
};
use octant_object::{
    base::{Base, BaseValue},
    define_class,
};
use safe_once::{
    api::once::OnceEntry,
    cell::{OnceCell},
};
define_class! {
    #[derive(Debug)]
    pub class Peer extends Base implements Debug {
        handle: OnceCell<RawHandle>,
    }
}

impl PeerValue {
    pub fn new() -> Self {
        PeerValue {
            parent: BaseValue::default(),
            handle: OnceCell::new(),
        }
    }
    pub fn raw_handle(&self) -> RawHandle {
        *self.handle.try_get().unwrap()
    }
    pub fn set_handle(&self, handle: RawHandle) {
        match self.handle.lock() {
            OnceEntry::Occupied(_) => panic!("already initialized"),
            OnceEntry::Vacant(x) => {
                x.init(handle);
            }
        }
    }
}

impl dyn Peer {
    pub fn typed_handle(&self) -> TypedHandle<dyn Peer> {
        TypedHandle::new(self.raw_handle())
    }
}

impl Drop for PeerValue {
    fn drop(&mut self) {
        todo!()
    }
}
