use octant_gui_core::define_sys_class;
use crate::object::Object;
define_sys_class!{
    class Window;
    extends Object;
    wasm web_sys::Window;
    new_client a;
    new_server a;
}