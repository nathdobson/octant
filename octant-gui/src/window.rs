use std::sync::OnceLock;

use octant_gui_core::Method;
use octant_gui_core::window::{WindowMethod, WindowTag};
use octant_object::define_class;

use crate::{document, Document, handle, node};
use crate::runtime::HasTypedHandle;

define_class! {
    #[derive(Debug)]
    pub class extends node {
        document: OnceLock<Document>,
    }
}

impl HasTypedHandle for Value {
    type TypeTag = WindowTag;
}

impl Value {
    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: node::Value::new(handle),
            document: OnceLock::new(),
        }
    }
}

impl Value {
    fn invoke(&self, method: WindowMethod) -> handle::Value {
        (**self).invoke(Method::Window(self.typed_handle(), method))
    }
    pub fn document(&self) -> &Document {
        self.document.get_or_init(|| {
            self.runtime()
                .add(document::Value::new(self.invoke(WindowMethod::Document)))
        })
    }
}
