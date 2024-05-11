use std::sync::Arc;

use web_sys::{CredentialCreationOptions, CredentialRequestOptions, Request, RequestInit};

use octant_gui_core::{GlobalMethod, HandleId};

use crate::{
    credential_creation_options, credential_request_options, peer, peer::ArcPeer, request,
    request_init, window, Runtime,
};
use crate::credential_creation_options::CredentialCreationOptionsValue;
use crate::credential_request_options::CredentialRequestOptionsValue;
use crate::request::RequestValue;
use crate::request_init::RequestInitValue;
use crate::window::WindowValue;

pub fn invoke_with(
    runtime: &Arc<Runtime>,
    method: &GlobalMethod,
    handle: HandleId,
) -> Option<ArcPeer> {
    match method {
        GlobalMethod::Window => Some(Arc::new(WindowValue::new(handle, window().unwrap()))),
        GlobalMethod::NewCredentialCreationOptions => Some(Arc::new(
            CredentialCreationOptionsValue::new(handle, CredentialCreationOptions::new()),
        )),
        GlobalMethod::NewCredentialRequestOptions => Some(Arc::new(
            CredentialRequestOptionsValue::new(handle, CredentialRequestOptions::new()),
        )),
        GlobalMethod::NewRequestInit => {
            Some(Arc::new(RequestInitValue::new(handle, RequestInit::new())))
        }
        GlobalMethod::NewRequest(url, init) => Some(Arc::new(RequestValue::new(
            handle,
            Request::new_with_str_and_init(url, runtime.handle(*init).native()).unwrap(),
        ))),
    }
}
