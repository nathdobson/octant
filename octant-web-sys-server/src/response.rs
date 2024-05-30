use octant_error::OctantError;
use octant_reffed::rc::RcRef;
use octant_runtime::{
    define_sys_class, define_sys_rpc, future_return::DataReturn, octant_future::OctantFuture,
};
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;
use octant_runtime::peer::AsNative;

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
    pub async fn text(self: &RcRef<Self>) -> anyhow::Result<String> {
        Ok(text(self.runtime(), self.rc()).await?.into_inner()?)
    }
}

define_sys_rpc! {
    fn text(runtime:_,response:RcResponse)->OctantFuture<DataReturn<Result<String, OctantError>>>{
        Ok(OctantFuture::spawn(runtime, async move{
            DataReturn::new(try{
                let text=JsFuture::from(response.native().text().map_err(OctantError::from)?).await.map_err(OctantError::from)?;
                text.as_string().unwrap()
            })
        }))
    }
}
