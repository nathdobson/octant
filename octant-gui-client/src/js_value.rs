use wasm_bindgen::JsValue;

use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::peer;

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
