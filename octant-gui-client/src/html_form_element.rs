use std::{marker::PhantomData, sync::Arc};

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::InputEvent;

use octant_gui_core::{
    HandleId, HtmlFormElementMethod, HtmlFormElementTag, HtmlFormElementUpMessage,
    HtmlInputElementUpMessage, TypedHandle, UpMessage, UpMessageList,
};
use octant_object::{cast::downcast_object, define_class};

use crate::{
    html_element::{HtmlElement, HtmlElementValue},
    html_input_element::HtmlInputElement,
    peer::ArcPeer,
    HasLocalType, Runtime,
};

define_class! {
    pub class HtmlFormElement extends HtmlElement {
        html_form_element: web_sys::HtmlFormElement,
    }
}

impl HtmlFormElementValue {
    pub fn new(handle: HandleId, html_form_element: web_sys::HtmlFormElement) -> Self {
        HtmlFormElementValue {
            parent: HtmlElementValue::new(handle, html_form_element.clone().into()),
            html_form_element,
        }
    }
    pub fn handle(&self) -> TypedHandle<HtmlFormElementTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
    pub fn native(&self) -> &web_sys::HtmlFormElement {
        &self.html_form_element
    }
}

impl dyn HtmlFormElement {
    pub fn invoke_with(
        self: Arc<Self>,
        runtime: Arc<Runtime>,
        method: &HtmlFormElementMethod,
        _handle_id: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            HtmlFormElementMethod::SetListener => {
                let this = self.clone();
                let closure: Box<dyn FnMut(_)> = Box::new({
                    let form = self.clone();
                    move |e: InputEvent| {
                        e.prevent_default();
                        let mut commands: Vec<UpMessage> = vec![];
                        let children = form.children();
                        for child in children {
                            let input: Option<Arc<dyn HtmlInputElement>> = downcast_object(child);
                            if let Some(input) = input {
                                input.native().set_attribute("disabled", "true").unwrap();
                                commands.push(UpMessage::HtmlInputElement(
                                    input.handle(),
                                    HtmlInputElementUpMessage::SetInput {
                                        value: input.native().value(),
                                    },
                                ))
                            }
                        }
                        commands.push(UpMessage::HtmlFormElement(
                            this.handle(),
                            HtmlFormElementUpMessage::Submit,
                        ));
                        if let Err(err) = runtime.send(UpMessageList { commands }) {
                            log::error!("Cannot send event {:?}", err);
                        }
                    }
                });
                let closure = Closure::wrap(closure);
                self.native()
                    .add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
                None
            }
            HtmlFormElementMethod::Enable => {
                let form = self.clone();
                let children = form.native().children();
                for input in 0..children.length() {
                    let input = children.item(input).unwrap();
                    input.remove_attribute("disabled").unwrap();
                }
                None
            }
        }
    }
}

impl HasLocalType for HtmlFormElementTag {
    type Local = dyn HtmlFormElement;
}
