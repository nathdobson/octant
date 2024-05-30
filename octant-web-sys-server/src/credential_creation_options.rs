use octant_object::class;
use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_rpc, peer::AsNative, DeserializePeer, PeerNew, SerializePeer};

#[cfg(side = "client")]
use crate::export::Export;
use crate::{
    object::Object, public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
};

#[class]
#[derive(PeerNew, SerializePeer, DeserializePeer)]
pub struct CredentialCreationOptions {
    parent: dyn Object,
    #[cfg(side = "client")]
    any_value: web_sys::CredentialCreationOptions,
}

pub trait CredentialCreationOptions: AsCredentialCreationOptions {
    #[cfg(side = "server")]
    fn public_key(self: &RcRef<Self>, options: PublicKeyCredentialCreationOptions);
}

impl<T> CredentialCreationOptions for T
where
    T: AsCredentialCreationOptions,
{
    #[cfg(side = "server")]
    fn public_key(self: &RcRef<Self>, options: PublicKeyCredentialCreationOptions) {
        public_key(self.runtime(), self.rc(), options)
    }
}

define_sys_rpc! {
    fn public_key(_runtime:_, options:RcCredentialCreationOptions, public_key:PublicKeyCredentialCreationOptions) -> (){
        options.native().clone().public_key(&public_key.export());
        Ok(())
    }
}
