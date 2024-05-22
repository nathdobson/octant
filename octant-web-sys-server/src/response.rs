use octant_runtime::define_sys_class;
use crate::object::Object;

define_sys_class! {
    class Response;
    extends Object;
    wasm web_sys::Response;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn Response {
    pub async fn text(&self) -> anyhow::Result<String> {
        todo!();
    }
}
