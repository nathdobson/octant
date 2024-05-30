use octant_object::class;
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::element::Element;

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlElement {
    parent: dyn Element,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlElement,
}

pub trait HtmlElement: AsHtmlElement {}

impl<T> HtmlElement for T where T: AsHtmlElement {}
