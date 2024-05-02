use std::sync::Arc;

use web_sys::{CredentialCreationOptions, CredentialRequestOptions};

use octant_gui_core::{GlobalMethod, HandleId};

use crate::{credential_creation_options, credential_request_options, peer, window};

pub fn invoke_with(method: &GlobalMethod, handle: HandleId) -> Option<Arc<dyn peer::Trait>> {
    match method {
        GlobalMethod::Window => Some(Arc::new(window::Value::new(handle, window().unwrap()))),
        GlobalMethod::NewCredentialCreationOptions => Some(Arc::new(
            credential_creation_options::Value::new(handle, CredentialCreationOptions::new()),
        )),
        GlobalMethod::NewCredentialRequestOptions => Some(Arc::new(
            credential_request_options::Value::new(handle, CredentialRequestOptions::new()),
        )),
    }
}
