use std::marker::PhantomData;
use web_sys::{HtmlInputElement, Node};

use octant_gui_core::html_form_element::HtmlFormElementTag;
use octant_gui_core::html_input_element::HtmlInputElementTag;
use octant_gui_core::node::NodeTag;
use octant_gui_core::{HandleId, TypedHandle};
use octant_object::define_class;

use crate::{html_element, object, HasLocalType};

define_class! {
    pub class extends html_element {
        html_input_element: HtmlInputElement,
    }
}

impl Value {
    pub fn new(handle: HandleId, html_input_element: HtmlInputElement) -> Self {
        Value {
            parent: html_element::Value::new(handle, html_input_element.clone().into()),
            html_input_element,
        }
    }
    pub fn native(&self) -> &HtmlInputElement {
        &self.html_input_element
    }
    pub fn handle(&self) -> TypedHandle<HtmlInputElementTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl HasLocalType for HtmlInputElementTag {
    type Local = dyn Trait;
}
