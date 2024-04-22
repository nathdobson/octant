use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use web_sys::Element;

use octant_gui_core::{ElementMethod, ElementTag};
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{HasLocalType, node, peer, Runtime};

struct State {}

define_class! {
    pub class extends node {
        element: Element,
        state: AtomicRefCell<State>,
    }
}

impl Value {
    pub fn new(handle: HandleId, element: Element) -> Self {
        Value {
            parent: node::Value::new(handle, element.clone().into()),
            element,
            state: AtomicRefCell::new(State {}),
        }
    }
    pub fn native(&self) -> &Element {
        &self.element
    }
    pub fn invoke_with(
        &self,
        _runtime: &Arc<Runtime>,
        method: &ElementMethod,
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
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
