use octant_object::{class, DebugClass};
use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_rpc, peer::AsNative, DeserializePeer, PeerNew, SerializePeer};

#[cfg(side = "client")]
use crate::export::Export;
use crate::{
    object::Object, public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
};
use crate::object::ObjectFields;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct CredentialCreationOptionsFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::CredentialCreationOptions,
}

#[class]
pub trait CredentialCreationOptions: Object {
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
