use std::sync::Arc;
use octant_reffed::{Arc2, ArcRef};
use octant_runtime::define_sys_class;

use crate::{
    object::Object, public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
};

define_sys_class! {
    class CredentialCreationOptions;
    extends Object;
    wasm web_sys::CredentialRequestOptions;
    new_client _;
    new_server _;
    server_fn {
        fn public_key(self: &ArcRef<Self>, options: PublicKeyCredentialCreationOptions) {
            todo!();
        }
    }
}
