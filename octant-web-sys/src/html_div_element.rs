use octant_object::define_class;
use serde::{Deserialize, Serialize};

#[cfg(side = "client")]
use octant_gui_client::html_element::{HtmlElement, HtmlElementValue};

#[cfg(side = "server")]
use octant_gui::html_element::HtmlElement;

use octant_gui_core::{define_sys_class, HandleId, TypeTag};

define_sys_class!{
    class HtmlDivElement;
    extends HtmlElement;
    wasm web_sys::HtmlDivElement;
}