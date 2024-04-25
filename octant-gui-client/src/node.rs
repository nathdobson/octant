use std::collections::HashSet;
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use by_address::ByAddress;
use web_sys::Node;

use octant_gui_core::{NodeMethod, NodeTag};
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{HasLocalType, node, object, peer, Runtime};

struct State {
    children: HashSet<ByAddress<Arc<dyn Trait>>>,
}

define_class! {
    pub class extends object {
        node: Node,
        state: AtomicRefCell<State>,
    }
}

impl Value {
    pub fn new(handle: HandleId, node: Node) -> Self {
        Value {
            parent: object::Value::new(handle, node.clone().into()),
            node,
            state: AtomicRefCell::new(State {
                children: HashSet::new(),
            }),
        }
    }
    pub fn children(&self) -> Vec<Arc<dyn node::Trait>> {
        self.state
            .borrow_mut()
            .children
            .iter()
            .map(|x| x.0.clone())
            .collect()
    }
    pub fn native(&self) -> &Node {
        &self.node
    }
}

impl dyn Trait {
    pub fn invoke_with(
        &self,
        runtime: &Arc<Runtime>,
        method: &NodeMethod,
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            NodeMethod::AppendChild(node) => {
                let node = runtime.handle(*node);
                self.state
                    .borrow_mut()
                    .children
                    .insert(ByAddress(node.clone()));
                self.native().append_child(node.native()).unwrap();
                None
            }
            NodeMethod::RemoveChild(node) => {
                let node = runtime.handle(*node);
                self.state
                    .borrow_mut()
                    .children
                    .remove(&ByAddress(node.clone()));
                self.native().remove_child(node.native()).unwrap();
                None
            }
            NodeMethod::SetNodeValue(value) => {
                self.native().set_node_value(value.as_deref());
                None
            }
        }
    }
}

impl HasLocalType for NodeTag {
    type Local = dyn Trait;
}
