use std::sync::Arc;

use octant_gui_core::{GlobalMethod, Method};

use crate::{
    credential_creation_options, CredentialCreationOptions, runtime::Runtime, window, Window,
};

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
}
