#[cfg(side = "server")]
use crate::event_handler::EventHandler;
#[cfg(side = "client")]
use crate::event_target::ClientEventHandler;
use crate::{
    html_element::{HtmlElement, HtmlElementFields},
    html_input_element::RcHtmlInputElement,
    node::Node,
    object::Object,
    octant_runtime::peer::AsNative,
};
use marshal::{Deserialize, Serialize};
use marshal_object::derive_variant;
use marshal_pointer::{Rcf, RcfRef};
use octant_error::OctantResult;
use octant_object::{cast::downcast_object, class, DebugClass};
use octant_runtime::{
    proto::{BoxUpMessage, UpMessage},
    rpc,
    runtime::Runtime,
    DeserializePeer, PeerNew, SerializePeer,
};
use safe_once::cell::OnceCell;
use std::{fmt::Debug, rc::Rc};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use web_sys::Event;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct HtmlFormElementFields {
    parent: HtmlElementFields,
    #[cfg(side = "client")]
    any_value: web_sys::HtmlFormElement,
    #[cfg(side = "server")]
    handler: OnceCell<Box<dyn EventHandler<()>>>,
}

#[class]
pub trait HtmlFormElement: HtmlElement {
    #[cfg(side = "server")]
    fn set_form_submit_handler(self: &RcfRef<Self>, handler: Box<dyn EventHandler<()>>) {
        self.handler.set(handler).ok().unwrap();
        self.set_form_submit_handler_impl();
    }
}

#[rpc]
impl dyn HtmlFormElement {
    #[rpc]
    fn set_form_submit_handler_impl(self: &RcfRef<Self>, runtime: &Rc<Runtime>) {
        let cb = ClientEventHandler::new({
            let this = Rcf::downgrade(&self.strong());
            move |e: Event| {
                e.prevent_default();
                if let Some(this) = this.upgrade() {
                    this.update_input_values_rec();
                    this.sink()
                        .send(Box::new(SubmitForm { form: this.clone() }));
                }
                Ok(())
            }
        });
        self.add_listener("submit", cb)?;
        Ok(())
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct SubmitForm {
    form: RcHtmlFormElement,
}

derive_variant!(BoxUpMessage, SubmitForm);

impl UpMessage for SubmitForm {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        if let Some(handler) = self.form.handler.try_get() {
            (handler)(())?;
        }
        Ok(())
    }
}
