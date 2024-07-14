use crate::{
    event_target::{EventTarget, EventTargetFields},
    html_input_element::RcHtmlInputElement,
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use by_address::ByAddress;
use marshal_pointer::Rcf;
use octant_object::{cast::downcast_object, class, DebugClass};
use octant_runtime::{
    reexports::marshal_pointer::RcfRef, rpc, runtime::Runtime, DeserializePeer, PeerNew,
    SerializePeer,
};
use std::{cell::RefCell, collections::HashSet, rc::Rc};

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
}

#[rpc]
impl dyn Node {
    pub fn descendants_by_type<T: 'static + ?Sized>(self: &RcfRef<Self>) -> Vec<Rcf<T>> {
        let mut output = vec![];
        self.descendants_by_type_impl(&mut output);
        output
    }
    fn descendants_by_type_impl<T: 'static + ?Sized>(
        self: &RcfRef<Self>,
        output_vec: &mut Vec<Rcf<T>>,
    ) {
        if let Ok(output) = downcast_object::<_, Rcf<T>>(self.strong()) {
            output_vec.push(output);
        }
        for child in self.children.borrow().iter() {
            child.descendants_by_type_impl(output_vec);
        }
    }
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
