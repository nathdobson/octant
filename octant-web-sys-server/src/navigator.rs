use octant_object::{class, DebugClass};
use safe_once::cell::OnceCell;
use std::rc::Rc;

use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{
    peer::AsNative, rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer,
};

use crate::{
    credentials_container::{
        CredentialsContainer, CredentialsContainerFields, RcCredentialsContainer,
    },
    object::{Object, ObjectFields},
};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct NavigatorFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::Navigator,
    #[cfg(side = "server")]
    credentials_container: OnceCell<RcCredentialsContainer>,
}

#[class]
pub trait Navigator: Object {
    #[cfg(side = "server")]
    fn credentials<'a>(self: &'a RcRef<Self>) -> &'a RcCredentialsContainer {
        self.navigator()
            .credentials_container
            .get_or_init(|| self.credentials_impl())
    }
}

#[rpc]
impl dyn Navigator {
    #[rpc]
    fn credentials_impl(self: &RcRef<Self>, _: &Rc<Runtime>) -> RcCredentialsContainer {
        Ok(RcCredentialsContainer::peer_new(
            self.native().credentials(),
        ))
    }
}
