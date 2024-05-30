use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};

use crate::element::{Element, ElementValue};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlElementValue {
    parent: ElementValue,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlElement,
}
#[class]
pub trait HtmlElement: Element {}
