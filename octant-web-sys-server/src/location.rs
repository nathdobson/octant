use std::rc::Rc;
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, rpc, SerializePeer};
use octant_runtime::octant_future::OctantFuture;
use octant_runtime::runtime::Runtime;
use crate::node::{Node, NodeFields};
use crate::object::{Object, ObjectFields};
use crate::octant_runtime::peer::AsNative;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct LocationFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    wasm: web_sys::Location,
}

#[class]
pub trait Location: Object {

}

#[rpc]
impl dyn Location{
    #[rpc]
    pub fn href(self: &RcfRef<Self>, runtime: &Rc<Runtime>) -> OctantFuture<String> {
        let this = self.strong();
        Ok(OctantFuture::<String>::spawn(runtime, async move {
            this.native().href().clone().unwrap()
        }))
    }
    #[rpc]
    pub fn set_href(self:&RcfRef<Self>, runtime:&Rc<Runtime>, href: String){
        self.native().set_href(&href)?;
        Ok(())
    }
}
