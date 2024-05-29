use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_class, define_sys_rpc};

use crate::{
    object::Object, public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
};
#[cfg(side = "client")]
use crate::export::Export;

define_sys_class! {
    class CredentialCreationOptions;
    extends Object;
    wasm web_sys::CredentialCreationOptions;
    new_client _;
    new_server _;
    server_fn {
        fn public_key(self: &RcRef<Self>, options: PublicKeyCredentialCreationOptions) {
            public_key(self.runtime(), self.rc(), options)
        }
    }
}

define_sys_rpc! {
    fn public_key(_runtime:_, options:RcCredentialCreationOptions, public_key:PublicKeyCredentialCreationOptions) -> (){
        options.native().clone().public_key(&public_key.export());
        Ok(())
    }
}
