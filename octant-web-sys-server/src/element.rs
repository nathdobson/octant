use octant_gui_core::define_sys_class;

use crate::node::Node;

define_sys_class! {
    class Element;
    extends Node;
    wasm web_sys::Element;
    new_client _;
    new_server _;
}
