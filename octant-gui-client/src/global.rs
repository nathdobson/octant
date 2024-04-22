use std::sync::Arc;

use web_sys::CredentialCreationOptions;

use octant_gui_core::{global::GlobalMethod, HandleId};

use crate::{credential_creation_options, peer, window};

pub fn invoke_with(method: &GlobalMethod, handle: HandleId) -> Option<Arc<dyn peer::Trait>> {
    match method {
        GlobalMethod::Window => Some(Arc::new(window::Value::new(handle, window().unwrap()))),
        GlobalMethod::NewCredentialCreationOptions => Some(Arc::new(
            credential_creation_options::Value::new(handle, CredentialCreationOptions::new()),
        )),
    }
}
