use octant_object::class;
use octant_reffed::rc::RcRef;
use octant_runtime::{
     define_sys_rpc, peer::AsNative, DeserializePeer, PeerNew, SerializePeer,
};

#[cfg(side = "client")]
use crate::export::Export;
use crate::{
    object::Object, public_key_credential_request_options::PublicKeyCredentialRequestOptions,
};

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct CredentialRequestOptions {
    parent: dyn Object,
    #[cfg(side = "client")]
    any_value: web_sys::CredentialRequestOptions,
}

pub trait CredentialRequestOptions: AsCredentialRequestOptions {
    #[cfg(side = "server")]
    fn public_key(self: &RcRef<Self>, options: PublicKeyCredentialRequestOptions);
}

impl<T> CredentialRequestOptions for T
where
    T: AsCredentialRequestOptions,
{
    #[cfg(side = "server")]
    fn public_key(self: &RcRef<Self>, options: PublicKeyCredentialRequestOptions) {
        public_key(self.runtime(), self.rc(), options)
    }
}

define_sys_rpc! {
    fn public_key(_runtime:_, options:RcCredentialRequestOptions, public_key:PublicKeyCredentialRequestOptions) -> (){
        options.native().clone().public_key(&public_key.export());
        Ok(())
    }
}
