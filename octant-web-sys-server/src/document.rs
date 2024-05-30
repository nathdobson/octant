use crate::octant_runtime::PeerNew;
use octant_object::{class, define_class};
use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{
    define_sys_rpc,
    handle::TypedHandle,
    immediate_return::AsTypedHandle,
    octant_future::OctantFuture,
    peer::{AsNative, Peer, PeerValue},
    proto::{DownMessage, UpMessage},
    runtime::Runtime,
    DeserializePeer, SerializePeer,
};
use octant_serde::{define_serde_impl, DeserializeWith};
use safe_once::cell::OnceCell;
use serde::Serialize;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::spawn_local;

use crate::{
    credential::AsCredential,
    element::{ElementValue, RcElement},
    html_div_element::{HtmlDivElement, HtmlDivElementValue, RcHtmlDivElement},
    html_element::{HtmlElement, HtmlElementValue, RcHtmlElement},
    html_form_element::{HtmlFormElementValue, RcHtmlFormElement},
    html_input_element::{HtmlInputElementValue, RcHtmlInputElement},
    node::{Node, NodeValue, RcNode},
    object::Object,
    text::{RcText, Text, TextValue},
};

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Document {
    parent: dyn Node,
    #[cfg(side = "client")]
    any_value: web_sys::Document,
    #[cfg(side = "server")]
    body: OnceCell<RcHtmlElement>,
}

pub trait Document: AsDocument {}

impl<T> Document for T where T: AsDocument {}

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

define_sys_rpc! {
    pub fn create_div(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlDivElement {
        Ok(Rc2::new(HtmlDivElementValue::peer_new(doc.native().create_element("div").unwrap().dyn_into().unwrap())))
    }
    pub fn create_text_node(_runtime:_, doc:Rc2<dyn Document>, text: String) -> RcText {
        Ok(Rc2::new(TextValue::peer_new(doc.native().create_text_node(&text).dyn_into().unwrap())))
    }
    pub fn create_input_element(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlInputElement {
        Ok(Rc2::new(HtmlInputElementValue::peer_new(doc.native().create_element("input").unwrap().dyn_into().unwrap())))
    }
    pub fn create_element(_runtime:_, doc:Rc2<dyn Document>, tag: String) -> RcElement {
        Ok(Rc2::new(ElementValue::peer_new(doc.native().create_element(&tag).unwrap())))
    }
    pub fn create_form_element(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlFormElement {
        Ok(Rc2::new(HtmlFormElementValue::peer_new(doc.native().create_element("form").unwrap().dyn_into().unwrap())))
    }
    pub fn body(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlElement {
        Ok(Rc2::new(HtmlElementValue::peer_new(doc.native().body().unwrap()) ))
    }
    pub fn location(runtime:_, doc:Rc2<dyn Document>) -> OctantFuture<String> {
        Ok(OctantFuture::<String>::spawn(&runtime, async move{
            doc.native().location().unwrap().href().clone().unwrap()
        }))
    }
}
