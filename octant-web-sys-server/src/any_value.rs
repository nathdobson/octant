use octant_object::class;
use octant_runtime::{
    define_sys_class,
    peer::{Peer, PeerValue},
    DeserializePeer, PeerNew, SerializePeer,
};

#[class]
#[derive(SerializePeer, DeserializePeer)]
pub struct AnyValue {
    parent: dyn Peer,
    #[cfg(side = "client")]
    any_value: wasm_bindgen::JsValue,
}

pub trait AnyValue: AsAnyValue {}

impl<T> AnyValue for T where T: AsAnyValue {}

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
