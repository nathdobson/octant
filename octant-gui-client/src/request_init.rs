use std::{marker::PhantomData, sync::Arc};

use web_sys::RequestInit;

use octant_gui_core::{HandleId, RequestInitMethod, RequestInitTag, TypedHandle};
use octant_object::define_class;

use crate::{HasLocalType, object, peer, Runtime};

define_class! {
    pub class extends object {
        request_init: RequestInit,
    }
}

impl Value {
    pub fn new(handle: HandleId, request_init: RequestInit) -> Self {
        Value {
            parent: object::Value::new(handle, Clone::clone(&request_init).into()),
            request_init,
        }
    }
    pub fn native(&self) -> &RequestInit {
        &self.request_init
    }
    pub fn handle(&self) -> TypedHandle<RequestInitTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Trait {
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &RequestInitMethod,
        _handle_id: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            _ => todo!(),
        }
    }
}

impl HasLocalType for RequestInitTag {
    type Local = dyn Trait;
}
