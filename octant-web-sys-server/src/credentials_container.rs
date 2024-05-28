#[cfg(side = "client")]
use crate::import::Import;
use crate::{
    credential_creation_options::ArcCredentialCreationOptions, credential_data::CredentialData,
    credential_request_options::ArcCredentialRequestOptions,
};
use octant_reffed::arc::Arc2;
use octant_runtime::{
    define_sys_class, define_sys_rpc, future_return::DataReturn,
    octant_future::OctantFuture,
};
use std::sync::Arc;
use octant_error::OctantError;
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

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
        req: ArcCredentialRequestOptions,
    ) -> anyhow::Result<CredentialData> {
        Ok(get_with_options(self.runtime(), self.clone(), req)
            .await?
            .into_inner()?)
    }
    pub async fn create_with_options(
        self: &Arc2<Self>,
        req: ArcCredentialCreationOptions,
    ) -> anyhow::Result<CredentialData> {
        Ok(create_with_options(self.runtime(), self.clone(), req)
            .await?
            .into_inner()?)
    }
}

define_sys_rpc! {
    fn get_with_options(
        runtime:_,
        credentials: ArcCredentialsContainer,
        options:ArcCredentialRequestOptions
    ) -> OctantFuture<DataReturn<Result<CredentialData, OctantError>>>{
        let fut = credentials.native().get_with_options(options.native()).map_err(OctantError::from)?;
        Ok(OctantFuture::spawn(runtime, async move{
            let data= JsFuture::from(fut).await;
            DataReturn::new(Import::<Result<CredentialData,OctantError>>::import(&data))
        }))
    }
    fn create_with_options(
        runtime:_,
        credentials: ArcCredentialsContainer,
        options:ArcCredentialCreationOptions
    ) -> OctantFuture<DataReturn<Result<CredentialData, OctantError>>>{
        let fut = credentials.native().create_with_options(options.native()).map_err(OctantError::from)?;
        Ok(OctantFuture::spawn(runtime, async move{
            let data= JsFuture::from(fut).await;
            DataReturn::new(Import::<Result<CredentialData,OctantError>>::import(&data))
        }))
    }
}
