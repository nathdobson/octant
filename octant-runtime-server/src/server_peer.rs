use std::{fmt::Debug, rc::Rc};

use octant_object::{
    base::{Base, BaseValue},
    class,
};

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
    PeerNew,
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

impl PeerNew for PeerValue {
    type Builder = PeerValue;
    fn peer_new(peer: PeerValue) -> Self {
        peer
    }
}
