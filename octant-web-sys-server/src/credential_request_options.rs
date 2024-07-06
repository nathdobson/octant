use marshal_pointer::RcfRef;
use std::rc::Rc;

use crate::{
    object::{Object, ObjectFields},
    public_key_credential_request_options::PublicKeyCredentialRequestOptions,
};
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};

use crate::octant_runtime::peer::AsNative;

#[cfg(side = "client")]
use crate::export::Export;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct CredentialRequestOptionsFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::CredentialRequestOptions,
}

#[class]
pub trait CredentialRequestOptions: Object {
    #[cfg(side = "server")]
    fn public_key(self: &RcfRef<Self>, options: PublicKeyCredentialRequestOptions) {
        self.public_key_impl(options);
    }
}

#[rpc]
impl dyn CredentialRequestOptions {
    #[rpc]
    fn public_key_impl(
        self: &RcfRef<Self>,
        _: &Rc<Runtime>,
        public_key_arg: PublicKeyCredentialRequestOptions,
    ) {
        self.native().clone().public_key(&public_key_arg.export());
        Ok(())
    }
}
