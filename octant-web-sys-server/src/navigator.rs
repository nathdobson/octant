use octant_reffed::arc::ArcRef;
use octant_runtime::define_sys_class;
use std::sync::Arc;

use crate::{credentials_container::ArcCredentialsContainer, object::Object};

define_sys_class! {
    class Navigator;
    extends Object;
    wasm web_sys::Navigator;
    new_client _;
    new_server _;
    server_fn {
        fn credentials<'a>(self: &'a ArcRef<Self>) -> &'a ArcCredentialsContainer {
            todo!();
        }
    }
}
