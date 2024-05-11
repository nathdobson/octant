
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::peer;
use crate::peer::{Peer, PeerValue};

define_class! {
    pub class JsValue extends Peer {
        js_value: wasm_bindgen::JsValue,
    }
}

impl JsValueValue {
    pub fn new(handle: HandleId, js_value: wasm_bindgen::JsValue) -> Self {
        JsValueValue {
            parent: PeerValue::new(handle.into()),
            js_value,
        }
    }
}
