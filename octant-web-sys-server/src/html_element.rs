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
use crate::dom_token_list::{DomTokenList, RcDomTokenList};
use crate::octant_runtime::peer::AsNative;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlElementFields {
    parent: ElementFields,
    #[cfg(side = "client")]
    wasm: web_sys::HtmlElement,
    #[cfg(side = "server")]
    style: OnceCell<RcCssStyleDeclaration>,
    #[cfg(side = "server")]
    class_list: OnceCell<RcDomTokenList>,
}

#[class]
pub trait HtmlElement: Element {
    #[cfg(side = "server")]
    fn style<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn CssStyleDeclaration> {
        &**self.style.get_or_init(|| self.style_impl())
    }
    #[cfg(side="server")]
    fn class_list<'a>(self:&'a RcfRef<Self>)->&'a RcfRef<dyn DomTokenList>{
        &**self.class_list.get_or_init(||self.class_list_impl())
    }
}

#[rpc]
impl dyn HtmlElement {
    #[rpc]
    fn style_impl(self: &RcfRef<Self>, runtime: &Rc<Runtime>) -> RcCssStyleDeclaration {
        Ok(RcCssStyleDeclaration::peer_new(self.native().style()))
    }
    #[rpc]
    fn class_list_impl(self: &RcfRef<Self>, runtime: &Rc<Runtime>) -> RcDomTokenList {
        Ok(RcDomTokenList::peer_new(self.native().class_list()))
    }
}
