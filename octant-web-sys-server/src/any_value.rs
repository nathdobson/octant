use octant_object::{class, DebugClass};
use octant_runtime::{
    peer::{Peer, PeerValue},
    DeserializePeer, PeerNew, SerializePeer,
};

#[derive(DebugClass, SerializePeer, DeserializePeer)]
pub struct AnyValueValue {
    parent: PeerValue,
    #[cfg(side = "client")]
    any_value: wasm_bindgen::JsValue,
}

#[class]
pub trait AnyValue: Peer {}

#[cfg(side = "client")]
impl PeerNew for AnyValueValue {
    type Builder = wasm_bindgen::JsValue;
    fn peer_new(any_value: wasm_bindgen::JsValue) -> Self {
        AnyValueValue {
            parent: PeerValue::new(),
            any_value,
        }
    }
}

#[cfg(side = "server")]
impl PeerNew for AnyValueValue {
    type Builder = PeerValue;
    fn peer_new(handle: PeerValue) -> Self {
        AnyValueValue { parent: handle }
    }
}
