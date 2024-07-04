use std::{fmt::Debug, rc::Rc};
use marshal::context::Context;
use marshal::decode::{AnyDecoder, Decoder};
use safe_once::{api::once::OnceEntry, cell::OnceCell};

use octant_object::{class, class::Class, DebugClass};
use octant_object::base::{Base, BaseFields};
use octant_runtime_derive::{DeserializePeer, SerializePeer};

use crate::{
    handle::{RawHandle, TypedHandle},
    PeerNew,
    runtime::RuntimeSink,
};

#[derive(Debug)]
struct PeerInit {
    handle: RawHandle,
    sink: Rc<RuntimeSink>,
}

#[derive(DebugClass, SerializePeer, DeserializePeer)]
pub struct PeerFields {
    parent: BaseFields,
    peer_init: OnceCell<PeerInit>,
}

#[class]
pub trait Peer: Base + Debug {}

impl PeerFields {
    pub fn new() -> Self {
        PeerFields {
            parent: BaseFields::default(),
            peer_init: OnceCell::new(),
        }
    }
    pub fn raw_handle(&self) -> RawHandle {
        self.peer_init.try_get().unwrap().handle
    }
    pub fn sink(&self) -> &Rc<RuntimeSink> {
        &self.peer_init.try_get().unwrap().sink
    }
    pub fn init(&self, handle: RawHandle, sink: Rc<RuntimeSink>) {
        match self.peer_init.lock() {
            OnceEntry::Occupied(_) => panic!("already initialized"),
            OnceEntry::Vacant(x) => {
                x.init(PeerInit { handle, sink });
            }
        }
    }
}

impl dyn Peer {
    pub fn typed_handle(&self) -> TypedHandle<dyn Peer> {
        TypedHandle::new(self.raw_handle())
    }
}

impl PeerNew for PeerFields {
    type Builder = ();
    fn peer_new(builder: Self::Builder) -> Self {
        PeerFields::new()
    }
}

pub trait AsNative: Class {
    type Native;
    fn native(&self) -> &Self::Native;
}

