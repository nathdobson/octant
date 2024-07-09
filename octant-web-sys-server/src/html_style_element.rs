use crate::{
    css_style_sheet::{CssStyleSheet, RcCssStyleSheet},
    html_element::{HtmlElement, HtmlElementFields},
};
use marshal_pointer::RcfRef;
use octant_error::octant_error;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use safe_once::cell::OnceCell;
use std::{cell::RefCell, rc::Rc};
#[cfg(side = "client")]
use wasm_bindgen::JsCast;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlStyleElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    native: web_sys::HtmlStyleElement,
    sheet: OnceCell<RcCssStyleSheet>,
}

#[class]
pub trait HtmlStyleElement: HtmlElement {
    #[cfg(side = "server")]
    fn sheet<'a>(self: &'a RcfRef<Self>) -> &'a RcfRef<dyn CssStyleSheet> {
        self.sheet.get_or_init(|| self.sheet_impl())
    }
}

#[rpc]
impl dyn HtmlStyleElement {
    #[rpc]
    fn sheet_impl(self: &RcfRef<Self>, _: &Rc<Runtime>) -> RcCssStyleSheet {
        Ok(RcCssStyleSheet::peer_new(
            self.native
                .sheet()
                .ok_or_else(|| octant_error!("could not find sheet"))?
                .dyn_into()
                .map_err(|e| octant_error!("expected a CssStyleSheet"))?,
        ))
    }
}
