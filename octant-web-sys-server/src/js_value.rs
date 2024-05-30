use octant_runtime::{
    define_sys_class,
    peer::{Peer, PeerValue},
    PeerNew,
};

define_sys_class! {
    class JsValue;
    extends Peer;
    wasm wasm_bindgen::JsValue;
}

#[cfg(side = "server")]
impl PeerNew for JsValueValue {
    type Builder = PeerValue;
    fn peer_new(handle: PeerValue) -> Self {
        JsValueValue { parent: handle }
    }
}

#[cfg(side = "client")]
impl PeerNew for JsValueValue {
    type Builder = wasm_bindgen::JsValue;
    fn peer_new(js_value: wasm_bindgen::JsValue) -> JsValueValue {
        JsValueValue {
            parent: PeerValue::new(),
            js_value,
        }
    }
}
