use std::{marker::PhantomData, sync::Arc};

use web_sys::Credential;

use octant_gui_core::{
    CredentialMethod, CredentialTag, CredentialUpMessage, HandleId,
    TypedHandle, UpMessage, UpMessageList,
};
use octant_object::define_class;

use crate::{HasLocalType, object, peer, Runtime};
use crate::import::Import;

define_class! {
    pub class extends object {
        credential: Credential,
    }
}

impl Value {
    pub fn new(handle: HandleId, credential: Credential) -> Self {
        Value {
            parent: object::Value::new(handle, credential.clone().into()),
            credential,
        }
    }
    pub fn native(&self) -> &Credential {
        &self.credential
    }
    pub fn handle(&self) -> TypedHandle<CredentialTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Trait {
    pub fn invoke_with(
        self: &Arc<Self>,
        runtime: &Arc<Runtime>,
        method: &CredentialMethod,
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            CredentialMethod::Materialize => {
                runtime
                    .send(UpMessageList {
                        commands: vec![UpMessage::Credential(
                            self.handle(),
                            CredentialUpMessage::Materialize(self.native().import()),
                        )],
                    })
                    .unwrap();
                None
            }
        }
    }
}

impl HasLocalType for CredentialTag {
    type Local = dyn Trait;
}
