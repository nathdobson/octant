use std::{marker::PhantomData, sync::Arc};

use octant_gui_core::{HandleId, HtmlInputElementMethod, HtmlInputElementTag, TypedHandle};
use octant_object::define_class;

use crate::{
    html_element,
    html_element::{HtmlElement, HtmlElementValue},
    peer,
    peer::ArcPeer,
    HasLocalType, Runtime,
};

define_class! {
    pub class HtmlInputElement extends HtmlElement {
        html_input_element: web_sys::HtmlInputElement,
    }
}

impl HtmlInputElementValue {
    pub fn new(handle: HandleId, html_input_element: web_sys::HtmlInputElement) -> Self {
        HtmlInputElementValue {
            parent: HtmlElementValue::new(handle, html_input_element.clone().into()),
            html_input_element,
        }
    }
    pub fn native(&self) -> &web_sys::HtmlInputElement {
        &self.html_input_element
    }
    pub fn handle(&self) -> TypedHandle<HtmlInputElementTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn HtmlInputElement {
    pub fn invoke_with(
        self: Arc<Self>,
        _runtime: Arc<Runtime>,
        method: &HtmlInputElementMethod,
        _handle_id: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            HtmlInputElementMethod::Clear => {
                self.html_input_element.set_value("");
                None
            }
        }
    }
}

impl HasLocalType for HtmlInputElementTag {
    type Local = dyn HtmlInputElement;
}
