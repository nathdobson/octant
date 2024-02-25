use std::sync::Arc;

use wasm_bindgen::JsCast;
use web_sys::Document;

use octant_gui_core::document::{DocumentMethod, DocumentTag};
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{
    element, HasLocalType, html_element, html_form_element, html_input_element, node, peer, text,
};

define_class! {
    pub class extends node {
        document: Document,
    }
}

impl Value {
    pub fn new(handle: HandleId, document: Document) -> Self {
        Value {
            parent: node::Value::new(handle, document.clone().into()),
            document,
        }
    }
    pub fn body(&self, handle: HandleId) -> Arc<dyn html_element::Trait> {
        Arc::new(html_element::Value::new(
            handle,
            self.document.body().unwrap(),
        ))
    }
    pub fn create_text_node(&self, handle: HandleId, content: &str) -> Arc<dyn text::Trait> {
        Arc::new(text::Value::new(
            handle,
            self.document.create_text_node(content),
        ))
    }
    pub fn create_element(&self, handle: HandleId, tag: &str) -> Arc<dyn element::Trait> {
        Arc::new(element::Value::new(
            handle,
            self.document.create_element(tag).unwrap(),
        ))
    }
    pub fn create_form_element(&self, handle: HandleId) -> Arc<dyn html_form_element::Trait> {
        Arc::new(html_form_element::Value::new(
            handle,
            self.document
                .create_element("form")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    pub fn create_input_element(&self, handle: HandleId) -> Arc<dyn html_input_element::Trait> {
        Arc::new(html_input_element::Value::new(
            handle,
            self.document
                .create_element("input")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    pub fn handle_with(
        &self,
        method: &DocumentMethod,
        handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            DocumentMethod::Body => Some(self.body(handle)),
            DocumentMethod::CreateTextNode(text) => Some(self.create_text_node(handle, text)),
            DocumentMethod::CreateElement(tag) => Some(self.create_element(handle, tag)),
            DocumentMethod::CreateFormElement => Some(self.create_form_element(handle)),
            DocumentMethod::CreateInputElement => Some(self.create_input_element(handle)),
        }
    }
}

impl HasLocalType for DocumentTag {
    type Local = dyn Trait;
}
