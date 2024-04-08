use std::sync::OnceLock;

use crate::{
    credentials_container, handle, navigator, node, object, runtime::HasTypedHandle,
    CredentialsContainer, Document, Navigator,
};
use octant_gui_core::{
    navigator::{NavigatorMethod, NavigatorTag},
    window::WindowMethod,
    Method,
};
use octant_object::define_class;

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
