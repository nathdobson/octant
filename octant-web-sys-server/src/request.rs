use octant_gui_core::define_sys_class;

use crate::object::Object;

define_sys_class! {
    class Request;
    extends Object;
    wasm web_sys::Request;
    new_client _;
    new_server _;
}
