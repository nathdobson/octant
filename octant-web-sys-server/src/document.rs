use crate::{
    element::RcElement,
    html_anchor_element::RcHtmlAnchorElement,
    html_br_element::RcHtmlBrElement,
    html_div_element::{HtmlDivElement, RcHtmlDivElement},
    html_element::{HtmlElement, RcHtmlElement},
    html_form_element::RcHtmlFormElement,
    html_head_element::{HtmlHeadElement, RcHtmlHeadElement},
    html_heading_element::RcHtmlHeadingElement,
    html_hr_element::RcHtmlHrElement,
    html_input_element::RcHtmlInputElement,
    html_li_element::RcHtmlLiElement,
    html_paragraph_element::RcHtmlParagraphElement,
    html_style_element::RcHtmlStyleElement,
    html_u_list_element::RcHtmlUListElement,
    location::{Location, RcLocation},
    node::{Node, NodeFields},
    object::Object,
    octant_runtime::{peer::AsNative, PeerNew},
    text::{RcText, Text},
};
use marshal_pointer::RcfRef;
use octant_error::octant_error;
use octant_object::{class, DebugClass};
use octant_runtime::{
    octant_future::OctantFuture, peer::Peer, rpc, runtime::Runtime, DeserializePeer, SerializePeer,
};
use safe_once::cell::OnceCell;
use std::rc::Rc;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
use crate::html_label_element::RcHtmlLabelElement;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct DocumentFields {
    parent: NodeFields,
    #[cfg(side = "client")]
    document: web_sys::Document,
    #[cfg(side = "server")]
    body: OnceCell<RcHtmlElement>,
    #[cfg(side = "server")]
    head: OnceCell<RcHtmlHeadElement>,
    #[cfg(side = "server")]
    location: OnceCell<RcLocation>,
}

#[class]
pub trait Document: Node {}

#[rpc]
impl dyn Document {
    #[rpc]
    pub fn create_div_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlDivElement {
        Ok(RcHtmlDivElement::peer_new(
            self.native().create_element("div")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_anchor_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlAnchorElement {
        Ok(RcHtmlAnchorElement::peer_new(
            self.native().create_element("a")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_form_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlFormElement {
        Ok(RcHtmlFormElement::peer_new(
            self.native().create_element("form")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_hr_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlHrElement {
        Ok(RcHtmlHrElement::peer_new(
            self.native().create_element("hr")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_br_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlBrElement {
        Ok(RcHtmlBrElement::peer_new(
            self.native().create_element("br")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_text_node(self: &RcfRef<Self>, _: &Rc<Runtime>, text: String) -> RcText {
        Ok(RcText::peer_new(
            self.native().create_text_node(&text).dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_heading_element(
        self: &RcfRef<Self>,
        _: &Rc<Runtime>,
        n: usize,
    ) -> RcHtmlHeadingElement {
        Ok(RcHtmlHeadingElement::peer_new(
            self.native()
                .create_element(&format!("h{}", n))?
                .dyn_into()
                .unwrap(),
        ))
    }
    #[rpc]
    pub fn create_input_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlInputElement {
        Ok(RcHtmlInputElement::peer_new(
            self.native().create_element("input")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_label_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlLabelElement {
        Ok(RcHtmlLabelElement::peer_new(
            self.native().create_element("label")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_paragraph_element(
        self: &RcfRef<Self>,
        _: &Rc<Runtime>,
    ) -> RcHtmlParagraphElement {
        Ok(RcHtmlParagraphElement::peer_new(
            self.native().create_element("p")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_u_list_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlUListElement {
        Ok(RcHtmlUListElement::peer_new(
            self.native().create_element("ul")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_li_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlLiElement {
        Ok(RcHtmlLiElement::peer_new(
            self.native().create_element("li")?.dyn_into().unwrap(),
        ))
    }
    #[rpc]
    pub fn create_style_element(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlStyleElement {
        Ok(RcHtmlStyleElement::peer_new(
            self.native().create_element("style")?.dyn_into().unwrap(),
        ))
    }
    #[cfg(side = "server")]
    pub fn location<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn Location> {
        self.document()
            .location
            .get_or_init(|| self.location_impl())
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
    #[cfg(side = "server")]
    pub fn head<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn HtmlHeadElement> {
        self.document().head.get_or_init(|| self.head_impl())
    }
    #[rpc]
    fn head_impl(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcHtmlHeadElement {
        Ok(RcHtmlHeadElement::peer_new(
            self.native()
                .head()
                .ok_or_else(|| octant_error!("document head missing"))?,
        ))
    }
}
