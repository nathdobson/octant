use std::sync::Arc;

use octant_gui_core::{ElementMethod, ElementTag, HandleId};
use octant_object::define_class;

use crate::{
    HasLocalType
    ,
    node::{Node, NodeValue},
    peer::ArcPeer, Runtime,
};

define_class! {
    pub class Element extends Node {
        element: web_sys::Element,
    }
}

impl ElementValue {
    pub fn new(handle: HandleId, element: web_sys::Element) -> Self {
        ElementValue {
            parent: NodeValue::new(handle, element.clone().into()),
            element,
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
