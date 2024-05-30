use octant_object::class;
use serde::Serialize;

use octant_runtime::{define_sys_class, peer::PeerValue, PeerNew};

use crate::html_element::{HtmlElement, HtmlElementValue};

#[class]
#[derive(PeerNew)]
pub struct HtmlDivElement {
    parent: dyn HtmlElement,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlDivElement,
}

pub trait HtmlDivElement: AsHtmlDivElement {}

impl<T> HtmlDivElement for T where T: AsHtmlDivElement {}
