use octant_runtime::{define_sys_class, define_sys_rpc};
use std::sync::Arc;

use crate::object::Object;

define_sys_class! {
    class Node;
    extends Object;
    wasm web_sys::Node;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn Node {
    pub fn append_child(self: &Arc<Self>, e: ArcNode) {
        append_child(self.runtime(), self.clone(), e);
    }
    pub fn remove_child(self: &Arc<Self>, e: ArcNode) {
        todo!()
    }
}

define_sys_rpc! {
    fn append_child(_runtime, this: ArcNode, add:ArcNode) -> () {
        this.native().append_child(add.native()).unwrap();
        Ok(())
    }
}
