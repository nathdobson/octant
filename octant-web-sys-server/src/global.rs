use std::{hint::must_use, marker::PhantomData, sync::Arc};

use catalog::register;
use safe_once::sync::OnceLock;
use serde::{Deserialize, Serialize};
use octant_reffed::{ArcRef, Reffed};
use octant_runtime::define_sys_rpc;
use octant_runtime::runtime::Runtime;
use octant_serde::define_serde_impl;
use wasm_error::WasmError;
use crate::request_init::RequestInitValue;

use crate::{
    credential_creation_options::ArcCredentialCreationOptions,
    credential_request_options::ArcCredentialRequestOptions,
    request::ArcRequest,
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
    pub fn window(&self) -> ArcRef<dyn Window> {
        self.window.get_or_init(|| window(&self.runtime)).reffed()
    }
    pub fn new_request_init(&self) -> ArcRequestInit {
        new_request_init(&self.runtime)
    }
    pub fn new_request(&self, url: String, request_init: &ArcRequestInit) -> ArcRequest {
        todo!();
    }
    pub fn new_credential_request_options(&self) -> ArcCredentialRequestOptions {
        todo!();
    }
    pub fn new_credential_creation_options(&self) -> ArcCredentialCreationOptions {
        todo!();
    }
}

define_sys_rpc! {
    fn window(_ctx) -> (Window, ) {
        Ok((Arc::new(WindowValue::new(web_sys::window().unwrap())),))
    }
}

define_sys_rpc! {
    fn new_request_init(_ctx) -> (RequestInit, ) {
        Ok((Arc::new(RequestInitValue::new(web_sys::RequestInit::new())),))
    }
}
