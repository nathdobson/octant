use octant_gui_core::{
    CredentialData, CredentialMethod, CredentialTag, CredentialUpMessage

    , Method,
};
use octant_object::define_class;

use crate::{
    handle, object,
    promise::Completable,
    runtime::{HasLocalType, HasTypedHandle}
    ,
};

define_class! {
    #[derive(Debug)]
    pub class extends object {
        data: Completable<CredentialData>,
    }
}

impl HasTypedHandle for Value {
    type TypeTag = CredentialTag;
}
impl HasLocalType for CredentialTag {
    type Local = dyn Trait;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
            data: Completable::new(),
        }
    }
    fn invoke(&self, method: CredentialMethod) -> handle::Value {
        (**self).invoke(Method::Credential(self.typed_handle(), method))
    }
    pub async fn materialize(&self) -> CredentialData {
        self.invoke(CredentialMethod::Materialize);
        self.runtime().flush().await.unwrap();
        self.data.recv().await
    }
}

impl dyn Trait {
    pub fn handle_event(&self, message: CredentialUpMessage) {
        match message {
            CredentialUpMessage::Materialize(x) => {
                self.data.send(x);
            }
        }
    }
}
