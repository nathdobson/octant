use std::{marker::PhantomData, sync::Arc};

use octant_gui_core::{
    HandleId, RequestMethod, RequestTag, TypedHandle,
};
use octant_object::define_class;

use crate::{HasLocalType, Runtime};
use crate::object::{Object, ObjectValue};
use crate::peer::ArcPeer;

define_class! {
    pub class Request extends Object {
        request: web_sys::Request,
    }
}

impl RequestValue {
    pub fn new(handle: HandleId, request: web_sys::Request) -> Self {
        RequestValue {
            parent: ObjectValue::new(handle, Clone::clone(&request).into()),
            request,
        }
    }
    pub fn native(&self) -> &web_sys::Request {
        &self.request
    }
    pub fn handle(&self) -> TypedHandle<RequestTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Request {
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &RequestMethod,
        _handle_id: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            _ => todo!(),
        }
    }
}

impl HasLocalType for RequestTag {
    type Local = dyn Request;
}
