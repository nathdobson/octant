use crate::{
    credential_creation_options::ArcCredentialCreationOptions, credential_data::CredentialData,
    credential_request_options::ArcCredentialRequestOptions,
};
use std::sync::Arc;
use octant_reffed::Arc2;
use octant_runtime::define_sys_class;

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
        self: &Arc2<Self>,
        req: &ArcCredentialRequestOptions,
    ) -> anyhow::Result<CredentialData> {
        todo!()
    }
    pub async fn create_with_options(
        self: &Arc2<Self>,
        req: &ArcCredentialCreationOptions,
    ) -> anyhow::Result<CredentialData> {
        todo!()
    }
}
