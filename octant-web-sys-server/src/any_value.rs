use octant_runtime::define_sys_class;
use octant_runtime::peer::{Peer, PeerValue};

define_sys_class! {
    class AnyValue;
    extends Peer;
    wasm wasm_bindgen::JsValue;
}

#[cfg(side = "client")]
impl AnyValueValue {
    pub fn new(any_value: wasm_bindgen::JsValue) -> Self {
        AnyValueValue {
            parent: PeerValue::new(),
            any_value,
        }
    }
}

#[cfg(side = "server")]
impl AnyValueValue {
    pub fn new(handle: PeerValue) -> Self {
        AnyValueValue { parent: handle }
    }
}
