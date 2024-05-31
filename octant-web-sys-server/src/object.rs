use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::js_value::{JsValue, JsValueFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ObjectFields {
    parent: JsValueFields,
    #[cfg(side = "client")]
    any_value: js_sys::Object,
}

#[class]
pub trait Object: JsValue {}
