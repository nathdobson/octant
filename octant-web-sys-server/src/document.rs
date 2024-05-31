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

#[cfg(side = "server")]
impl dyn Document {
    pub fn create_div(self: &RcRef<Self>) -> RcHtmlDivElement {
        create_div(self.runtime(), self.rc())
    }
    pub fn create_form_element(self: &RcRef<Self>) -> RcHtmlFormElement {
        create_form_element(self.runtime(), self.rc())
    }
    pub fn create_element(self: &RcRef<Self>, tag: &str) -> RcElement {
        create_element(self.runtime(), self.rc(), tag.to_string())
    }
    pub fn create_text_node(self: &RcRef<Self>, text: String) -> RcText {
        create_text_node(self.runtime(), self.rc(), text)
    }
    pub fn create_input_element(self: &RcRef<Self>) -> RcHtmlInputElement {
        create_input_element(self.runtime(), self.rc())
    }
    pub fn location(self: &RcRef<Self>) -> OctantFuture<String> {
        location(self.runtime(), self.rc())
    }
    pub fn body<'a>(self: &'a RcRef<Self>) -> &'a RcRef<dyn HtmlElement> {
        self.document()
            .body
            .get_or_init(|| body(self.runtime(), self.rc()))
    }
}

#[rpc]
pub fn create_div(_: &Rc<Runtime>, doc: Rc2<dyn Document>) -> RcHtmlDivElement {
    Ok(Rc2::new(HtmlDivElementFields::peer_new(
        doc.native()
            .create_element("div")
            .unwrap()
            .dyn_into()
            .unwrap(),
    )))
}

#[rpc]
pub fn create_text_node(_: &Rc<Runtime>, doc: Rc2<dyn Document>, text: String) -> RcText {
    Ok(Rc2::new(TextFields::peer_new(
        doc.native().create_text_node(&text).dyn_into().unwrap(),
    )))
}

#[rpc]
pub fn create_input_element(_: &Rc<Runtime>, doc: Rc2<dyn Document>) -> RcHtmlInputElement {
    Ok(Rc2::new(HtmlInputElementFields::peer_new(
        doc.native()
            .create_element("input")
            .unwrap()
            .dyn_into()
            .unwrap(),
    )))
}

#[rpc]
pub fn create_element(_: &Rc<Runtime>, doc: Rc2<dyn Document>, tag: String) -> RcElement {
    Ok(Rc2::new(ElementFields::peer_new(
        doc.native().create_element(&tag).unwrap(),
    )))
}

#[rpc]
pub fn create_form_element(_: &Rc<Runtime>, doc: Rc2<dyn Document>) -> RcHtmlFormElement {
    Ok(Rc2::new(HtmlFormElementFields::peer_new(
        doc.native()
            .create_element("form")
            .unwrap()
            .dyn_into()
            .unwrap(),
    )))
}

#[rpc]
pub fn body(_: &Rc<Runtime>, doc: Rc2<dyn Document>) -> RcHtmlElement {
    Ok(Rc2::new(HtmlElementFields::peer_new(
        doc.native().body().unwrap(),
    )))
}

#[rpc]
pub fn location(runtime: &Rc<Runtime>, doc: Rc2<dyn Document>) -> OctantFuture<String> {
    Ok(OctantFuture::<String>::spawn(&runtime, async move {
        doc.native().location().unwrap().href().clone().unwrap()
    }))
}
