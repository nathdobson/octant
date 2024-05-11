use std::sync::Arc;

use atomic_refcell::AtomicRefCell;

use octant_gui_core::{ElementMethod, ElementTag, HandleId};
use octant_object::define_class;

use crate::{
    node,
    node::{Node, NodeValue},
    peer,
    peer::ArcPeer,
    HasLocalType, Runtime,
};

struct State {}

define_class! {
    pub class Element extends Node {
        element: web_sys::Element,
        state: AtomicRefCell<State>,
    }
}

impl ElementValue {
    pub fn new(handle: HandleId, element: web_sys::Element) -> Self {
        ElementValue {
            parent: NodeValue::new(handle, element.clone().into()),
            element,
            state: AtomicRefCell::new(State {}),
        }
    }
    pub fn native(&self) -> &web_sys::Element {
        &self.element
    }
    pub fn invoke_with(
        &self,
        _runtime: &Arc<Runtime>,
        method: &ElementMethod,
        _handle: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            ElementMethod::SetAttribute(name, value) => {
                self.native().set_attribute(&name, &value).unwrap();
                None
            }
        }
    }
}

impl HasLocalType for ElementTag {
    type Local = dyn Element;
}
