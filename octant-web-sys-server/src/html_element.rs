use octant_gui_core::define_sys_class;

use crate::element::Element;

define_sys_class!{
    class HtmlElement;
    extends Element;
    wasm web_sys::HtmlElement;
    new_client _;
    new_server _;
}