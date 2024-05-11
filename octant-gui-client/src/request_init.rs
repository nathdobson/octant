use std::{marker::PhantomData, sync::Arc};

use octant_gui_core::{HandleId, RequestInitMethod, RequestInitTag, TypedHandle};
use octant_object::define_class;

use crate::{
    object,
    object::{Object, ObjectValue},
    peer, HasLocalType, Runtime,
};
use crate::peer::ArcPeer;

define_class! {
    pub class RequestInit extends Object {
        request_init: web_sys::RequestInit,
    }
}

impl RequestInitValue {
    pub fn new(handle: HandleId, request_init: web_sys::RequestInit) -> Self {
        RequestInitValue {
            parent: ObjectValue::new(handle, Clone::clone(&request_init).into()),
            request_init,
        }
    }
    pub fn native(&self) -> &web_sys::RequestInit {
        &self.request_init
    }
    pub fn handle(&self) -> TypedHandle<RequestInitTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn RequestInit {
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &RequestInitMethod,
        _handle_id: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            _ => todo!(),
        }
    }
}

impl HasLocalType for RequestInitTag {
    type Local = dyn RequestInit;
}
