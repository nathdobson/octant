use octant_object::{class, DebugClass};
use octant_runtime::{
    DeserializePeer,
    peer::{Peer, PeerFields}, PeerNew, SerializePeer,
};

#[derive(DebugClass, SerializePeer, DeserializePeer)]
pub struct JsValueFields {
    parent: PeerFields,
    #[cfg(side = "client")]
    any_value: wasm_bindgen::JsValue,
}

#[class]
pub trait JsValue: Peer {}

#[cfg(side = "client")]
impl PeerNew for JsValueFields {
    type Builder = wasm_bindgen::JsValue;
    fn peer_new(any_value: wasm_bindgen::JsValue) -> Self {
        JsValueFields {
            parent: PeerFields::new(),
            any_value,
        }
    }
}

#[cfg(side = "server")]
impl PeerNew for JsValueFields {
    type Builder = PeerFields;
    fn peer_new(handle: PeerFields) -> Self {
        JsValueFields { parent: handle }
    }
}
