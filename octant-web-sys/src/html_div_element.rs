use octant_object::define_class;
use serde::{Deserialize, Serialize};

#[cfg(side = "client")]
use octant_gui_client::html_element::{HtmlElement, HtmlElementValue};

#[cfg(side = "server")]
use octant_gui::html_element::HtmlElement;

use octant_gui_core::{HandleId, TypeTag};

#[cfg(side = "client")]
define_class! {
    pub class HtmlDivElement extends HtmlElement {
        html_div_element: web_sys::HtmlDivElement,
    }
}

#[cfg(side = "client")]
impl HtmlDivElementValue {
    pub fn new(handle: HandleId, html_div_element: web_sys::HtmlDivElement) -> Self {
        HtmlDivElementValue {
            parent: HtmlElementValue::new(handle, html_div_element.clone().into()),
            html_div_element,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct HtmlDivElementTag;

impl TypeTag for HtmlDivElementTag {}

#[cfg(side = "server")]
define_class! {
    #[derive(Debug)]
    pub class HtmlDivElement extends HtmlElement {

    }
}
