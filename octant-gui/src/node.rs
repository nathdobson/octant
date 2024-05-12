use std::collections::HashSet;

use atomic_refcell::AtomicRefCell;
use by_address::ByAddress;

use octant_gui_core::{NodeMethod, NodeTag};
use octant_gui_core::Method;
use octant_object::define_class;

use crate::handle::HandleValue;
use crate::object::{Object, ObjectValue};
use crate::runtime::HasTypedHandle;

#[derive(Debug)]
struct State {
    children: HashSet<ByAddress<ArcNode>>,
}

define_class! {
    #[derive(Debug)]
    pub class Node extends Object {
        state: AtomicRefCell<State>
    }
}

impl HasTypedHandle for NodeValue {
    type TypeTag = NodeTag;
}

impl NodeValue {
    fn invoke(&self, method: NodeMethod) -> HandleValue {
        (**self).invoke(Method::Node(self.typed_handle(), method))
    }

    pub fn new(handle: HandleValue) -> Self {
        NodeValue {
            parent: ObjectValue::new(handle),
            state: AtomicRefCell::new(State {
                children: HashSet::new(),
            }),
        }
    }
    pub fn append_child(&self, child: ArcNode) {
        self.invoke(NodeMethod::AppendChild(child.typed_handle()));
        self.state.borrow_mut().children.insert(ByAddress(child));
    }
    pub fn remove_child(&self, child: ArcNode) {
        self.invoke(NodeMethod::RemoveChild(child.typed_handle()));
        self.state.borrow_mut().children.remove(&ByAddress(child));
    }

    pub fn set_node_value(&self, value: Option<String>) {
        self.invoke(NodeMethod::SetNodeValue(value));
    }
}
