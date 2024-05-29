use serde::Serialize;

use octant_runtime::define_sys_class;

use crate::html_element::HtmlElement;

define_sys_class! {
    class HtmlDivElement;
    extends HtmlElement;
    wasm web_sys::HtmlDivElement;
    new_client _;
    new_server _;
}
