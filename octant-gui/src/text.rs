use octant_object::define_class;

use crate::{handle, node};
use crate::handle::HandleValue;
use crate::node::{Node, NodeValue};

define_class! {
    #[derive(Debug)]
    pub class Text extends Node{}
}

impl TextValue {
    pub fn new(handle: HandleValue) -> Self {
        TextValue {
            parent: NodeValue::new(handle),
        }
    }
}
