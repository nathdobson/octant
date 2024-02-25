use web_sys::HtmlElement;

use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::element;

define_class! {
    pub class extends element {
        element: HtmlElement,
    }
}

impl Value {
    pub fn new(handle: HandleId, element: HtmlElement) -> Self {
        Value {
            parent: element::Value::new(handle, element.clone().into()),
            element,
        }
    }
}
