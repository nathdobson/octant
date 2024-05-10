use std::{marker::PhantomData, sync::Arc};

use web_sys::Response;

use octant_gui_core::{
    HandleId,
    ResponseMethod, ResponseTag, TypedHandle,
};
use octant_object::define_class;

use crate::{HasLocalType, object, peer, promise, Runtime};

define_class! {
    pub class extends object {
        response: Response,
    }
}

impl Value {
    pub fn new(handle: HandleId, response: Response) -> Self {
        Value {
            parent: object::Value::new(handle, Clone::clone(&response).into()),
            response,
        }
    }
    pub fn native(&self) -> &Response {
        &self.response
    }
    pub fn handle(&self) -> TypedHandle<ResponseTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Trait {
    pub fn text(&self, handle: HandleId) -> Arc<dyn promise::Trait> {
        Arc::new(promise::Value::new(
            handle,
            self.response.text().unwrap(),
        ))
    }
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &ResponseMethod,
        handle_id: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            ResponseMethod::Text() => Some(self.text(handle_id)),
        }
    }
}

impl HasLocalType for ResponseTag {
    type Local = dyn Trait;
}
