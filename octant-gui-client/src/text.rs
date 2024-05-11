use octant_gui_core::HandleId;
use octant_object::define_class;

use crate::{node, node::Node};
use crate::node::NodeValue;

define_class! {
    pub class Text extends Node {
        text: web_sys::Text,
    }
}

impl TextValue {
    pub fn new(handle: HandleId, text: web_sys::Text) -> Self {
        TextValue {
            parent: NodeValue::new(handle, text.clone().into()),
            text,
        }
    }
}
