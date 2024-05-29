use safe_once::sync::OnceLock;
use serde::Serialize;

use octant_object::define_class;
use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{
    // octant_future::Completable,
    define_sys_class,
    define_sys_rpc,
    handle::TypedHandle,
    immediate_return::AsTypedHandle,
    peer::{Peer, PeerValue},
    proto::{DownMessage, UpMessage},
    runtime::Runtime,
};
use octant_runtime::octant_future::OctantFuture;
use octant_serde::{define_serde_impl, DeserializeWith};
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::spawn_local;

use crate::{
    html_div_element::HtmlDivElementValue,
    html_element::{HtmlElement, HtmlElementValue, RcHtmlElement},
    node::NodeValue,
    text::{Text, TextValue},
};
use crate::{
    element::{ElementValue, RcElement},
    html_div_element::{HtmlDivElement, RcHtmlDivElement},
    html_form_element::{HtmlFormElementValue, RcHtmlFormElement},
    html_input_element::{HtmlInputElementValue, RcHtmlInputElement},
    node::{Node, RcNode},
    text::RcText,
};

define_sys_class! {
    class Document;
    extends Node;
    wasm web_sys::Document;
    new_client _;
    new_server _;
    server_field body: OnceLock<RcHtmlElement>;
    server_fn {
        fn create_div(self: &RcRef<Self>) -> RcHtmlDivElement{
            create_div(self.runtime(), self.rc())
        }
        fn create_form_element(self: &RcRef<Self>) -> RcHtmlFormElement {
            create_form_element(self.runtime(), self.rc())
        }
        fn create_element(self: &RcRef<Self>, tag: &str) -> RcElement {
            create_element(self.runtime(), self.rc(), tag.to_string())
        }
        fn create_text_node(self: &RcRef<Self>, text: String) -> RcText{
            create_text_node(self.runtime(),self.rc(),text)
        }
        fn create_input_element(self: &RcRef<Self>) -> RcHtmlInputElement{
            create_input_element(self.runtime(), self.rc())
        }
        fn location(self: &RcRef<Self>) -> OctantFuture<String>{
            location(self.runtime(), self.rc())
        }
        fn body<'a> (self: &'a RcRef<Self>) -> &'a RcRef<dyn HtmlElement>{
            self.document().body.get_or_init(||{
                body(self.runtime(),self.rc())
            })
        }
    }
}

define_sys_rpc! {
    pub fn create_div(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlDivElement {
        Ok(Rc2::new(HtmlDivElementValue::new(doc.native().create_element("div").unwrap().dyn_into().unwrap())))
    }
    pub fn create_text_node(_runtime:_, doc:Rc2<dyn Document>, text: String) -> RcText {
        Ok(Rc2::new(TextValue::new(doc.native().create_text_node(&text).dyn_into().unwrap())))
    }
    pub fn create_input_element(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlInputElement {
        Ok(Rc2::new(HtmlInputElementValue::new(doc.native().create_element("input").unwrap().dyn_into().unwrap())))
    }
    pub fn create_element(_runtime:_, doc:Rc2<dyn Document>, tag: String) -> RcElement {
        Ok(Rc2::new(ElementValue::new(doc.native().create_element(&tag).unwrap())))
    }
    pub fn create_form_element(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlFormElement {
        Ok(Rc2::new(HtmlFormElementValue::new(doc.native().create_element("form").unwrap().dyn_into().unwrap())))
    }
    pub fn body(_runtime:_, doc:Rc2<dyn Document>) -> RcHtmlElement {
        Ok(Rc2::new(HtmlElementValue::new(doc.native().body().unwrap()) ))
    }
    pub fn location(runtime:_, doc:Rc2<dyn Document>) -> OctantFuture<String> {
        Ok(OctantFuture::<String>::spawn(&runtime, async move{
            doc.native().location().unwrap().href().clone().unwrap()
        }))
    }
}
