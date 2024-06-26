use std::{cell::RefCell, collections::HashSet, rc::Rc};

use by_address::ByAddress;
use octant_object::{class, DebugClass};

use octant_reffed::rc::RcRef;
use octant_runtime::{
    peer::AsNative, rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer,
};

use crate::object::{Object, ObjectFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct NodeFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::Node,
    children: RefCell<HashSet<ByAddress<RcNode>>>,
}

#[class]
pub trait Node: Object {
    fn children(self: &RcRef<Self>) -> Vec<RcNode> {
        self.node()
            .children
            .borrow()
            .iter()
            .map(|x| x.0.clone())
            .collect()
    }
    #[cfg(side = "server")]
    fn append_child(self: &RcRef<Self>, e: RcNode) {
        self.node()
            .children
            .borrow_mut()
            .insert(ByAddress(e.clone()));
        self.append_child_impl(e);
    }
    #[cfg(side = "server")]
    fn remove_child(self: &RcRef<Self>, e: RcNode) {
        self.node()
            .children
            .borrow_mut()
            .remove(&ByAddress(e.clone()));
        self.remove_child_impl(e);
    }
}

#[rpc]
impl dyn Node {
    #[rpc]
    fn append_child_impl(self: &RcRef<Self>, _: &Rc<Runtime>, add: RcNode) -> () {
        self.node()
            .children
            .borrow_mut()
            .insert(ByAddress(add.clone()));
        self.native().append_child(add.native()).unwrap();
        Ok(())
    }
    #[rpc]
    fn remove_child_impl(self: &RcRef<Self>, _: &Rc<Runtime>, add: RcNode) -> () {
        self.node()
            .children
            .borrow_mut()
            .remove(&ByAddress(add.clone()));
        self.native().remove_child(add.native()).unwrap();
        Ok(())
    }
}
