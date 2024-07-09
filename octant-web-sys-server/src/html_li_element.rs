use crate::html_element::{HtmlElement, HtmlElementFields};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};
use std::{cell::RefCell, rc::Rc};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlLiElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    any_value: web_sys::HtmlLiElement,
}
#[class]
pub trait HtmlLiElement: HtmlElement {}
