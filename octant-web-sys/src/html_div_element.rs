use octant_object::define_class;
use serde::{Deserialize, Serialize};
use crate::html_element::HtmlElement;
use octant_gui_core::{define_sys_class, HandleId, TypeTag};

define_sys_class!{
    class HtmlDivElement;
    extends HtmlElement;
    wasm web_sys::HtmlDivElement;
}