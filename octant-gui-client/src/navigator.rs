use std::sync::Arc;

use octant_gui_core::{HandleId, NavigatorMethod, NavigatorTag};
use octant_object::define_class;

use crate::{
    credentials_container,
    credentials_container::{ArcCredentialsContainer, CredentialsContainerValue},
    object,
    object::{Object, ObjectValue},
    peer, HasLocalType,
};
use crate::peer::ArcPeer;

define_class! {
    pub class Navigator extends Object {
        navigator: web_sys::Navigator,
    }
}

impl NavigatorValue {
    pub fn new(handle: HandleId, navigator: web_sys::Navigator) -> Self {
        NavigatorValue {
            parent: ObjectValue::new(handle, navigator.clone().into()),
            navigator,
        }
    }
    pub fn credentials(&self, handle: HandleId) -> ArcCredentialsContainer {
        Arc::new(CredentialsContainerValue::new(
            handle,
            self.navigator.credentials(),
        ))
    }
    pub fn invoke_with(
        &self,
        method: &NavigatorMethod,
        handle: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            NavigatorMethod::Credentials => Some(self.credentials(handle)),
        }
    }
}

impl HasLocalType for NavigatorTag {
    type Local = dyn Navigator;
}
