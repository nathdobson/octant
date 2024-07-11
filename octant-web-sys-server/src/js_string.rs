use crate::{
    css_style_declaration::{CssStyleDeclaration, RcCssStyleDeclaration},
    dom_token_list::{DomTokenList, RcDomTokenList},
    element::{Element, ElementFields},
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{DeserializePeer, PeerNew, SerializePeer};
use safe_once::cell::OnceCell;
use crate::js_value::{JsValue, JsValueFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct JsStringFields {
    parent: JsValueFields,
    #[cfg(side = "client")]
    native: js_sys::JsString,
}

#[class]
pub trait JsString: JsValue {}
