use octant_gui_core::define_sys_class;

use crate::object::Object;

define_sys_class!{
    class RequestInit;
    extends Object;
    wasm web_sys::RequestInit;
    new_client _;
    new_server _;
}