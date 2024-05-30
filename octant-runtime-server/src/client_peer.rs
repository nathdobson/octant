use std::{fmt::Debug, rc::Rc};

use safe_once::{api::once::OnceEntry, cell::OnceCell};

use octant_object::base::{Base, BaseValue};

use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::RuntimeSink,
    PeerNew,
};
use octant_object::class;

#[derive(Debug)]
struct PeerInit {
    handle: RawHandle,
    sink: Rc<RuntimeSink>,
}

#[class]
pub struct Peer {
    parent: dyn Base,
    peer_init: OnceCell<PeerInit>,
}

pub trait Peer: AsPeer + Debug {}

impl<T> Peer for T where T: AsPeer + Debug {}

impl PeerValue {
    pub fn new() -> Self {
        PeerValue {
            parent: BaseValue::default(),
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

impl PeerNew for PeerValue {
    type Builder = ();
    fn peer_new(builder: Self::Builder) -> Self {
        PeerValue::new()
    }
}
