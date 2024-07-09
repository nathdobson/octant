use crate::{
    css_style_declaration::RcCssStyleDeclaration,
    element::{Element, ElementFields},
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use safe_once::cell::OnceCell;
use std::rc::Rc;
use crate::css_style_declaration::CssStyleDeclaration;
use crate::octant_runtime::peer::AsNative;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlElementFields {
    parent: ElementFields,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlElement,
    #[cfg(side = "server")]
    style: OnceCell<RcCssStyleDeclaration>,
}

#[class]
pub trait HtmlElement: Element {
    #[cfg(side = "server")]
    fn style<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn CssStyleDeclaration> {
        &**self.style.get_or_init(|| self.style_impl())
    }
}

#[rpc]
impl dyn HtmlElement {
    #[rpc]
    fn style_impl(self: &RcfRef<Self>, runtime: &Rc<Runtime>) -> RcCssStyleDeclaration {
        Ok(RcCssStyleDeclaration::peer_new(self.native().style()))
    }
}
