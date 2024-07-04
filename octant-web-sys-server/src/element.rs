use std::rc::Rc;
use marshal_pointer::rc_ref::RcRef;

use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, rpc, SerializePeer};
use octant_runtime::runtime::Runtime;
use crate::node::{Node, NodeFields};
use crate::octant_runtime::peer::AsNative;
#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ElementFields {
    parent: NodeFields,
    #[cfg(side = "client")]
    wasm: web_sys::Element,
}
#[class]
pub trait Element: Node {
    #[cfg(side = "server")]
    fn set_attribute(self: &RcRef<Self>, key: &str, value: &str) {
        self.set_attribute_impl(key.to_string(), value.to_string())
    }
}

#[rpc]
impl dyn Element {
    #[rpc]
    fn set_attribute_impl(
        self: &RcRef<dyn Element>,
        _: &Rc<Runtime>,
        key: String,
        value: String,
    ) -> () {
        self.native().set_attribute(&key, &value).unwrap();
        Ok(())
    }
}
