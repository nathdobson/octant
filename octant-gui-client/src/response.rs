use std::{marker::PhantomData, sync::Arc};

use octant_gui_core::{HandleId, ResponseMethod, ResponseTag, TypedHandle};
use octant_object::define_class;

use crate::{
    HasLocalType
    ,
    object::Object
    ,
    peer::ArcPeer,
    promise::{ArcPromise, PromiseValue}, Runtime,
};
use crate::object::ObjectValue;

define_class! {
    pub class Response extends Object {
        response: web_sys::Response,
    }
}

impl ResponseValue {
    pub fn new(handle: HandleId, response: web_sys::Response) -> Self {
        ResponseValue {
            parent: ObjectValue::new(handle, Clone::clone(&response).into()),
            response,
        }
    }
    pub fn native(&self) -> &web_sys::Response {
        &self.response
    }
    pub fn handle(&self) -> TypedHandle<ResponseTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Response {
    pub fn text(&self, handle: HandleId) -> ArcPromise {
        Arc::new(PromiseValue::new(handle, self.response.text().unwrap()))
    }
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &ResponseMethod,
        handle_id: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            ResponseMethod::Text() => Some(self.text(handle_id)),
        }
    }
}

impl HasLocalType for ResponseTag {
    type Local = dyn Response;
}
