use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use web_sys::Element;

use octant_gui_core::element::{ElementMethod, ElementTag};
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{HasLocalType, node, peer, Runtime};

struct State {
    children: Vec<Arc<dyn node::Trait>>,
}

define_class! {
    pub class extends node {
        element: Element,
        state:AtomicRefCell<State>,
    }
}

impl Value {
    pub fn new(handle: HandleId, element: Element) -> Self {
        Value {
            parent: node::Value::new(handle, element.clone().into()),
            element,
            state: AtomicRefCell::new(State { children: vec![] }),
        }
    }
    pub fn native(&self) -> &Element {
        &self.element
    }
    pub fn children(&self) -> Vec<Arc<dyn node::Trait>> {
        self.state.borrow_mut().children.clone()
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
                self.state.borrow_mut().children.push(node.clone());
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
