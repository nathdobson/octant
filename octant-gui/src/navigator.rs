use std::sync::OnceLock;

use octant_gui_core::{
    {NavigatorMethod, NavigatorTag}
    ,
    Method,
};
use octant_object::define_class;

use crate::{
    credentials_container, CredentialsContainer, handle, object,
    runtime::HasTypedHandle,
};

define_class! {
    #[derive(Debug)]
    pub class extends object {
        credentials:OnceLock<CredentialsContainer>,
    }
}

impl HasTypedHandle for Value {
    type TypeTag = NavigatorTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
            credentials: OnceLock::new(),
        }
    }
    pub fn credentials(&self) -> &CredentialsContainer {
        self.credentials.get_or_init(|| {
            self.runtime().add(credentials_container::Value::new(
                self.invoke(NavigatorMethod::Credentials),
            ))
        })
    }
}

impl Value {
    fn invoke(&self, method: NavigatorMethod) -> handle::Value {
        (**self).invoke(Method::Navigator(self.typed_handle(), method))
    }
}
