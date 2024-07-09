use crate::object::{Object, ObjectFields};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::rc::Rc;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct DomTokenListFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    native: web_sys::DomTokenList,
}

#[class]
pub trait DomTokenList: Object {
    #[cfg(side = "server")]
    fn add(self: &RcfRef<Self>, token: &str) {
        self.add_impl(token.to_owned());
    }
    #[cfg(side = "server")]
    fn remove(self: &RcfRef<Self>, token: &str) {
        self.remove_impl(token.to_owned());
    }
}

#[rpc]
impl dyn DomTokenList {
    #[rpc]
    pub fn add_impl(self: &RcfRef<Self>, _: &Rc<Runtime>, token: String) {
        self.native.add_1(&token)?;
        Ok(())
    }
    #[rpc]
    pub fn remove_impl(self: &RcfRef<Self>, _: &Rc<Runtime>, token: String) {
        self.native.remove_1(&token)?;
        Ok(())
    }
}
