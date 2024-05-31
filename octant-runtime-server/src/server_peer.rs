use std::{fmt::Debug, rc::Rc};

use octant_object::{base::{Base, BaseFields}, class, class::Class, DebugClass};

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
    PeerNew,
};

#[derive(DebugClass)]
pub struct PeerFields {
    parent: BaseFields,
    runtime: Rc<Runtime>,
    handle: RawHandle,
}

#[class]
pub trait Peer: Base + Debug {
    fn runtime(&self) -> &Rc<Runtime> {
        &self.peer().runtime
    }
}

impl PeerFields {
    pub fn new(runtime: Rc<Runtime>, handle: RawHandle) -> Self {
        PeerFields {
            parent: BaseFields::default(),
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

impl Drop for PeerFields {
    fn drop(&mut self) {
        self.runtime().delete(self.handle);
    }
}

impl PeerNew for PeerFields {
    type Builder = PeerFields;
    fn peer_new(peer: PeerFields) -> Self {
        peer
    }
}

pub trait AsNative: Class {}
