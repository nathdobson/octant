use octant_reffed::arc::ArcRef;
use octant_runtime::{define_sys_class, define_sys_rpc};
use std::sync::Arc;

use crate::node::Node;

define_sys_class! {
    class Element;
    extends Node;
    wasm web_sys::Element;
    new_client _;
    new_server _;
    server_fn {
        fn set_attribute(self: &ArcRef<Self>, key: &str, value: &str) {
            set_attribute(self.runtime(),self.arc(),key.to_string(),value.to_string())
        }
    }
}

define_sys_rpc! {
    fn set_attribute(_runtime:_, this: ArcElement, key:String, value:String) -> () {
        this.native().set_attribute(&key, &value).unwrap();
        Ok(())
    }
}
