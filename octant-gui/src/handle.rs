use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use octant_gui_core::{HandleId, Method};
use octant_object::{base, define_class};

use crate::handle;
use crate::runtime::Runtime;

pub trait ParentTrait = Send + Sync + Any + Debug;

define_class! {
    pub class extends base implements ParentTrait{
        root: Arc<Runtime>,
        id : HandleId,
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        self.root.delete(self.id)
    }
}

impl Value {
    pub fn new(root: Arc<Runtime>, handle: HandleId) -> Self {
        Value {
            parent: base::Value::new(),
            root,
            id: handle,
        }
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
