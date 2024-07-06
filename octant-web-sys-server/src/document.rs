use crate::{
    element::RcElement,
    html_div_element::{HtmlDivElement, RcHtmlDivElement},
    html_element::{HtmlElement, RcHtmlElement},
    html_form_element::RcHtmlFormElement,
    html_input_element::RcHtmlInputElement,
    node::{Node, NodeFields},
    object::Object,
    octant_runtime::{peer::AsNative, PeerNew},
    text::{RcText, Text},
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{
    octant_future::OctantFuture, peer::Peer, rpc, runtime::Runtime, DeserializePeer, SerializePeer,
};
use safe_once::cell::OnceCell;
use std::rc::Rc;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
use crate::html_anchor_element::RcHtmlAnchorElement;
use crate::location::{Location, RcLocation};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct DocumentFields {
    parent: NodeFields,
    #[cfg(side = "client")]
    document: web_sys::Document,
    #[cfg(side = "server")]
    body: OnceCell<RcHtmlElement>,
    location: OnceCell<RcLocation>,
}

#[class]
pub trait Document: Node {}

#[rpc]
impl dyn Document {
    #[rpc]
    pub fn create_div_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlDivElement {
        Ok(RcHtmlDivElement::peer_new(
            self.native()
                .create_element("div")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    #[rpc]
    pub fn create_anchor_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlAnchorElement {
        Ok(RcHtmlAnchorElement::peer_new(
            self.native()
                .create_element("a")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    #[rpc]
    pub fn create_form_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlFormElement {
        Ok(RcHtmlFormElement::peer_new(
            self.native()
                .create_element("form")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    #[rpc]
    pub fn create_element(self: &RcfRef<Self>, _: &Rc<Runtime>, tag: String) -> RcElement {
        Ok(RcElement::peer_new(
            self.native().create_element(&tag).unwrap(),
        ))
    }
    #[rpc]
    pub fn create_text_node(self: &RcfRef<Self>, _: &Rc<Runtime>, text: String) -> RcText {
        Ok(RcText::peer_new(
            self.native().create_text_node(&text).dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_input_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlInputElement {
        Ok(RcHtmlInputElement::peer_new(
            self.native()
                .create_element("input")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    #[cfg(side = "server")]
    pub fn location<'a>(self: &'a RcfRef<Self>)->&'a RcfRef<dyn Location>{
        self.document().location.get_or_init(|| self.location_impl())
    }
    #[rpc]
    fn location_impl(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcLocation {
        Ok(RcLocation::peer_new(self.native().location().unwrap()))
    }
    #[cfg(side = "server")]
    pub fn body<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn HtmlElement> {
        self.document().body.get_or_init(|| self.body_impl())
    }
    #[rpc]
    fn body_impl(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlElement {
        Ok(RcHtmlElement::peer_new(self.native().body().unwrap()))
    }
}
