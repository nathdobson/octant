use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use octant_gui_core::{HandleId, Method};
use octant_object::{base, define_class};
use octant_object::base::Base;

use crate::handle;
use crate::runtime::Runtime;

pub trait ParentTrait = Send + Sync + Any + Debug;

define_class! {
    pub class Handle extends Base implements ParentTrait{
        root: Arc<Runtime>,
        id : HandleId,
    }
}

impl Debug for HandleValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

impl Drop for HandleValue {
    fn drop(&mut self) {
        self.root.delete(self.id)
    }
}

impl HandleValue {
    pub fn new(root: Arc<Runtime>, handle: HandleId) -> Self {
        HandleValue {
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

impl HandleValue {
    pub fn invoke(&self, method: Method) -> HandleValue {
        self.root.invoke(method)
    }
    pub fn handle(&self) -> HandleId {
        self.id
    }
}
