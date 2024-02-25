use std::fmt::{Debug, Formatter};
use std::sync::OnceLock;

use octant_gui_core::document::{DocumentMethod, DocumentTag};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::runtime::HasTypedHandle;
use crate::{element, handle, html_element, html_form_element, node, text, Element, HtmlElement, HtmlFormElement, Text, html_input_element, HtmlInputElement};

define_class! {
    #[derive(Debug)]
    pub class extends node {
        body: OnceLock<HtmlElement>,
    }
}

impl HasTypedHandle for Value {
    type TypeTag = DocumentTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: node::Value::new(handle),
            body: OnceLock::new(),
        }
    }
    fn invoke(&self, method: DocumentMethod) -> handle::Value {
        (**self).invoke(Method::Document(self.typed_handle(), method))
    }
    pub fn body(&self) -> &HtmlElement {
        self.body.get_or_init(|| {
            self.runtime()
                .add(html_element::Value::new(self.invoke(DocumentMethod::Body)))
        })
    }
    pub fn create_text_node(&self, text: &str) -> Text {
        self.runtime().add(text::Value::new(
            self.invoke(DocumentMethod::CreateTextNode(text.to_string())),
        ))
    }
    pub fn create_element(&self, tag: &str) -> Element {
        self.runtime().add(element::Value::new(
            self.invoke(DocumentMethod::CreateElement(tag.to_string())),
        ))
    }
    pub fn create_form_element(&self) -> HtmlFormElement {
        self.runtime().add(html_form_element::Value::new(
            self.invoke(DocumentMethod::CreateFormElement),
        ))
    }
    pub fn create_input_element(&self) -> HtmlInputElement {
        self.runtime().add(html_input_element::Value::new(
            self.invoke(DocumentMethod::CreateInputElement),
        ))
    }
}
