use std::collections::HashSet;
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use by_address::ByAddress;

use octant_gui_core::{NodeMethod, NodeTag};
use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{HasLocalType, node, object, peer, Runtime};
use crate::object::{Object, ObjectValue};
use crate::peer::ArcPeer;

struct State {
    children: HashSet<ByAddress<ArcNode>>,
}

define_class! {
    pub class Node extends Object {
        node: web_sys::Node,
        state: AtomicRefCell<State>,
    }
}

impl NodeValue {
    pub fn new(handle: HandleId, node: web_sys::Node) -> Self {
        NodeValue {
            parent: ObjectValue::new(handle, node.clone().into()),
            node,
            state: AtomicRefCell::new(State {
                children: HashSet::new(),
            }),
        }
    }
    pub fn children(&self) -> Vec<ArcNode> {
        self.state
            .borrow_mut()
            .children
            .iter()
            .map(|x| x.0.clone())
            .collect()
    }
    pub fn native(&self) -> &web_sys::Node {
        &self.node
    }
}

impl dyn Node {
    pub fn invoke_with(
        &self,
        runtime: &Arc<Runtime>,
        method: &NodeMethod,
        _handle: HandleId,
    ) -> Option<ArcPeer> {
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
    type Local = dyn Node;
}
