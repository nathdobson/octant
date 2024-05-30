use octant_object::{class, DebugClass};
use serde::Serialize;

use octant_runtime::{peer::PeerValue, DeserializePeer, PeerNew, SerializePeer};

use crate::html_element::{HtmlElement, HtmlElementValue};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlDivElementValue {
    parent: HtmlElementValue,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlDivElement,
}

#[class]
pub trait HtmlDivElement: HtmlElement {}
