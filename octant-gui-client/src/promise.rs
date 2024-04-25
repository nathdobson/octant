use std::{marker::PhantomData, sync::Arc};

use js_sys::Promise;

use octant_gui_core::{
    {PromiseMethod, PromiseTag},
    HandleId, TypedHandle,
};
use octant_object::define_class;

use crate::{HasLocalType, object, peer, Runtime};

define_class! {
    pub class extends object {
        promise: Promise,
    }
}

impl Value {
    pub fn new(handle: HandleId, promise: Promise) -> Self {
        Value {
            parent: object::Value::new(handle, promise.clone().into()),
            promise,
        }
    }
    pub fn native(&self) -> &Promise {
        &self.promise
    }
    pub fn handle(&self) -> TypedHandle<PromiseTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Trait {
    pub fn invoke_with(
        self: &Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &PromiseMethod,
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            _ => todo!(),
        }
    }
}

impl HasLocalType for PromiseTag {
    type Local = dyn Trait;
}
