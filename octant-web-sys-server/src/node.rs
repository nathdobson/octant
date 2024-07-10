use by_address::ByAddress;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    event_target::{EventTarget, EventTargetFields},
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use octant_object::{cast::downcast_object, class, DebugClass};
use octant_runtime::{
    reexports::marshal_pointer::RcfRef, rpc, runtime::Runtime, DeserializePeer, PeerNew,
    SerializePeer,
};
use crate::html_input_element::RcHtmlInputElement;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct NodeFields {
    parent: EventTargetFields,
    #[cfg(side = "client")]
    any_value: web_sys::Node,
    children: RefCell<HashSet<ByAddress<RcNode>>>,
}

#[class]
pub trait Node: EventTarget {
    fn children(self: &RcfRef<Self>) -> Vec<RcNode> {
        self.node()
            .children
            .borrow()
            .iter()
            .map(|x| x.0.clone())
            .collect()
    }
    #[cfg(side = "server")]
    fn append_child(self: &RcfRef<Self>, e: RcNode) {
        self.node()
            .children
            .borrow_mut()
            .insert(ByAddress(e.clone()));
        self.append_child_impl(e);
    }
    #[cfg(side = "server")]
    fn remove_child(self: &RcfRef<Self>, e: RcNode) {
        self.node()
            .children
            .borrow_mut()
            .remove(&ByAddress(e.clone()));
        self.remove_child_impl(e);
    }
    #[cfg(side = "client")]
    fn update_input_values_rec(self: &RcfRef<Self>) {
        if let Ok(input) = downcast_object::<_, RcHtmlInputElement>(self.strong()) {
            input.update_input_value();
        }
        for child in self.children() {
            child.update_input_values_rec();
        }
    }
}

#[rpc]
impl dyn Node {
    #[rpc]
    fn append_child_impl(self: &RcfRef<Self>, _: &Rc<Runtime>, add: RcNode) -> () {
        self.node()
            .children
            .borrow_mut()
            .insert(ByAddress(add.clone()));
        self.native().append_child(add.native()).unwrap();
        Ok(())
    }
    #[rpc]
    fn remove_child_impl(self: &RcfRef<Self>, _: &Rc<Runtime>, add: RcNode) -> () {
        self.node()
            .children
            .borrow_mut()
            .remove(&ByAddress(add.clone()));
        self.native().remove_child(add.native()).unwrap();
        Ok(())
    }
}
