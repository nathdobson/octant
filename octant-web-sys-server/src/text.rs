use octant_gui_core::define_sys_class;

use crate::node::Node;

define_sys_class! {
    class Text;
    extends Node;
    wasm web_sys::Text;
}