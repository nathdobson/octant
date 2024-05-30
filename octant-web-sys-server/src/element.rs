use crate::{node::Node, octant_runtime::peer::AsNative};
use octant_object::{class, DebugClass};
use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_rpc, DeserializePeer, PeerNew, SerializePeer};
use crate::node::NodeValue;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ElementValue {
    parent: NodeValue,
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

define_sys_rpc! {
    fn set_attribute(_runtime:_, this: RcElement, key:String, value:String) -> () {
        this.native().set_attribute(&key, &value).unwrap();
        Ok(())
    }
}
