use crate::{node::Node, octant_runtime::peer::AsNative};
use octant_object::class;
use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_rpc, DeserializePeer, PeerNew, SerializePeer};

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Element {
    parent: dyn Node,
    #[cfg(side = "client")]
    wasm: web_sys::Element,
}

pub trait Element: AsElement {
    #[cfg(side = "server")]
    fn set_attribute(self: &RcRef<Self>, key: &str, value: &str);
}

impl<T> Element for T
where
    T: AsElement,
{
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
