use std::sync::Arc;

use octant_gui_core::define_sys_class;

use crate::{credentials_container::ArcCredentialsContainer, object::Object};

define_sys_class! {
    class Navigator;
    extends Object;
    wasm web_sys::Navigator;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn Navigator {
    pub fn credentials<'a>(self: &'a Arc<Self>) -> &'a ArcCredentialsContainer {
        todo!();
    }
}
