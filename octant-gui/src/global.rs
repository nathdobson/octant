use std::sync::Arc;

use octant_gui_core::{DownMessage, GlobalMethod, Method};

use crate::{credential_creation_options, credential_request_options, CredentialCreationOptions, CredentialRequestOptions, Request, request, request_init, RequestInit, runtime::Runtime, window, Window};
use crate::runtime::HasTypedHandle;

pub struct Global {
    runtime: Arc<Runtime>,
    window: Window,
}

impl Global {
    pub fn new(root: Arc<Runtime>) -> Arc<Self> {
        Arc::new(Global {
            runtime: root.clone(),
            window: root.add(window::Value::new(
                root.invoke(Method::Global(GlobalMethod::Window)),
            )),
        })
    }
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn new_credential_creation_options(&self) -> CredentialCreationOptions {
        self.runtime.add(credential_creation_options::Value::new(
            self.runtime
                .invoke(Method::Global(GlobalMethod::NewCredentialCreationOptions)),
        ))
    }
    pub fn new_credential_request_options(&self) -> CredentialRequestOptions {
        self.runtime
            .add(credential_request_options::Value::new(self.runtime.invoke(
                Method::Global(GlobalMethod::NewCredentialRequestOptions),
            )))
    }
    pub fn new_request_init(&self) -> RequestInit {
        self.runtime.add(request_init::Value::new(
            self.runtime
                .invoke(Method::Global(GlobalMethod::NewRequestInit)),
        ))
    }
    pub fn new_request(&self, url: String, init: &RequestInit) -> Request {
        self.runtime
            .add(request::Value::new(self.runtime.invoke(
                Method::Global(GlobalMethod::NewRequest(url, init.typed_handle())),
            )))
    }
    pub fn fail(&self, e: anyhow::Error) {
        self.runtime.send(DownMessage::Fail(format!("{}", e)));
    }
}
