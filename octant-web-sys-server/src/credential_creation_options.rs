use octant_object::{class, DebugClass};
use octant_reffed::rc::RcRef;
use octant_runtime::{
    peer::AsNative, rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer,
};
use std::rc::Rc;

#[cfg(side = "client")]
use crate::export::Export;
use crate::{
    object::{Object, ObjectFields},
    public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
};

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
        self.public_key_impl(options);
    }
}

#[rpc]
impl dyn CredentialCreationOptions {
    #[rpc]
    fn public_key_impl(
        self: &RcRef<Self>,
        _: &Rc<Runtime>,
        public_key_arg: PublicKeyCredentialCreationOptions,
    ) {
        self.native().clone().public_key(&public_key_arg.export());
        Ok(())
    }
}
