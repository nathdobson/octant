use std::{any::Any, hint::must_use, marker::PhantomData};
use std::rc::Rc;

use catalog::register;
use safe_once::cell::OnceCell;
use serde::{Deserialize, Serialize};

use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{define_sys_rpc, PeerNew, runtime::Runtime};
use octant_runtime::peer::AsNative;
use octant_serde::define_serde_impl;

use crate::{
    credential_creation_options::{CredentialCreationOptionsFields, RcCredentialCreationOptions},
    credential_request_options::{CredentialRequestOptionsFields, RcCredentialRequestOptions},
    event_listener::{EventListenerFields, RcEventListener},
    request::{RcRequest, RequestFields},
    request_init::{RcRequestInit, RequestInit},
    window::{RcWindow, Window, WindowFields},
};
use crate::request_init::RequestInitFields;

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

define_sys_rpc! {
    fn window(_runtime:_) -> RcWindow {
        Ok(Rc2::new(WindowFields::peer_new(web_sys::window().unwrap())))
    }
    fn new_request_init(_runtime:_) -> RcRequestInit {
        Ok(Rc2::new(RequestInitFields::peer_new(web_sys::RequestInit::new())))
    }
    fn new_request(_runtime:_, url:String, init:RcRequestInit) -> RcRequest {
        Ok(Rc2::new(RequestFields::peer_new(web_sys::Request::new_with_str_and_init(&url, init.native()).unwrap())))
    }
    fn new_credential_request_options(_runtime:_) -> RcCredentialRequestOptions {
        Ok(Rc2::new(CredentialRequestOptionsFields::peer_new(web_sys::CredentialRequestOptions::new())))
    }
    fn new_credential_creation_options(_runtime:_) -> RcCredentialCreationOptions {
        Ok(Rc2::new(CredentialCreationOptionsFields::peer_new(web_sys::CredentialCreationOptions::new())))
    }
    fn new_event_listener(_runtime:_) -> RcEventListener {
        Ok(Rc2::new(EventListenerFields::peer_new(())))
    }
}
