use crate::{
    html_div_element::HtmlDivElementValue,
    html_element::{ArcHtmlElement, HtmlElement, HtmlElementValue},
    node::NodeValue,
    text::{Text, TextValue},
};
use octant_object::define_class;
use octant_reffed::arc::{Arc2, ArcRef};
use octant_runtime::octant_future::OctantFuture;
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
use octant_serde::{define_serde_impl, DeserializeWith};
use safe_once::sync::OnceLock;
use serde::Serialize;
use std::{future::Future, sync::Arc};

#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::spawn_local;

use crate::{
    element::{ArcElement, ElementValue},
    html_div_element::{ArcHtmlDivElement, HtmlDivElement},
    html_form_element::{ArcHtmlFormElement, HtmlFormElementValue},
    html_input_element::{ArcHtmlInputElement, HtmlInputElementValue},
    node::{ArcNode, Node},
    text::ArcText,
};

define_sys_class! {
    class Document;
    extends Node;
    wasm web_sys::Document;
    new_client _;
    new_server _;
    server_field body: OnceLock<ArcHtmlElement>;
    server_fn {
        fn create_div(self: &ArcRef<Self>) -> ArcHtmlDivElement{
            create_div(self.runtime(), self.arc())
        }
        fn create_form_element(self: &ArcRef<Self>) -> ArcHtmlFormElement {
            create_form_element(self.runtime(), self.arc())
        }
        fn create_element(self: &ArcRef<Self>, tag: &str) -> ArcElement {
            create_element(self.runtime(), self.arc(), tag.to_string())
        }
        fn create_text_node(self: &ArcRef<Self>, text: String) -> ArcText{
            create_text_node(self.runtime(),self.arc(),text)
        }
        fn create_input_element(self: &ArcRef<Self>) -> ArcHtmlInputElement{
            create_input_element(self.runtime(), self.arc())
        }
        fn location(self: &ArcRef<Self>) -> OctantFuture<String>{
            location(self.runtime(), self.arc())
        }
        fn body<'a> (self: &'a ArcRef<Self>) -> &'a ArcRef<dyn HtmlElement>{
            self.document().body.get_or_init(||{
                body(self.runtime(),self.arc())
            })
        }
    }
}

define_sys_rpc! {
    pub fn create_div(_runtime:_, doc:Arc2<dyn Document>) -> ArcHtmlDivElement {
        Ok(Arc2::new(HtmlDivElementValue::new(doc.native().create_element("div").unwrap().dyn_into().unwrap())))
    }
    pub fn create_text_node(_runtime:_, doc:Arc2<dyn Document>, text: String) -> ArcText {
        Ok(Arc2::new(TextValue::new(doc.native().create_text_node(&text).dyn_into().unwrap())))
    }
    pub fn create_input_element(_runtime:_, doc:Arc2<dyn Document>) -> ArcHtmlInputElement {
        Ok(Arc2::new(HtmlInputElementValue::new(doc.native().create_element("input").unwrap().dyn_into().unwrap())))
    }
    pub fn create_element(_runtime:_, doc:Arc2<dyn Document>, tag: String) -> ArcElement {
        Ok(Arc2::new(ElementValue::new(doc.native().create_element(&tag).unwrap())))
    }
    pub fn create_form_element(_runtime:_, doc:Arc2<dyn Document>) -> ArcHtmlFormElement {
        Ok(Arc2::new(HtmlFormElementValue::new(doc.native().create_element("form").unwrap().dyn_into().unwrap())))
    }
    pub fn body(_runtime:_, doc:Arc2<dyn Document>) -> ArcHtmlElement {
        Ok(Arc2::new(HtmlElementValue::new(doc.native().body().unwrap()) ))
    }
    pub fn location(runtime:_, doc:Arc2<dyn Document>) -> OctantFuture<String> {
        Ok(OctantFuture::<String>::spawn(&runtime, async move{
            doc.native().location().unwrap().href().clone().unwrap()
        }))
    }
}
