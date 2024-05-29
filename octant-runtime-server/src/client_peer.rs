use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::RuntimeSink,
};
use octant_object::{
    base::{Base, BaseValue},
    define_class,
};
use safe_once::{api::once::OnceEntry, cell::OnceCell};
use std::{fmt::Debug};
use std::rc::Rc;

#[derive(Debug)]
struct PeerInit {
    handle: RawHandle,
    sink: Rc<RuntimeSink>,
}

define_class! {
    pub class Peer extends Base implements Debug {
        field peer_init: OnceCell<PeerInit>;
    }
}

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
