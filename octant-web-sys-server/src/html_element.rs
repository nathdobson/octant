use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::element::{Element, ElementFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlElementFields {
    parent: ElementFields,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlElement,
}
#[class]
pub trait HtmlElement: Element {}
