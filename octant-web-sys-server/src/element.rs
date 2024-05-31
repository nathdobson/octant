use crate::{
    node::{Node, NodeFields},
    octant_runtime::peer::AsNative,
};
use octant_object::{class, DebugClass};
use octant_reffed::rc::RcRef;
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::rc::Rc;

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
        set_attribute(
            self.runtime(),
            self.rc(),
            key.to_string(),
            value.to_string(),
        )
    }
}

#[rpc]
fn set_attribute(_: &Rc<Runtime>, this: RcElement, key: String, value: String) -> () {
    this.native().set_attribute(&key, &value).unwrap();
    Ok(())
}
