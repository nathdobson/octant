use octant_runtime::define_sys_class;
use std::sync::Arc;

use crate::node::Node;

define_sys_class! {
    class Element;
    extends Node;
    wasm web_sys::Element;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn Element {
    pub fn set_attribute(self: &Arc<Self>, a: &str, b: &str) {
        todo!();
    }
}
