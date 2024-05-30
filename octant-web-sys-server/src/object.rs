use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::js_value::{JsValue, JsValueValue};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ObjectValue {
    parent: JsValueValue,
    #[cfg(side = "client")]
    any_value: js_sys::Object,
}

#[class]
pub trait Object: JsValue {}
