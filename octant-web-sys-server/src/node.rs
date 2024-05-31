use std::{cell::RefCell, collections::HashSet};

use by_address::ByAddress;
use octant_object::{class, DebugClass};

use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_rpc, peer::AsNative, DeserializePeer, PeerNew, SerializePeer};

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
        append_child(self.runtime(), self.rc(), e);
    }
    #[cfg(side = "server")]
    fn remove_child(self: &RcRef<Self>, e: RcNode) {
        self.node()
            .children
            .borrow_mut()
            .remove(&ByAddress(e.clone()));
        remove_child(self.runtime(), self.rc(), e);
    }
}

define_sys_rpc! {
    fn append_child(_runtime:_, this: RcNode, add:RcNode) -> () {
        this.node().children.borrow_mut().insert(ByAddress(add.clone()));
        this.native().append_child(add.native()).unwrap();
        Ok(())
    }
}

define_sys_rpc! {
    fn remove_child(_runtime:_, this: RcNode, add:RcNode) -> () {
        this.node().children.borrow_mut().remove(&ByAddress(add.clone()));
        this.native().remove_child(add.native()).unwrap();
        Ok(())
    }
}
