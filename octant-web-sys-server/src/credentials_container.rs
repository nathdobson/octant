use crate::{
    credential_creation_options::ArcCredentialCreationOptions, credential_data::CredentialData,
    credential_request_options::ArcCredentialRequestOptions,
};
use octant_gui_core::define_sys_class;
use std::sync::Arc;

use crate::object::Object;

define_sys_class! {
    class CredentialsContainer;
    extends Object;
    wasm web_sys::CredentialsContainer;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn CredentialsContainer {
    pub async fn get_with_options(
        self: &Arc<Self>,
        req: &ArcCredentialRequestOptions,
    ) -> anyhow::Result<CredentialData> {
        todo!()
    }
    pub async fn create_with_options(
        self: &Arc<Self>,
        req: &ArcCredentialCreationOptions,
    ) -> anyhow::Result<CredentialData> {
        todo!()
    }
}
