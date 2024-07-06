use std::cell::RefCell;
use std::collections::HashSet;
use by_address::ByAddress;
use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};
use crate::js_value::{JsValue, JsValueFields};
use crate::object::{Object, ObjectFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct NullFields {
    parent: JsValueFields,
    #[cfg(side = "client")]
    value: wasm_bindgen::JsValue,
}

#[class]
pub trait Null: JsValue {
}