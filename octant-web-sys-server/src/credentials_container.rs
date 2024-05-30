#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

use octant_error::OctantError;
use octant_object::{class, DebugClass};
use octant_reffed::rc::Rc2;
use octant_runtime::{
    define_sys_rpc, future_return::DataReturn, octant_future::OctantFuture, peer::AsNative,
    DeserializePeer, PeerNew, SerializePeer,
};

#[cfg(side = "client")]
use crate::import::Import;
use crate::{
    credential_creation_options::RcCredentialCreationOptions, credential_data::CredentialData,
    credential_request_options::RcCredentialRequestOptions, object::Object,
};
use crate::object::ObjectValue;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct CredentialsContainerValue {
    parent: ObjectValue,
    #[cfg(side = "client")]
    any_value: web_sys::CredentialsContainer,
}

#[class]
pub trait CredentialsContainer : Object {}

#[cfg(side = "server")]
impl dyn CredentialsContainer {
    pub async fn get_with_options(
        self: &Rc2<Self>,
        req: RcCredentialRequestOptions,
    ) -> anyhow::Result<CredentialData> {
        Ok(get_with_options(self.runtime(), self.clone(), req)
            .await?
            .into_inner()?)
    }
    pub async fn create_with_options(
        self: &Rc2<Self>,
        req: RcCredentialCreationOptions,
    ) -> anyhow::Result<CredentialData> {
        Ok(create_with_options(self.runtime(), self.clone(), req)
            .await?
            .into_inner()?)
    }
}

define_sys_rpc! {
    fn get_with_options(
        runtime:_,
        credentials: RcCredentialsContainer,
        options:RcCredentialRequestOptions
    ) -> OctantFuture<DataReturn<Result<CredentialData, OctantError>>>{
        let fut = credentials.native().get_with_options(options.native()).map_err(OctantError::from)?;
        Ok(OctantFuture::spawn(runtime, async move{
            let data= JsFuture::from(fut).await;
            DataReturn::new(Import::<Result<CredentialData,OctantError>>::import(&data))
        }))
    }
    fn create_with_options(
        runtime:_,
        credentials: RcCredentialsContainer,
        options:RcCredentialCreationOptions
    ) -> OctantFuture<DataReturn<Result<CredentialData, OctantError>>>{
        let fut = credentials.native().create_with_options(options.native()).map_err(OctantError::from)?;
        Ok(OctantFuture::spawn(runtime, async move{
            let data= JsFuture::from(fut).await;
            DataReturn::new(Import::<Result<CredentialData,OctantError>>::import(&data))
        }))
    }
}
