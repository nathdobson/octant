use crate::object::Object;
use octant_gui_core::define_sys_class;
define_sys_class! {
    class Node;
    extends Object;
    wasm web_sys::Node;
    new_client a;
    new_server a;
}
