use octant_runtime::define_sys_class;

use crate::object::Object;

define_sys_class! {
    class Credential;
    extends Object;
    wasm web_sys::Credential;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn Credential {}
