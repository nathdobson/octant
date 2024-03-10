use std::collections::HashSet;

use atomic_refcell::AtomicRefCell;
use by_address::ByAddress;

use octant_gui_core::Method;
use octant_gui_core::node::{NodeMethod, NodeTag};
use octant_object::define_class;

use crate::{handle, Node, object};
use crate::runtime::HasTypedHandle;

#[derive(Debug)]
struct State {
    children: HashSet<ByAddress<Node>>,
}

define_class! {
    #[derive(Debug)]
    pub class extends object {
        state: AtomicRefCell<State>
    }
}

impl HasTypedHandle for Value {
    type TypeTag = NodeTag;
}

impl Value {
    fn invoke(&self, method: NodeMethod) -> handle::Value {
        (**self).invoke(Method::Node(self.typed_handle(), method))
    }

    pub fn new(handle: handle::Value) -> Self {
        Value {
            parent: object::Value::new(handle),
            state: AtomicRefCell::new(State {
                children: HashSet::new(),
            }),
        }
    }
    pub fn append_child(&self, child: Node) {
        self.invoke(NodeMethod::AppendChild(child.typed_handle()));
        self.state.borrow_mut().children.insert(ByAddress(child));
    }
    pub fn remove_child(&self, child: Node) {
        self.invoke(NodeMethod::RemoveChild(child.typed_handle()));
        self.state.borrow_mut().children.remove(&ByAddress(child));
    }

    pub fn set_node_value(&self, value: Option<String>) {
        self.invoke(NodeMethod::SetNodeValue(value));
    }
}
