use octant_reffed::arc::{Arc2, ArcRef};
use octant_runtime::{define_sys_class, define_sys_rpc};
use safe_once::sync::OnceLock;
use std::sync::Arc;
use octant_reffed::rc::{Rc2, RcRef};

use crate::{credentials_container::RcCredentialsContainer, object::Object};
use crate::credentials_container::{CredentialsContainer, CredentialsContainerValue};

define_sys_class! {
    class Navigator;
    extends Object;
    wasm web_sys::Navigator;
    new_client _;
    new_server _;
    server_field credentials_container: OnceLock<RcCredentialsContainer>;
    server_fn {
        fn credentials<'a>(self: &'a RcRef<Self>) -> &'a RcCredentialsContainer {
            self.navigator().credentials_container.get_or_init(|| credentials(self.runtime(),self.rc()))
        }
    }
}

define_sys_rpc! {
    fn credentials(_runtime:_, navigator:RcNavigator) -> RcCredentialsContainer{
        Ok(Rc2::new(CredentialsContainerValue::new(navigator.native().credentials())))
    }
}
