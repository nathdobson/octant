use marshal_pointer::rc_ref::RcRef;
use std::rc::Rc;

#[cfg(side = "client")]
use crate::import::Import;
use crate::{
    credential_creation_options::RcCredentialCreationOptions,
    credential_data::CredentialData,
    credential_request_options::RcCredentialRequestOptions,
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use octant_error::{OctantError, OctantResult};
use octant_object::{class, DebugClass};
use octant_runtime::{
    future_return::DataReturn, octant_future::OctantFuture, rpc, runtime::Runtime, DeserializePeer,
    PeerNew, SerializePeer,
};
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct CredentialsContainerFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::CredentialsContainer,
}

#[class]
pub trait CredentialsContainer: Object {}

#[cfg(side = "server")]
impl dyn CredentialsContainer {
    pub async fn get_with_options(
        self: &RcRef<Self>,
        req: RcCredentialRequestOptions,
    ) -> OctantResult<CredentialData> {
        Ok(self.get_with_options_impl(req).await?.into_inner()?)
    }
    pub async fn create_with_options(
        self: &RcRef<Self>,
        req: RcCredentialCreationOptions,
    ) -> OctantResult<CredentialData> {
        Ok(self.create_with_options_impl(req).await?.into_inner()?)
    }
}

#[rpc]
impl dyn CredentialsContainer {
    #[rpc]
    fn get_with_options_impl(
        self: &RcRef<Self>,
        runtime: &Rc<Runtime>,
        options: RcCredentialRequestOptions,
    ) -> OctantFuture<DataReturn<Result<CredentialData, OctantError>>> {
        let fut = self
            .native()
            .get_with_options(options.native())
            .map_err(OctantError::from)?;
        Ok(OctantFuture::spawn(runtime, async move {
            let data = JsFuture::from(fut).await;
            DataReturn::new(Import::<Result<CredentialData, OctantError>>::import(&data))
        }))
    }

    #[rpc]
    fn create_with_options_impl(
        self: &RcRef<Self>,
        runtime: &Rc<Runtime>,
        options: RcCredentialCreationOptions,
    ) -> OctantFuture<DataReturn<Result<CredentialData, OctantError>>> {
        let fut = self
            .native()
            .create_with_options(options.native())
            .map_err(OctantError::from)?;
        Ok(OctantFuture::spawn(runtime, async move {
            let data = JsFuture::from(fut).await;
            DataReturn::new(Import::<Result<CredentialData, OctantError>>::import(&data))
        }))
    }
}
