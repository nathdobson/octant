use web_sys::Node;

use octant_gui_core::HandleId;
use octant_gui_core::node::NodeTag;
use octant_object::define_class;

use crate::{HasLocalType, object};

define_class! {
    pub class extends object {
        node: Node,
    }
}

impl Value {
    pub fn new(handle: HandleId, node: Node) -> Self {
        Value {
            parent: object::Value::new(handle, node.clone().into()),
            node,
        }
    }
    pub fn native(&self) -> &Node {
        &self.node
    }
}

impl HasLocalType for NodeTag {
    type Local = dyn Trait;
}
