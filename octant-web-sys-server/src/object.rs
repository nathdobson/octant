use octant_gui_core::define_sys_class;

use crate::js_value::JsValue;

define_sys_class! {
    class Object;
    extends JsValue;
    wasm js_sys::Object;
    new_client a;
    new_server a;
}
