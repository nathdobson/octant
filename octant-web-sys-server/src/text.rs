use crate::{
    node::{Node, NodeFields},
    object::Object,
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::rc::Rc;

use crate::octant_runtime::peer::AsNative;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct TextFields {
    parent: NodeFields,
    #[cfg(side = "client")]
    any_value: web_sys::Text,
}

#[class]
pub trait Text: Node {}

#[rpc]
impl dyn Text {
    #[rpc]
    pub fn set_node_value(self: &RcfRef<Self>, _: &Rc<Runtime>, value: String) {
        self.native().set_node_value(Some(&value));
        Ok(())
    }
}
