use octant_runtime::{define_sys_class, define_sys_rpc};
use std::sync::Arc;
use crate::html_div_element::HtmlDivElementValue;

#[cfg(side = "client")]
use wasm_bindgen::JsCast;

use crate::{
    element::ArcElement,
    html_div_element::{ArcHtmlDivElement, HtmlDivElement},
    html_form_element::ArcHtmlFormElement,
    html_input_element::ArcHtmlInputElement,
    node::{ArcNode, Node},
    text::ArcText,
};

define_sys_class! {
    class Document;
    extends Node;
    wasm web_sys::Document;
    new_client a;
    new_server a;
}

#[cfg(side = "server")]
impl dyn Document {
    pub fn create_div(self: &Arc<Self>) -> ArcHtmlDivElement {
        create_div(self.runtime(), self.clone())
    }
    pub fn create_text_node(self: &Arc<Self>, content: &str) -> ArcText {
        todo!();
    }
    pub fn create_input_element(self: &Arc<Self>) -> ArcHtmlInputElement {
        todo!()
    }
    pub fn create_form_element(self: &Arc<Self>) -> ArcHtmlFormElement {
        todo!()
    }
    pub fn create_element(self: &Arc<Self>, tag: &str) -> ArcElement {
        todo!()
    }
    pub fn body(self: &Arc<Self>) -> ArcNode {
        todo!();
    }
}

define_sys_rpc! {
    fn create_div(_runtime, document: Arc<dyn Document>) -> (HtmlDivElement, ) {
        Ok((Arc::new(HtmlDivElementValue::new(document.native().create_element("div").unwrap().dyn_into().unwrap())), ))
    }
}
