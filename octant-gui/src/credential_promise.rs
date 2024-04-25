use octant_gui_core::{
    Credential, CredentialPromiseMethod, CredentialPromiseTag, CredentialPromiseUpMessage, Error,
    Method,
};
use octant_object::define_class;

use crate::{
    handle, object,
    promise::Completable,
    runtime::{HasLocalType, HasTypedHandle},
};

define_class! {
    #[derive(Debug)]
    pub class extends object{
        completable: Completable<Result<Credential, Error>>,
    }
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
            completable: Completable::new(),
        }
    }
}

impl dyn Trait {
    fn invoke(&self, method: CredentialPromiseMethod) -> handle::Value {
        (**self).invoke(Method::CredentialPromise(self.typed_handle(), method))
    }
    pub fn wait(&self) {
        self.invoke(CredentialPromiseMethod::Wait);
    }
    pub async fn get(&self) -> Result<Credential, Error> {
        self.completable.recv().await
    }
    pub fn handle_event(&self, message: CredentialPromiseUpMessage) {
        match message {
            CredentialPromiseUpMessage::Done(credential) => self.done(credential),
        }
    }
    pub fn done(&self, credential: Result<Credential, Error>) {
        self.completable.send(credential)
    }
}

impl HasTypedHandle for Value {
    type TypeTag = CredentialPromiseTag;
}

impl HasLocalType for CredentialPromiseTag {
    type Local = dyn Trait;
}
