use std::fmt::Debug;
use std::sync::OnceLock;

use octant_gui_core::{DocumentMethod, DocumentTag};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::element::{ArcElement, ElementValue};
use crate::handle::HandleValue;
use crate::html_element::{ArcHtmlElement, HtmlElementValue};
use crate::html_form_element::{ArcHtmlFormElement, HtmlFormElementValue};
use crate::html_input_element::{ArcHtmlInputElement, HtmlInputElementValue};
use crate::node::{Node, NodeValue};
use crate::runtime::HasTypedHandle;
use crate::text::{ArcText, TextValue};

define_class! {
    #[derive(Debug)]
    pub class Document extends Node {
        body: OnceLock<ArcHtmlElement>,
    }
}

impl HasTypedHandle for DocumentValue {
    type TypeTag = DocumentTag;
}

impl DocumentValue {
    pub fn new(handle: HandleValue) -> Self {
        DocumentValue {
            parent: NodeValue::new(handle),
            body: OnceLock::new(),
        }
    }
    fn invoke(&self, method: DocumentMethod) -> HandleValue {
        (**self).invoke(Method::Document(self.typed_handle(), method))
    }
    pub fn body(&self) -> &ArcHtmlElement {
        self.body.get_or_init(|| {
            self.runtime()
                .add(HtmlElementValue::new(self.invoke(DocumentMethod::Body)))
        })
    }
    pub fn create_text_node(&self, text: &str) -> ArcText {
        self.runtime().add(TextValue::new(
            self.invoke(DocumentMethod::CreateTextNode(text.to_string())),
        ))
    }
    pub fn create_element(&self, tag: &str) -> ArcElement {
        self.runtime().add(ElementValue::new(
            self.invoke(DocumentMethod::CreateElement(tag.to_string())),
        ))
    }
    pub fn create_form_element(&self) -> ArcHtmlFormElement {
        self.runtime().add(HtmlFormElementValue::new(
            self.invoke(DocumentMethod::CreateFormElement),
        ))
    }
    pub fn create_input_element(&self) -> ArcHtmlInputElement {
        self.runtime().add(HtmlInputElementValue::new(
            self.invoke(DocumentMethod::CreateInputElement),
        ))
    }
}
