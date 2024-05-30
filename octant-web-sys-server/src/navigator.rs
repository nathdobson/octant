use octant_object::class;
use safe_once::cell::OnceCell;

use octant_reffed::rc::{Rc2, RcRef};
use octant_runtime::{define_sys_rpc, peer::AsNative, DeserializePeer, PeerNew, SerializePeer};

use crate::{
    credential::AsCredential,
    credentials_container::{
        CredentialsContainer, CredentialsContainerValue, RcCredentialsContainer,
    },
    object::Object,
};

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct Navigator {
    parent: dyn Object,
    #[cfg(side = "client")]
    any_value: web_sys::Navigator,
    #[cfg(side = "server")]
    credentials_container: OnceCell<RcCredentialsContainer>,
}

pub trait Navigator: AsNavigator {
    #[cfg(side = "server")]
    fn credentials<'a>(self: &'a RcRef<Self>) -> &'a RcCredentialsContainer;
}

impl<T> Navigator for T
where
    T: AsNavigator,
{
    #[cfg(side = "server")]
    fn credentials<'a>(self: &'a RcRef<Self>) -> &'a RcCredentialsContainer {
        self.navigator()
            .credentials_container
            .get_or_init(|| credentials(self.runtime(), self.rc()))
    }
}

define_sys_rpc! {
    fn credentials(_runtime:_, navigator:RcNavigator) -> RcCredentialsContainer{
        Ok(Rc2::new(CredentialsContainerValue::peer_new(navigator.native().credentials())))
    }
}
