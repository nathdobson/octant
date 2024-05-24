use crate::{
    html_div_element::HtmlDivElementValue,
    html_element::{ArcHtmlElement, HtmlElement, HtmlElementValue},
    node::NodeValue,
    text::{Text, TextValue},
};
use octant_object::define_class;
use octant_reffed::{ArcRef, Reffed};
use octant_runtime::{
    completable::Completable,
    define_sys_class, define_sys_rpc,
    handle::TypedHandle,
    peer::{Peer, PeerValue},
    proto::{DownMessage, UpMessage},
    runtime::Runtime,
};
use octant_serde::{define_serde_impl, derive_deserialize_with_for_struct};
use safe_once::sync::OnceLock;
use serde::Serialize;
use std::sync::Arc;

#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::spawn_local;

use crate::{
    element::ArcElement,
    html_div_element::{ArcHtmlDivElement, HtmlDivElement},
    html_form_element::ArcHtmlFormElement,
    html_input_element::ArcHtmlInputElement,
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
    pub fn create_div(self: &Arc<Self>) -> ArcHtmlDivElement {
        create_div(self.runtime(), self.clone())
    }
    pub fn create_text_node(self: &Arc<Self>, content: String) -> ArcText {
        create_text_node(self.runtime(), self.clone(), content)
    }
    pub fn create_input_element(self: &Arc<Self>) -> ArcHtmlInputElement {
        todo!()
    }
    pub fn create_form_element(self: &Arc<Self>) -> ArcHtmlFormElement {
        todo!()
    }
    pub fn create_element(self: &Arc<Self>, tag: &str) -> ArcElement {
        todo!()
    }
    pub fn body<'a>(self: &'a Arc<Self>) -> &'a ArcHtmlElement {
        self.body.get_or_init(|| body(self.runtime(), self.clone()))
    }
    pub async fn location<'a>(self: ArcRef<'a, Self>) -> String {
        location(self.runtime(), self.arc()).await
    }
}

define_sys_rpc! {
    fn create_div(_runtime, document: Arc<dyn Document>) -> (HtmlDivElement, ) {
        Ok((Arc::new(HtmlDivElementValue::new(document.native().create_element("div").unwrap().dyn_into().unwrap())), ))
    }
}

define_sys_rpc! {
    fn create_text_node(_runtime, document: Arc<dyn Document>, text: String) -> (Text, ) {
        Ok((Arc::new(TextValue::new(document.native().create_text_node(&text).dyn_into().unwrap())), ))
    }
}

define_sys_rpc! {
    fn body(_runtime, document: Arc<dyn Document>) -> (HtmlElement, ) {
        Ok((Arc::new(HtmlElementValue::new(document.native().body().unwrap())), ))
    }
}

define_sys_class! {
    class LocationPromise;
    extends Peer;
    new_client _;
    server_field response: Completable<String>;
}

#[cfg(side = "server")]
impl LocationPromiseValue {
    pub fn new(parent: PeerValue) -> Self {
        LocationPromiseValue {
            parent,
            response: Completable::new(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LocationRequest {
    document: ArcDocument,
    promise: TypedHandle<dyn LocationPromise>,
}

#[derive(Serialize, Debug)]
pub struct LocationResponse {
    promise: Arc<dyn LocationPromise>,
    location: String,
}

derive_deserialize_with_for_struct! {
    struct LocationRequest {
        document: ArcDocument,
        promise: TypedHandle<dyn LocationPromise>,
    }
}

derive_deserialize_with_for_struct! {
    struct LocationResponse {
        promise: Arc<dyn LocationPromise>,
        location: String,
    }
}

define_serde_impl!(LocationRequest: DownMessage);
impl DownMessage for LocationRequest {
    #[cfg(side = "client")]
    fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()> {
        log::info!("Received request");
        let promise: ArcLocationPromise = Arc::new(LocationPromiseValue::new());
        runtime.add(self.promise, promise.clone());
        spawn_local({
            let runtime = runtime.clone();
            async move {
                log::info!("Sending response");
                runtime.send(Box::<LocationResponse>::new(LocationResponse {
                    promise,
                    location: location_impl(&runtime, self.document).await,
                }));
            }
        });
        Ok(())
    }
}

define_serde_impl!(LocationResponse: UpMessage);
impl UpMessage for LocationResponse {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()> {
        self.promise.response.send(self.location);
        Ok(())
    }
}

#[cfg(side = "server")]
async fn location(runtime: &Arc<Runtime>, document: ArcDocument) -> String {
    let promise: ArcLocationPromise = runtime.add(LocationPromiseValue::new(runtime.add_uninit()));
    log::info!("Sending request");
    runtime.send(Box::<LocationRequest>::new(LocationRequest {
        document,
        promise: promise.typed_handle(),
    }));
    let response = promise.response.recv().await;
    response
}

#[cfg(side = "client")]
async fn location_impl(runtime: &Arc<Runtime>, document: Arc<dyn Document>) -> String {
    document.native().location().unwrap().href().unwrap()
}
