use std::fmt::Debug;
use std::rc::Rc;

use octant_object::{
    base::{Base, BaseValue},
    define_class,
};

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
};

define_class! {
    pub class Peer extends Base implements Debug{
        field runtime: Rc<Runtime>;
        field handle: RawHandle;
        fn runtime(self: &Self) -> &Rc<Runtime> {
            &self.peer().runtime
        }
    }
}

impl PeerValue {
    pub fn new(runtime: Rc<Runtime>, handle: RawHandle) -> Self {
        PeerValue {
            parent: BaseValue::default(),
            runtime,
            handle,
        }
    }
    pub fn raw_handle(&self) -> RawHandle {
        self.handle
    }
    // pub fn runtime(&self) -> &Arc<Runtime> {
    //     &self.runtime
    // }
}

impl dyn Peer {
    pub fn typed_handle(&self) -> TypedHandle<dyn Peer> {
        TypedHandle::new(self.handle)
    }
}

impl Drop for PeerValue {
    fn drop(&mut self) {
        self.runtime().delete(self.handle);
    }
}
