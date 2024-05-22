use std::sync::Arc;

use serde::Serialize;
use octant_runtime::define_sys_class;
use crate::html_element::HtmlElement;

define_sys_class! {
    class HtmlInputElement;
    extends HtmlElement;
    wasm web_sys::HtmlInputElement;
    new_client _;
    new_server _;
}

impl HtmlInputElementValue {
    pub fn input_value(&self) -> Arc<String> {
        todo!();
    }
}
