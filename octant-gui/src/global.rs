use std::sync::Arc;

use octant_gui_core::{DownMessage, GlobalMethod, Method};
use crate::{Runtime, window};
use crate::credential_creation_options::{ArcCredentialCreationOptions, CredentialCreationOptionsValue};
use crate::credential_request_options::{ArcCredentialRequestOptions, CredentialRequestOptionsValue};
use crate::request::{ArcRequest, RequestValue};
use crate::request_init::{ArcRequestInit, RequestInitValue};

use crate::runtime::HasTypedHandle;
use crate::window::{ArcWindow, Window, WindowValue};

pub struct Global {
    runtime: Arc<Runtime>,
    window: ArcWindow,
}

impl Global {
    pub fn new(root: Arc<Runtime>) -> Arc<Self> {
        Arc::new(Global {
            runtime: root.clone(),
            window: root.add(WindowValue::new(
                root.invoke(Method::Global(GlobalMethod::Window)),
            )),
        })
    }
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
    pub fn window(&self) -> &ArcWindow {
        &self.window
    }
    pub fn new_credential_creation_options(&self) -> ArcCredentialCreationOptions {
        self.runtime.add(CredentialCreationOptionsValue::new(
            self.runtime
                .invoke(Method::Global(GlobalMethod::NewCredentialCreationOptions)),
        ))
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
            .add(RequestValue::new(self.runtime.invoke(
                Method::Global(GlobalMethod::NewRequest(url, init.typed_handle())),
            )))
    }
    pub fn fail(&self, e: anyhow::Error) {
        self.runtime.send(DownMessage::Fail(format!("{}", e)));
    }
}
