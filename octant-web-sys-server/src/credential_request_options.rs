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
    public_key_credential_request_options::PublicKeyCredentialRequestOptions,
};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct CredentialRequestOptionsFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::CredentialRequestOptions,
}

#[class]
pub trait CredentialRequestOptions: Object {
    #[cfg(side = "server")]
    fn public_key(self: &RcRef<Self>, options: PublicKeyCredentialRequestOptions) {
        public_key(self.runtime(), self.rc(), options)
    }
}

#[rpc]
fn public_key(
    _: &Rc<Runtime>,
    options: RcCredentialRequestOptions,
    public_key: PublicKeyCredentialRequestOptions,
) {
    options.native().clone().public_key(&public_key.export());
    Ok(())
}
