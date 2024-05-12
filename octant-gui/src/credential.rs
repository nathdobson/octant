use octant_gui_core::{
    CredentialData, CredentialMethod, CredentialTag, CredentialUpMessage

    , Method,
};
use octant_object::define_class;

use crate::{
    promise::Completable,
    runtime::{HasLocalType, HasTypedHandle}
    ,
};
use crate::handle::HandleValue;
use crate::object::{Object, ObjectValue};

define_class! {
    #[derive(Debug)]
    pub class Credential extends Object {
        data: Completable<CredentialData>,
    }
}

impl HasTypedHandle for CredentialValue {
    type TypeTag = CredentialTag;
}
impl HasLocalType for CredentialTag {
    type Local = dyn Credential;
}

impl CredentialValue {
    pub fn new(handle: HandleValue) -> Self {
        CredentialValue {
            parent: ObjectValue::new(handle),
            data: Completable::new(),
        }
    }
    fn invoke(&self, method: CredentialMethod) -> HandleValue {
        (**self).invoke(Method::Credential(self.typed_handle(), method))
    }
    pub async fn materialize(&self) -> CredentialData {
        self.invoke(CredentialMethod::Materialize);
        // self.runtime().flush().await.unwrap();
        self.data.recv().await
    }
}

impl dyn Credential {
    pub fn handle_event(&self, message: CredentialUpMessage) {
        match message {
            CredentialUpMessage::Materialize(x) => {
                self.data.send(x);
            }
        }
    }
}
