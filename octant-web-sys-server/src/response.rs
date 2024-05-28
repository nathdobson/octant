use crate::object::Object;
use octant_reffed::arc::ArcRef;
use octant_runtime::{
    define_sys_class, define_sys_rpc, error::OctantError, future_return::DataReturn,
    octant_future::OctantFuture,
};
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;
use wasm_error::WasmError;

define_sys_class! {
    class Response;
    extends Object;
    wasm web_sys::Response;
    new_client _;
    new_server _;
}

#[cfg(side = "server")]
impl dyn Response {
    pub async fn text(self: &ArcRef<Self>) -> anyhow::Result<String> {
        Ok(text(self.runtime(), self.arc()).await?.into_inner()?)
    }
}

define_sys_rpc! {
    fn text(runtime:_,response:ArcResponse)->OctantFuture<DataReturn<Result<String, OctantError>>>{
        Ok(OctantFuture::spawn(runtime, async move{
            DataReturn::new(try{
                let text=JsFuture::from(response.native().text().map_err(WasmError::new)?).await.map_err(WasmError::new)?;
                text.as_string().unwrap()
            })
        }))
    }
}
