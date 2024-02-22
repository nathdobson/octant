use std::sync::Arc;

use octant_gui_core::{HandleId, Method};
use octant_object::define_class;

use crate::handle;
use crate::runtime::Runtime;

define_class! {
    pub class{
        root: Arc<Runtime>,
        id : HandleId,
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        self.root.delete(self.id)
    }
}

impl Value {
    pub fn new(root: Arc<Runtime>, handle: HandleId) -> Self {
        Value { root, id: handle }
    }
    pub fn id(&self) -> HandleId {
        self.id
    }
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.root
    }
}

impl Value {
    pub fn invoke(&self, method: Method) -> handle::Value {
        self.root.invoke(method)
    }
    pub fn handle(&self) -> HandleId {
        self.id
    }
}
