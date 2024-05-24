use octant_runtime::define_sys_class;
use octant_runtime::peer::{Peer, PeerValue};
define_sys_class! {
    class JsValue;
    extends Peer;
    wasm wasm_bindgen::JsValue;
}

#[cfg(side = "server")]
impl From<PeerValue> for JsValueValue {
    fn from(handle: PeerValue) -> Self {
        JsValueValue { parent: handle }
    }
}

#[cfg(side = "client")]
impl JsValueValue {
    pub fn new(js_value: wasm_bindgen::JsValue) -> JsValueValue {
        JsValueValue {
            parent: PeerValue::new(),
            js_value,
        }
    }
}
