use std::{hint::must_use, marker::PhantomData, sync::Arc};

use catalog::register;
use safe_once::sync::OnceLock;
use serde::{Deserialize, Serialize};

#[cfg(side = "server")]
use octant_gui::{
    runtime::{HasTypedHandle, Runtime},
    UP_MESSAGE_HANDLER_REGISTRY, UpMessageHandler,
};
#[cfg(side = "client")]
use octant_gui_client::{DOWN_MESSAGE_HANDLER_REGISTRY, DownMessageHandler};
use octant_gui_core::{
    define_sys_rpc, DownMessage, FromHandle, NewUpMessage, TypedHandle, UpMessage, UpMessageList,
};
use octant_serde::define_serde_impl;
use wasm_error::WasmError;

use crate::{
    credential_request_options::ArcCredentialRequestOptions,
    request::ArcRequest,
    request_init::{ArcRequestInit, RequestInit},
};
use crate::credential_creation_options::ArcCredentialCreationOptions;
use crate::window::{ArcWindow, Window, WindowTag, WindowValue};

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
    pub fn window(&self) -> &ArcWindow {
        self.window.get_or_init(|| window(&self.runtime))
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
        Ok((web_sys::window().unwrap(),))
    }
}

define_sys_rpc! {
    fn new_request_init(_ctx) -> (RequestInit, ) {
        Ok((web_sys::RequestInit::new(),))
    }
}
