use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::object::{Object, ObjectFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct RequestInitFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::RequestInit,
}

#[class]
pub trait RequestInit: Object {}
