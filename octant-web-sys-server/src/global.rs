#[cfg(side = "server")]
use crate::event_handler::EventHandler;
use crate::{
    credential_creation_options::RcCredentialCreationOptions,
    credential_request_options::RcCredentialRequestOptions, null::RcNull,
    octant_runtime::peer::AsNative, request::RcRequest, request_init::RcRequestInit,
    window::RcWindow,
};
#[cfg(side = "server")]
use crate::{
    js_value::{JsValue, RcJsValue},
    request_init::RequestInit,
    window::Window,
};
use marshal_pointer::{EmptyRcf, Rcf, RcfRef};
use octant_runtime::{rpc, runtime::Runtime, PeerNew};
use safe_once::cell::OnceCell;
use std::{any::Any, rc::Rc};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use web_sys::Event;

#[cfg(side = "server")]
pub struct Global {
    runtime: Rc<Runtime>,
    window: OnceCell<RcWindow>,
    null: OnceCell<RcJsValue>,
}

#[cfg(side = "server")]
impl Global {
    pub fn runtime(&self) -> &Rc<Runtime> {
        &self.runtime
    }
    pub fn new(runtime: Rc<Runtime>) -> Rc<Self> {
        Rc::new(Global {
            runtime,
            window: OnceCell::new(),
            null: OnceCell::new(),
        })
    }
}

#[cfg(side = "server")]
impl Global {
    pub fn window(&self) -> &RcfRef<dyn Window> {
        self.window.get_or_init(|| window(&self.runtime))
    }
    pub fn null(&self) -> &RcfRef<dyn JsValue> {
        self.null.get_or_init(|| new_null(&self.runtime))
    }
    pub fn new_request_init(&self) -> RcRequestInit {
        new_request_init(&self.runtime)
    }
    pub fn new_request(&self, url: String, request_init: RcRequestInit) -> RcRequest {
        new_request(self.runtime(), url, request_init)
    }
    pub fn new_credential_request_options(&self) -> RcCredentialRequestOptions {
        new_credential_request_options(self.runtime())
    }
    pub fn new_credential_creation_options(&self) -> RcCredentialCreationOptions {
        new_credential_creation_options(self.runtime())
    }
    // pub fn new_form_submit_listener(&self, handler: Box<dyn EventHandler>) -> RcFormSubmitListener {
    //     let result = new_form_submit_listener(self.runtime());
    //     result.set_handler(handler);
    //     result
    // }
    // pub fn new_anchor_click_listener(&self) -> RcAnchorClickListener {
    //     let result = new_form_submit_listener(self.runtime());
    //     result.set_handler(handler);
    //     result
    // }
}

#[rpc]
fn window(_: &Rc<Runtime>) -> RcWindow {
    Ok(RcWindow::peer_new(web_sys::window().unwrap()))
}

#[rpc]
fn new_null(_: &Rc<Runtime>) -> RcNull {
    Ok(RcNull::peer_new(wasm_bindgen::JsValue::null()))
}

#[rpc]
fn new_request_init(_: &Rc<Runtime>) -> RcRequestInit {
    Ok(RcRequestInit::peer_new(web_sys::RequestInit::new()))
}

#[rpc]
fn new_request(_: &Rc<Runtime>, url: String, init: RcRequestInit) -> RcRequest {
    Ok(RcRequest::peer_new(
        web_sys::Request::new_with_str_and_init(&url, init.native()).unwrap(),
    ))
}

#[rpc]
fn new_credential_request_options(_: &Rc<Runtime>) -> RcCredentialRequestOptions {
    Ok(RcCredentialRequestOptions::peer_new(
        web_sys::CredentialRequestOptions::new(),
    ))
}

#[rpc]
fn new_credential_creation_options(_: &Rc<Runtime>) -> RcCredentialCreationOptions {
    Ok(RcCredentialCreationOptions::peer_new(
        web_sys::CredentialCreationOptions::new(),
    ))
}
