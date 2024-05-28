use std::{any::Any, hint::must_use, marker::PhantomData, sync::Arc};

use crate::request_init::RequestInitValue;
use catalog::register;
use octant_reffed::arc::{Arc2, ArcRef};
use octant_runtime::{define_sys_rpc, runtime::Runtime};
use octant_serde::define_serde_impl;
use safe_once::sync::OnceLock;
use serde::{Deserialize, Serialize};
use wasm_error::WasmError;

use crate::{
    credential_creation_options::{ArcCredentialCreationOptions, CredentialCreationOptionsValue},
    credential_request_options::{ArcCredentialRequestOptions, CredentialRequestOptionsValue},
    event_listener::{ArcEventListener, EventListenerValue},
    request::{ArcRequest, RequestValue},
    request_init::{ArcRequestInit, RequestInit},
    window::{ArcWindow, Window, WindowValue},
};

#[cfg(side = "server")]
pub struct Global {
    runtime: Arc<Runtime>,
    window: OnceLock<ArcWindow>,
}

#[cfg(side = "server")]
impl Global {
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
    pub fn new(runtime: Arc<Runtime>) -> Arc<Self> {
        Arc::new(Global {
            runtime,
            window: OnceLock::new(),
        })
    }
}

#[cfg(side = "server")]
impl Global {
    pub fn window(&self) -> &ArcRef<dyn Window> {
        self.window.get_or_init(|| window(&self.runtime))
    }
    pub fn new_request_init(&self) -> ArcRequestInit {
        new_request_init(&self.runtime)
    }
    pub fn new_request(&self, url: String, request_init: ArcRequestInit) -> ArcRequest {
        new_request(self.runtime(), url, request_init)
    }
    pub fn new_credential_request_options(&self) -> ArcCredentialRequestOptions {
        new_credential_request_options(self.runtime())
    }
    pub fn new_credential_creation_options(&self) -> ArcCredentialCreationOptions {
        new_credential_creation_options(self.runtime())
    }
    pub fn new_event_listener(
        &self,
        handler: impl 'static + Sync + Send + Any + Fn(),
    ) -> ArcEventListener {
        let listener = new_event_listener(self.runtime());
        listener.set_handler(handler);
        listener
    }
}

define_sys_rpc! {
    fn window(_runtime:_) -> ArcWindow {
        Ok(Arc2::new(WindowValue::new(web_sys::window().unwrap())))
    }
}

define_sys_rpc! {
    fn new_request_init(_runtime:_) -> ArcRequestInit {
        Ok(Arc2::new(RequestInitValue::new(web_sys::RequestInit::new())))
    }
}

define_sys_rpc! {
    fn new_request(_runtime:_, url:String, init:ArcRequestInit) -> ArcRequest {
        Ok(Arc2::new(RequestValue::new(web_sys::Request::new_with_str_and_init(&url, init.native()).unwrap())))
    }
}

define_sys_rpc! {
    fn new_credential_request_options(_runtime:_) -> ArcCredentialRequestOptions {
        Ok(Arc2::new(CredentialRequestOptionsValue::new(web_sys::CredentialRequestOptions::new())))
    }
}

define_sys_rpc! {
    fn new_credential_creation_options(_runtime:_) -> ArcCredentialCreationOptions {
        Ok(Arc2::new(CredentialCreationOptionsValue::new(web_sys::CredentialCreationOptions::new())))
    }
}

define_sys_rpc! {
    fn new_event_listener(_runtime:_) -> ArcEventListener {
        Ok(Arc2::new(EventListenerValue::new()))
    }
}
