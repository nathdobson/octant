use std::sync::Arc;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Credential, Response};

use octant_gui_core::{AnyValueMethod, AnyValueTag, HandleId, JsClass};
use octant_object::define_class;

use crate::{
    credential, peer,
    peer::{ArcPeer, Peer, PeerValue},
    response, HasLocalType, Runtime,
};
use crate::credential::CredentialValue;
use crate::response::ResponseValue;

define_class! {
    pub class AnyValue extends Peer {
        js_value: JsValue,
    }
}

impl AnyValueValue {
    pub fn new(handle: HandleId, js_value: JsValue) -> Self {
        AnyValueValue {
            parent: PeerValue::new(handle.into()),
            js_value,
        }
    }
}

impl dyn AnyValue {
    pub fn invoke_with(
        self: &Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &AnyValueMethod,
        handle: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            AnyValueMethod::Downcast(class) => match class {
                JsClass::Credential => Some(Arc::new(CredentialValue::new(
                    handle,
                    self.js_value.dyn_ref::<Credential>().unwrap().clone(),
                ))),
                JsClass::Response => Some(Arc::new(ResponseValue::new(
                    handle,
                    Clone::clone(self.js_value.dyn_ref::<Response>().unwrap()),
                ))),
            },
        }
    }
}

impl HasLocalType for AnyValueTag {
    type Local = dyn AnyValue;
}
