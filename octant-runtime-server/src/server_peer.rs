use std::{fmt::Debug, rc::Rc};

use octant_object::{
    base::{Base, BaseValue},
    class,
};

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
};

#[class]
pub struct Peer {
    parent: dyn Base,
    runtime: Rc<Runtime>,
    handle: RawHandle,
}

pub trait Peer: AsPeer + Debug {
    fn runtime(&self) -> &Rc<Runtime> {
        &self.peer().runtime
    }
}

impl<T> Peer for T where T: AsPeer + Debug {}

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
