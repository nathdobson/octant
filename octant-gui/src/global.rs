use std::sync::Arc;

use safe_once::sync::OnceLock;

use octant_gui_core::{DownMessage, GlobalMethod, Method};

use crate::{
    credential_creation_options::{ArcCredentialCreationOptions, CredentialCreationOptionsValue},
    credential_request_options::{ArcCredentialRequestOptions, CredentialRequestOptionsValue},
    request::{ArcRequest, RequestValue},
    request_init::{ArcRequestInit, RequestInitValue},
    Runtime,
    runtime::HasTypedHandle,
    window::{ArcWindow, WindowValue},
};

pub struct Global {
    runtime: Arc<Runtime>,
    window: OnceLock<ArcWindow>,
}

impl Global {
    pub fn new(runtime: Arc<Runtime>) -> Arc<Self> {
        Arc::new(Global {
            runtime,
            window: OnceLock::new(),
        })
    }
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
    pub fn window(&self) -> &ArcWindow {
        self.window.get_or_init(|| {
            self.runtime.add(WindowValue::new(
                self.runtime.invoke(Method::Global(GlobalMethod::Window)),
            ))
        })
    }
    pub fn new_credential_creation_options(&self) -> ArcCredentialCreationOptions {
        self.runtime
            .add(CredentialCreationOptionsValue::new(self.runtime.invoke(
                Method::Global(GlobalMethod::NewCredentialCreationOptions),
            )))
    }
    pub fn new_credential_request_options(&self) -> ArcCredentialRequestOptions {
        self.runtime
            .add(CredentialRequestOptionsValue::new(self.runtime.invoke(
                Method::Global(GlobalMethod::NewCredentialRequestOptions),
            )))
    }
    pub fn new_request_init(&self) -> ArcRequestInit {
        self.runtime.add(RequestInitValue::new(
            self.runtime
                .invoke(Method::Global(GlobalMethod::NewRequestInit)),
        ))
    }
    pub fn new_request(&self, url: String, init: &ArcRequestInit) -> ArcRequest {
        self.runtime
            .add(RequestValue::new(self.runtime.invoke(Method::Global(
                GlobalMethod::NewRequest(url, init.typed_handle()),
            ))))
    }
    pub fn fail(&self, e: anyhow::Error) {
        self.runtime.send(DownMessage::Fail(format!("{}", e)));
    }
}
