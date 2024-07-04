use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::html_element::{HtmlElement, HtmlElementFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlDivElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlDivElement,
}

#[class]
pub trait HtmlDivElement: HtmlElement {}
