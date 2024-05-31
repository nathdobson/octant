use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::{node::Node, object::Object};
use crate::node::NodeFields;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct TextFields {
    parent: NodeFields,
    #[cfg(side = "client")]
    any_value: web_sys::Text,
}

#[class]
pub trait Text: Node {}
