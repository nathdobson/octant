use octant_reffed::ArcRef;
use octant_runtime::{define_sys_class, define_sys_rpc};
use std::sync::Arc;

use crate::object::Object;

define_sys_class! {
    class Node;
    extends Object;
    wasm web_sys::Node;
    new_client _;
    new_server _;
    server_fn {
        fn append_child(self: ArcRef<Self>, e:ArcNode){
            append_child(self.runtime(), self.arc(), e);
        }
        fn remove_child(self: ArcRef<Self>, e:ArcNode){
            remove_child(self.runtime(), self.arc(), e);
        }
    }
}

define_sys_rpc! {
    fn append_child(_runtime:_, this: ArcNode, add:ArcNode) -> () {
        this.native().append_child(add.native()).unwrap();
        Ok(())
    }
}

define_sys_rpc! {
    fn remove_child(_runtime:_, this: ArcNode, add:ArcNode) -> () {
        this.native().remove_child(add.native()).unwrap();
        Ok(())
    }
}
