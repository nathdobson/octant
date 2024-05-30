use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_class, define_sys_rpc};
use octant_runtime::peer::AsNative;

use crate::{
    object::Object, public_key_credential_request_options::PublicKeyCredentialRequestOptions,
};
#[cfg(side="client")]
use crate::export::Export;

define_sys_class! {
    class CredentialRequestOptions;
    extends Object;
    wasm web_sys::CredentialRequestOptions;
    new_client _;
    new_server _;
    server_fn {
        fn public_key(self: &RcRef<Self>, options: PublicKeyCredentialRequestOptions) {
            public_key(self.runtime(), self.rc(), options)
        }
    }
}

define_sys_rpc! {
    fn public_key(_runtime:_, options:RcCredentialRequestOptions, public_key:PublicKeyCredentialRequestOptions) -> (){
        options.native().clone().public_key(&public_key.export());
        Ok(())
    }
}
