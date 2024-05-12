use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::element::Element;
use crate::element::ElementValue;

define_class! {
    pub class HtmlElement extends Element {
        element: web_sys::HtmlElement,
    }
}

impl HtmlElementValue {
    pub fn new(handle: HandleId, element: web_sys::HtmlElement) -> Self {
        HtmlElementValue {
            parent: ElementValue::new(handle, element.clone().into()),
            element,
        }
    }
}
