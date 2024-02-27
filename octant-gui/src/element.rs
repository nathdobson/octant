use atomic_refcell::AtomicRefCell;

use octant_gui_core::element::{ElementMethod, ElementTag};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::{handle, node};
use crate::runtime::HasTypedHandle;

#[derive(Debug)]
struct State {
}

define_class! {
    #[derive(Debug)]
    pub class extends  node {
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
            state: AtomicRefCell::new(State {  }),
        }
    }
}

impl Value {
    fn invoke(&self, method: ElementMethod) -> handle::Value {
        (**self).invoke(Method::Element(self.typed_handle(), method))
    }
    pub fn set_attribute(&self, name: &str, value: &str) {
        self.invoke(ElementMethod::SetAttribute(
            name.to_string(),
            value.to_string(),
        ));
    }
}
