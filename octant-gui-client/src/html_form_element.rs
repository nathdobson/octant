use web_sys::HtmlFormElement;

use octant_gui_core::HandleId;
use octant_gui_core::html_form_element::HtmlFormElementTag;
use octant_object::define_class;

use crate::{HasLocalType, html_element};

define_class! {
    pub class extends html_element {
        html_form_element: HtmlFormElement,
    }
}

impl Value {
    pub fn new(handle: HandleId, html_form_element: HtmlFormElement) -> Self {
        Value {
            parent: html_element::Value::new(handle, html_form_element.clone().into()),
            html_form_element,
        }
    }
}

impl HasLocalType for HtmlFormElementTag {
    type Local = dyn Trait;
}
