use by_address::ByAddress;
use octant_reffed::arc::ArcRef;
use octant_runtime::{define_sys_class, define_sys_rpc};
#[cfg(side = "server")]
use parking_lot::Mutex;
use std::{cell::RefCell, collections::HashSet, sync::Arc};

use crate::object::Object;

define_sys_class! {
    class Node;
    extends Object;
    wasm web_sys::Node;
    new_client _;
    new_server _;
    client_field children: RefCell<HashSet<ByAddress<ArcNode>>>;
    server_field children: Mutex<HashSet<ByAddress<ArcNode>>>;
    client_fn{
        fn children(self:&ArcRef<Self>)->Vec<ArcNode>{
            self.node().children.borrow().iter().map(|x|x.0.clone()).collect()
        }
    }
    server_fn {
        fn append_child(self: &ArcRef<Self>, e:ArcNode){
            self.node().children.lock().insert(ByAddress(e.clone()));
            append_child(self.runtime(), self.arc(), e);
        }
        fn remove_child(self: &ArcRef<Self>, e:ArcNode){
            self.node().children.lock().remove(&ByAddress(e.clone()));
            remove_child(self.runtime(), self.arc(), e);
        }
    }
}

define_sys_rpc! {
    fn append_child(_runtime:_, this: ArcNode, add:ArcNode) -> () {
        this.node().children.borrow_mut().insert(ByAddress(add.clone()));
        this.native().append_child(add.native()).unwrap();
        Ok(())
    }
}

define_sys_rpc! {
    fn remove_child(_runtime:_, this: ArcNode, add:ArcNode) -> () {
        this.node().children.borrow_mut().remove(&ByAddress(add.clone()));
        this.native().remove_child(add.native()).unwrap();
        Ok(())
    }
}
