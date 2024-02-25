use std::sync::Arc;

use web_sys::Element;

use octant_gui_core::element::{ElementMethod, ElementTag};
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{HasLocalType, node, peer, Runtime};

define_class! {
    pub class extends node {
        element: Element,
    }
}

impl Value {
    pub fn new(handle: HandleId, element: Element) -> Self {
        Value {
            parent: node::Value::new(handle, element.clone().into()),
            element,
        }
    }
    pub fn native(&self) -> &Element {
        &self.element
    }
    pub fn invoke_with(
        &self,
        runtime: &Arc<Runtime>,
        method: &ElementMethod,
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            ElementMethod::AppendChild(node) => {
                let node = runtime.handle(*node);
                self.native().append_child(node.native()).unwrap();
                None
            }
            ElementMethod::SetAttribute(name, value) => {
                self.native().set_attribute(&name, &value).unwrap();
                None
            }
        }
    }
}

impl HasLocalType for ElementTag {
    type Local = dyn Trait;
}
