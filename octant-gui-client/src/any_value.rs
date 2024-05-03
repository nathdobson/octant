use std::sync::Arc;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::Credential;

use octant_gui_core::{AnyValueMethod, AnyValueTag, HandleId, JsClass};
use octant_object::define_class;

use crate::{credential, peer, HasLocalType, Runtime};

define_class! {
    pub class extends peer {
        js_value: JsValue,
    }
}

impl Value {
    pub fn new(handle: HandleId, js_value: JsValue) -> Self {
        Value {
            parent: peer::Value::new(handle.into()),
            js_value,
        }
    }
}

impl dyn Trait {
    pub fn invoke_with(
        self: &Arc<Self>,
        _runtime: &Arc<Runtime>,
        method: &AnyValueMethod,
        handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            AnyValueMethod::Downcast(class) => match class {
                JsClass::Credential => Some(Arc::new(credential::Value::new(
                    handle,
                    self.js_value.dyn_ref::<Credential>().unwrap().clone(),
                ))),
            },
        }
    }
}

impl HasLocalType for AnyValueTag {
    type Local = dyn Trait;
}
