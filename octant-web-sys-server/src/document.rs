use crate::octant_runtime::PeerNew;
use octant_object::{class, DebugClass};
use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{
    handle::TypedHandle,
    immediate_return::AsTypedHandle,
    octant_future::OctantFuture,
    peer::{AsNative, Peer, PeerFields},
    proto::{DownMessage, UpMessage},
    rpc,
    runtime::Runtime,
    DeserializePeer, SerializePeer,
};
use octant_serde::{define_serde_impl, DeserializeWith};
use safe_once::cell::OnceCell;
use serde::Serialize;
use std::rc::Rc;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::spawn_local;

use crate::{
    element::{ElementFields, RcElement},
    html_div_element::{HtmlDivElement, HtmlDivElementFields, RcHtmlDivElement},
    html_element::{HtmlElement, HtmlElementFields, RcHtmlElement},
    html_form_element::{HtmlFormElementFields, RcHtmlFormElement},
    html_input_element::{HtmlInputElementFields, RcHtmlInputElement},
    node::{Node, NodeFields, RcNode},
    object::Object,
    text::{RcText, Text, TextFields},
};

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
        Ok(Rc2::new(HtmlDivElementFields::peer_new(
            self.native()
                .create_element("div")
                .unwrap()
                .dyn_into()
                .unwrap(),
        )))
    }
    #[rpc]
    pub fn create_form_element(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcHtmlFormElement {
        Ok(Rc2::new(HtmlFormElementFields::peer_new(
            self.native()
                .create_element("form")
                .unwrap()
                .dyn_into()
                .unwrap(),
        )))
    }
    #[rpc]
    pub fn create_element(self: &RcRef<Self>, _: &Rc<Runtime>, tag: String) -> RcElement {
        Ok(Rc2::new(ElementFields::peer_new(
            self.native().create_element(&tag).unwrap(),
        )))
    }
    #[rpc]
    pub fn create_text_node(self: &RcRef<Self>, _: &Rc<Runtime>, text: String) -> RcText {
        Ok(Rc2::new(TextFields::peer_new(
            self.native().create_text_node(&text).dyn_into().unwrap(),
        )))
    }
    #[rpc]
    pub fn create_input_element(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcHtmlInputElement {
        Ok(Rc2::new(HtmlInputElementFields::peer_new(
            self.native()
                .create_element("input")
                .unwrap()
                .dyn_into()
                .unwrap(),
        )))
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
        Ok(Rc2::new(HtmlElementFields::peer_new(
            self.native().body().unwrap(),
        )))
    }
}
