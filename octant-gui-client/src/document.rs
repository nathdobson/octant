use std::sync::Arc;

use wasm_bindgen::JsCast;

use octant_gui_core::{DocumentMethod, DocumentTag, HandleId};
use octant_object::define_class;

use crate::{
    element::{ArcElement, ElementValue}
    ,
    HasLocalType
    ,
    html_element::{ArcHtmlElement, HtmlElementValue}
    ,
    html_form_element::{ArcHtmlFormElement, HtmlFormElementValue}
    ,
    html_input_element::{ArcHtmlInputElement, HtmlInputElementValue}
    ,
    node::{Node, NodeValue}
    ,
    peer::ArcPeer,
    text::{ArcText, TextValue},
};

define_class! {
    pub class Document extends Node {
        document: web_sys::Document,
    }
}

impl DocumentValue {
    pub fn new(handle: HandleId, document: web_sys::Document) -> Self {
        DocumentValue {
            parent: NodeValue::new(handle, document.clone().into()),
            document,
        }
    }
    pub fn body(&self, handle: HandleId) -> ArcHtmlElement {
        Arc::new(HtmlElementValue::new(handle, self.document.body().unwrap()))
    }
    pub fn create_text_node(&self, handle: HandleId, content: &str) -> ArcText {
        Arc::new(TextValue::new(
            handle,
            self.document.create_text_node(content),
        ))
    }
    pub fn create_element(&self, handle: HandleId, tag: &str) -> ArcElement {
        Arc::new(ElementValue::new(
            handle,
            self.document.create_element(tag).unwrap(),
        ))
    }
    pub fn create_form_element(&self, handle: HandleId) -> ArcHtmlFormElement {
        Arc::new(HtmlFormElementValue::new(
            handle,
            self.document
                .create_element("form")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    pub fn create_input_element(&self, handle: HandleId) -> ArcHtmlInputElement {
        Arc::new(HtmlInputElementValue::new(
            handle,
            self.document
                .create_element("input")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    pub fn handle_with(&self, method: &DocumentMethod, handle: HandleId) -> Option<ArcPeer> {
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
    type Local = dyn Document;
}
