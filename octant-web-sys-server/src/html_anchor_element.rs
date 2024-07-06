use crate::html_element::{HtmlElement, HtmlElementFields};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use std::rc::Rc;
use crate::octant_runtime::peer::AsNative;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlAnchorElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlAnchorElement,
}

#[class]
pub trait HtmlAnchorElement: HtmlElement {}

#[rpc]
impl dyn HtmlAnchorElement {
    #[rpc]
    pub fn set_href(self: &RcfRef<Self>, _: &Rc<Runtime>, href: String) -> () {
        self.native().set_attribute("href", &href)?;
        Ok(())
    }
}
