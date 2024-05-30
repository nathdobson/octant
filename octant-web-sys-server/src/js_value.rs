use crate::any_value::AnyValueValue;
use octant_object::{class, DebugClass};
use octant_runtime::{
    peer::{Peer, PeerValue},
    DeserializePeer, PeerNew, SerializePeer,
};

#[derive(DebugClass, SerializePeer, DeserializePeer)]
pub struct JsValueValue {
    parent: PeerValue,
    #[cfg(side = "client")]
    any_value: wasm_bindgen::JsValue,
}

#[class]
pub trait JsValue: Peer {}

#[cfg(side = "client")]
impl PeerNew for JsValueValue {
    type Builder = wasm_bindgen::JsValue;
    fn peer_new(any_value: wasm_bindgen::JsValue) -> Self {
        JsValueValue {
            parent: PeerValue::new(),
            any_value,
        }
    }
}

#[cfg(side = "server")]
impl PeerNew for JsValueValue {
    type Builder = PeerValue;
    fn peer_new(handle: PeerValue) -> Self {
        JsValueValue { parent: handle }
    }
}
