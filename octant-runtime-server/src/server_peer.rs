use crate::{
    handle::{RawHandle, TypedHandle},
    runtime::Runtime,
};
use octant_object::{
    base::{Base, BaseValue},
    define_class,
};
use std::{fmt::Debug, sync::Arc};

pub trait SendSyncDebug: Send + Sync + Debug {}
impl<T> SendSyncDebug for T where T: Send + Sync + Debug {}

define_class! {
    #[derive(Debug)]
    pub class Peer extends Base implements SendSyncDebug{
        runtime: Arc<Runtime>,
        handle: RawHandle,
    }
}

impl PeerValue {
    pub fn new(runtime: Arc<Runtime>, handle: RawHandle) -> Self {
        PeerValue {
            parent: BaseValue::default(),
            runtime,
            handle,
        }
    }
    pub fn raw_handle(&self) -> RawHandle {
        self.handle
    }
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
}

impl dyn Peer {
    pub fn typed_handle(&self) -> TypedHandle<dyn Peer> {
        TypedHandle::new(self.handle)
    }
}

impl Drop for PeerValue {
    fn drop(&mut self) {
        todo!()
    }
}
