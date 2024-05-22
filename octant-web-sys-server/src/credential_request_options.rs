use octant_gui_core::define_sys_class;
use std::sync::Arc;

use crate::{
    object::Object, public_key_credential_request_options::PublicKeyCredentialRequestOptions,
};

define_sys_class! {
    class CredentialRequestOptions;
    extends Object;
    wasm web_sys::CredentialRequestOptions;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn CredentialRequestOptions {
    pub fn public_key(self: &Arc<Self>, options: PublicKeyCredentialRequestOptions) {
        todo!();
    }
}
