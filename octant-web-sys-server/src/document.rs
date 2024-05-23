use octant_runtime::{define_sys_class, define_sys_rpc};
use std::sync::Arc;
use safe_once::sync::OnceLock;
use crate::html_div_element::HtmlDivElementValue;
use crate::text::Text;
use crate::text::TextValue;
use crate::node::NodeValue;
use crate::html_element::{ArcHtmlElement, HtmlElement};
use crate::html_element::HtmlElementValue;

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
    new_client _;
    new_server _;
    server_field body: OnceLock<ArcHtmlElement>;
}

#[cfg(side = "server")]
impl dyn Document {
    pub fn create_div(self: &Arc<Self>) -> ArcHtmlDivElement {
        create_div(self.runtime(), self.clone())
    }
    pub fn create_text_node(self: &Arc<Self>, content: String) -> ArcText {
        create_text_node(self.runtime(), self.clone(), content)
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
    pub fn body<'a>(self: &'a Arc<Self>) -> &'a ArcHtmlElement {
        self.body.get_or_init(||{
            body(self.runtime(), self.clone())
        })
    }
}

define_sys_rpc! {
    fn create_div(_runtime, document: Arc<dyn Document>) -> (HtmlDivElement, ) {
        Ok((Arc::new(HtmlDivElementValue::new(document.native().create_element("div").unwrap().dyn_into().unwrap())), ))
    }
}

define_sys_rpc! {
    fn create_text_node(_runtime, document: Arc<dyn Document>, text: String) -> (Text, ) {
        Ok((Arc::new(TextValue::new(document.native().create_text_node(&text).dyn_into().unwrap())), ))
    }
}

define_sys_rpc! {
    fn body(_runtime, document: Arc<dyn Document>) -> (HtmlElement, ) {
        Ok((Arc::new(HtmlElementValue::new(document.native().body().unwrap())), ))
    }
}
