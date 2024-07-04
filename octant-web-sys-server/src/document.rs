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
use marshal_pointer::rc_ref::RcRef;
use octant_object::{class, DebugClass};
use octant_runtime::{
    octant_future::OctantFuture, peer::Peer, rpc, runtime::Runtime, DeserializePeer, SerializePeer,
};
use safe_once::cell::OnceCell;
use std::rc::Rc;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct DocumentFields {
    parent: NodeFields,
    #[cfg(side = "client")]
    any_value: web_sys::Document,
    #[cfg(side = "server")]
    body: OnceCell<RcHtmlElement>,
}

#[class]
pub trait Document: Node {}

#[rpc]
impl dyn Document {
    #[rpc]
    pub fn create_div_element(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcHtmlDivElement {
        Ok(RcHtmlDivElement::peer_new(
            self.native()
                .create_element("div")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    #[rpc]
    pub fn create_form_element(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcHtmlFormElement {
        Ok(RcHtmlFormElement::peer_new(
            self.native()
                .create_element("form")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    #[rpc]
    pub fn create_element(self: &RcRef<Self>, _: &Rc<Runtime>, tag: String) -> RcElement {
        Ok(RcElement::peer_new(
            self.native().create_element(&tag).unwrap(),
        ))
    }
    #[rpc]
    pub fn create_text_node(self: &RcRef<Self>, _: &Rc<Runtime>, text: String) -> RcText {
        Ok(RcText::peer_new(
            self.native().create_text_node(&text).dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_input_element(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcHtmlInputElement {
        Ok(RcHtmlInputElement::peer_new(
            self.native()
                .create_element("input")
                .unwrap()
                .dyn_into()
                .unwrap(),
        ))
    }
    #[rpc]
    pub fn location(self: &RcRef<Self>, runtime: &Rc<Runtime>) -> OctantFuture<String> {
        let this = self.rc();
        Ok(OctantFuture::<String>::spawn(runtime, async move {
            this.native().location().unwrap().href().clone().unwrap()
        }))
    }
    #[cfg(side = "server")]
    pub fn body<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn HtmlElement> {
        self.document().body.get_or_init(|| self.body_impl())
    }
    #[rpc]
    fn body_impl(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcHtmlElement {
        Ok(RcHtmlElement::peer_new(self.native().body().unwrap()))
    }
}
