use std::{fmt::Debug, rc::Rc};

use octant_object::{base::{Base, BaseValue}, class, class::Class, DebugClass};

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
    PeerNew,
};

#[derive(DebugClass)]
pub struct PeerValue {
    parent: BaseValue,
    runtime: Rc<Runtime>,
    handle: RawHandle,
}

#[class]
pub trait Peer: Base + Debug {
    fn runtime(&self) -> &Rc<Runtime> {
        &self.peer().runtime
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

pub trait AsNative: Class {}
