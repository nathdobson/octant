use serde::Serialize;

use octant_gui_core::define_sys_class;

use crate::html_element::HtmlElement;

define_sys_class! {
    class HtmlFormElement;
    extends HtmlElement;
    wasm web_sys::HtmlFormElement;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl HtmlFormElementValue {
    pub(crate) fn set_handler(&self, handler: impl Fn()) {
        todo!();
    }
}
