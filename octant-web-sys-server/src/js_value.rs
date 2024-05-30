use crate::any_value::{AnyValueValue, AsAnyValue};
use octant_object::class;
use octant_runtime::{
    define_sys_class,
    peer::{Peer, PeerValue},
    DeserializePeer, PeerNew, SerializePeer,
};

#[class]
#[derive(SerializePeer, DeserializePeer)]
pub struct JsValue {
    parent: dyn Peer,
    #[cfg(side = "client")]
    any_value: wasm_bindgen::JsValue,
}

pub trait JsValue: AsJsValue {}

impl<T> JsValue for T where T: AsJsValue {}

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
