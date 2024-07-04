use marshal_pointer::rc_ref::RcRef;
use octant_runtime::{rpc, runtime::Runtime, PeerNew};
use safe_once::cell::OnceCell;
use std::{any::Any, rc::Rc};

use crate::{
    credential_creation_options::RcCredentialCreationOptions,
    credential_request_options::RcCredentialRequestOptions,
    event_listener::RcEventListener,
    octant_runtime::peer::AsNative,
    request::RcRequest,
    request_init::{RcRequestInit, RequestInit},
    window::{RcWindow, Window},
};

#[cfg(side = "server")]
pub struct Global {
    runtime: Rc<Runtime>,
    window: OnceCell<RcWindow>,
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
        })
    }
}

#[cfg(side = "server")]
impl Global {
    pub fn window(&self) -> &RcRef<dyn Window> {
        self.window.get_or_init(|| window(&self.runtime))
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
    pub fn new_event_listener(&self, handler: impl 'static + Any + Fn()) -> RcEventListener {
        let listener = new_event_listener(self.runtime());
        listener.set_handler(handler);
        listener
    }
}

#[rpc]
fn window(_: &Rc<Runtime>) -> RcWindow {
    Ok(RcWindow::peer_new(web_sys::window().unwrap()))
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

#[rpc]
fn new_event_listener(_: &Rc<Runtime>) -> RcEventListener {
    Ok(RcEventListener::peer_new(()))
}
