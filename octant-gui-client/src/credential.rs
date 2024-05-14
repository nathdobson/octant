use std::{marker::PhantomData, sync::Arc};

use octant_gui_core::{
    CredentialMethod, CredentialTag, CredentialUpMessage, HandleId,
    TypedHandle, UpMessage, UpMessageList,
};
use octant_object::define_class;

use crate::{HasLocalType, Runtime};
use crate::import::Import;
use crate::object::{Object, ObjectValue};
use crate::peer::ArcPeer;

define_class! {
    pub class Credential extends Object {
        credential: web_sys::Credential,
    }
}

impl CredentialValue {
    pub fn new(handle: HandleId, credential: web_sys::Credential) -> Self {
        CredentialValue {
            parent: ObjectValue::new(handle, credential.clone().into()),
            credential,
        }
    }
    pub fn native(&self) -> &web_sys::Credential {
        &self.credential
    }
    pub fn handle(&self) -> TypedHandle<CredentialTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Credential {
    pub fn invoke_with(
        self: &Arc<Self>,
        runtime: &Arc<Runtime>,
        method: &CredentialMethod,
        _handle: HandleId,
    ) -> Option<ArcPeer> {
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
    type Local = dyn Credential;
}
