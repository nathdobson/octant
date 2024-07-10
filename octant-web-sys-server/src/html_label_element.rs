use crate::{
    html_element::{HtmlElement, HtmlElementFields},
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};
use safe_once::cell::OnceCell;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlLabelElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    native: web_sys::HtmlLabelElement,
}

#[class]
pub trait HtmlLabelElement: HtmlElement {}
