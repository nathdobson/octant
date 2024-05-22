use octant_runtime::define_sys_class;

use crate::object::Object;

define_sys_class! {
    class Node;
    extends Object;
    wasm web_sys::Node;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl NodeValue {
    pub fn append_child(&self, e: ArcNode) {
        todo!();
    }
    pub fn remove_child(&self, e: ArcNode) {
        todo!()
    }
    pub fn set_attribute(&self, a: &str, b: &str) {
        todo!();
    }
}
