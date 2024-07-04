use std::rc::Rc;

use marshal_pointer::rc_ref::RcRef;

use octant_object::{class, DebugClass};
use octant_runtime::{
    DeserializePeer, peer::AsNative, PeerNew, rpc, runtime::Runtime, SerializePeer,
};

use crate::{
    object::{Object, ObjectFields},
    public_key_credential_creation_options::PublicKeyCredentialCreationOptions,
};
#[cfg(side = "client")]
use crate::export::Export;

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
