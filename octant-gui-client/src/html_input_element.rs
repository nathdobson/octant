use std::marker::PhantomData;
use std::sync::Arc;

use web_sys::HtmlInputElement;

use octant_gui_core::{HandleId, TypedHandle};
use octant_gui_core::html_input_element::{
    HtmlInputElementMethod, HtmlInputElementTag,
};
use octant_object::define_class;

use crate::{HasLocalType, html_element, peer, Runtime};

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

impl dyn Trait {
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: Arc<Runtime>,
        method: &HtmlInputElementMethod,
        _handle_id: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            HtmlInputElementMethod::Clear => {
                self.html_input_element.set_value("");
                None
            }
        }
    }
}

impl HasLocalType for HtmlInputElementTag {
    type Local = dyn Trait;
}
