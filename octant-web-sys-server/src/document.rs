use std::sync::Arc;

#[cfg(side = "client")]
use wasm_bindgen::JsCast;

use octant_gui_core::{define_sys_class, define_sys_rpc};

use crate::{
    html_div_element::{ArcHtmlDivElement, HtmlDivElement},
    node::Node,
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
}

define_sys_rpc! {
    fn create_div(_runtime, document: Arc<dyn Document>) -> (HtmlDivElement, ) {
        Ok((document.native().create_element("div").unwrap().dyn_into().unwrap(), ))
    }
}
