use std::sync::OnceLock;

use octant_gui_core::{Method, NavigatorMethod, NavigatorTag};
use octant_object::define_class;

use crate::{
    credentials_container::ArcCredentialsContainer,
    object::Object,
};
use crate::credentials_container::CredentialsContainerValue;
use crate::handle::HandleValue;
use crate::object::ObjectValue;
use crate::runtime::HasTypedHandle;

define_class! {
    #[derive(Debug)]
    pub class Navigator extends Object {
        credentials:OnceLock<ArcCredentialsContainer>,
    }
}

impl HasTypedHandle for NavigatorValue {
    type TypeTag = NavigatorTag;
}

impl NavigatorValue {
    pub fn new(handle: HandleValue) -> Self {
        NavigatorValue {
            parent: ObjectValue::new(handle),
            credentials: OnceLock::new(),
        }
    }
    pub fn credentials(&self) -> &ArcCredentialsContainer {
        self.credentials.get_or_init(|| {
            self.runtime().add(CredentialsContainerValue::new(
                self.invoke(NavigatorMethod::Credentials),
            ))
        })
    }
}

impl NavigatorValue {
    fn invoke(&self, method: NavigatorMethod) -> HandleValue {
        (**self).invoke(Method::Navigator(self.typed_handle(), method))
    }
}
