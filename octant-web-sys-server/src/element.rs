use octant_runtime::define_sys_class;
use std::sync::Arc;
use octant_reffed::ArcRef;

use crate::node::Node;

define_sys_class! {
    class Element;
    extends Node;
    wasm web_sys::Element;
    new_client _;
    new_server _;
    server_fn {
        fn set_attribute(self: &ArcRef<Self>, a: &str, b: &str) {
            todo!();
        }
    }
}
