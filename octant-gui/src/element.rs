use atomic_refcell::AtomicRefCell;

use octant_gui_core::element::{ElementMethod, ElementTag};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::{handle, node, Node};
use crate::runtime::HasTypedHandle;

struct State {
    children: Vec<Node>,
}

define_class! {
    pub class : node {
        state: AtomicRefCell<State>,
    }
}

impl HasTypedHandle for Value {
    type TypeTag = ElementTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: node::Value::new(handle),
            state: AtomicRefCell::new(State { children: vec![] }),
        }
    }
}

impl Value {
    fn invoke(&self, method: ElementMethod) -> handle::Value {
        (**self).invoke(Method::Element(self.typed_handle(), method))
    }
    pub fn append_child(&self, child: Node) {
        self.invoke(ElementMethod::AppendChild(child.typed_handle()));
        self.state.borrow_mut().children.push(child);
    }
    pub fn set_attribute(&self, name: &str, value: &str) {
        self.invoke(ElementMethod::SetAttribute(
            name.to_string(),
            value.to_string(),
        ));
    }
}
