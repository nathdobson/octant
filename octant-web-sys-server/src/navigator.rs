use octant_object::{class, DebugClass};
use safe_once::cell::OnceCell;

use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{define_sys_rpc, peer::AsNative, DeserializePeer, PeerNew, SerializePeer};

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
            .get_or_init(|| credentials(self.runtime(), self.rc()))
    }
}

define_sys_rpc! {
    fn credentials(_runtime:_, navigator:RcNavigator) -> RcCredentialsContainer{
        Ok(Rc2::new(CredentialsContainerFields::peer_new(navigator.native().credentials())))
    }
}
