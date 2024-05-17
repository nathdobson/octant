use octant_gui_core::define_sys_class;
use crate::element::Element;
define_sys_class!{
    class HtmlElement;
    extends Element;
    wasm web_sys::HtmlElement;
}