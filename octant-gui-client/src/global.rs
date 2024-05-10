use std::sync::Arc;

use web_sys::{CredentialCreationOptions, CredentialRequestOptions, Request, RequestInit};

use octant_gui_core::{GlobalMethod, HandleId};

use crate::{credential_creation_options, credential_request_options, peer, request, request_init, Runtime, window};

pub fn invoke_with(
    runtime: &Arc<Runtime>,
    method: &GlobalMethod,
    handle: HandleId,
) -> Option<Arc<dyn peer::Trait>> {
    match method {
        GlobalMethod::Window => Some(Arc::new(window::Value::new(handle, window().unwrap()))),
        GlobalMethod::NewCredentialCreationOptions => Some(Arc::new(
            credential_creation_options::Value::new(handle, CredentialCreationOptions::new()),
        )),
        GlobalMethod::NewCredentialRequestOptions => Some(Arc::new(
            credential_request_options::Value::new(handle, CredentialRequestOptions::new()),
        )),
        GlobalMethod::NewRequestInit => Some(Arc::new(request_init::Value::new(
            handle,
            RequestInit::new(),
        ))),
        GlobalMethod::NewRequest(url, init) => Some(Arc::new(request::Value::new(
            handle,
            Request::new_with_str_and_init(url, runtime.handle(*init).native()).unwrap(),
        ))),
    }
}
