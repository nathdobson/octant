use atomic_refcell::AtomicRefCell;

use octant_gui_core::{ElementMethod, ElementTag};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::{handle, node};
use crate::handle::HandleValue;
use crate::node::{Node, NodeValue};
use crate::runtime::HasTypedHandle;

#[derive(Debug)]
struct State {}

define_class! {
    #[derive(Debug)]
    pub class Element extends Node {
        state: AtomicRefCell<State>,
    }
}

impl HasTypedHandle for ElementValue {
    type TypeTag = ElementTag;
}

impl ElementValue {
    pub fn new(handle: HandleValue) -> Self {
        ElementValue {
            parent: NodeValue::new(handle),
            state: AtomicRefCell::new(State {}),
        }
    }
}

impl ElementValue {
    fn invoke(&self, method: ElementMethod) -> HandleValue {
        (**self).invoke(Method::Element(self.typed_handle(), method))
    }
    pub fn set_attribute(&self, name: &str, value: &str) {
        self.invoke(ElementMethod::SetAttribute(
            name.to_string(),
            value.to_string(),
        ));
    }
}
