use std::{marker::PhantomData, sync::Arc};

use web_sys::Request;

use octant_gui_core::{
    HandleId, RequestMethod, RequestTag, TypedHandle,
};
use octant_object::define_class;

use crate::{HasLocalType, object, peer, Runtime};

define_class! {
    pub class extends object {
        request: Request,
    }
}

impl Value {
    pub fn new(handle: HandleId, request: Request) -> Self {
        Value {
            parent: object::Value::new(handle, Clone::clone(&request).into()),
            request,
        }
    }
    pub fn native(&self) -> &Request {
        &self.request
    }
    pub fn handle(&self) -> TypedHandle<RequestTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Trait {
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &RequestMethod,
        _handle_id: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            _ => todo!(),
        }
    }
}

impl HasLocalType for RequestTag {
    type Local = dyn Trait;
}
