use crate::{
    html_div_element::HtmlDivElementValue,
    html_element::{ArcHtmlElement, HtmlElement, HtmlElementValue},
    node::NodeValue,
    text::{Text, TextValue},
};
use octant_object::define_class;
use octant_reffed::{ArcRef, Reffed};
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
    element::ArcElement,
    html_div_element::{ArcHtmlDivElement, HtmlDivElement},
    html_form_element::ArcHtmlFormElement,
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
}

#[cfg(side = "server")]
impl dyn Document {
    pub fn create_form_element(self: ArcRef<Self>) -> ArcHtmlFormElement {
        todo!()
    }
    pub fn create_element(self: ArcRef<Self>, tag: &str) -> ArcElement {
        todo!()
    }
    pub fn body<'a>(self: ArcRef<'a, Self>) -> &'a ArcHtmlElement {
        self.body.get_or_init(|| body(self.runtime(), self.arc()))
    }
    pub fn location<'a>(self: ArcRef<'a, Self>) -> OctantFuture<String> {
        location(self.runtime(), self.arc())
    }
}

define_sys_rpc! {
    impl Document {
        pub fn create_input_element(self:_, _runtime:_) -> ArcHtmlInputElement {
            Ok(Arc::new(HtmlInputElementValue::new(self.native().create_element("input").unwrap().dyn_into().unwrap())))
        }
    }
}

define_sys_rpc! {
    impl Document {
        pub fn create_div(self:_, _runtime:_) -> ArcHtmlDivElement {
            Ok(Arc::new(HtmlDivElementValue::new(self.native().create_element("div").unwrap().dyn_into().unwrap())))
        }
    }
}

define_sys_rpc! {
    impl Document {
        pub fn create_text_node(self:_, _runtime:_, text: String) -> ArcText {
            Ok(Arc::new(TextValue::new(self.native().create_text_node(&text).dyn_into().unwrap())))
        }
    }
}

define_sys_rpc! {
    fn body(_runtime:_, document: Arc<dyn Document>) -> ArcHtmlElement {
        Ok(Arc::new(HtmlElementValue::new(document.native().body().unwrap()) ))
    }
}

define_sys_rpc! {
    fn location(runtime:_, document: Arc<dyn Document>) -> OctantFuture<String> {
        Ok(OctantFuture::<String>::spawn(runtime, async move{
            document.native().location().unwrap().href().clone().unwrap()
        }))
    }
}
