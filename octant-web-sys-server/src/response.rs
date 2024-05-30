use octant_error::OctantError;
use octant_object::{class, DebugClass};
use octant_reffed::rc::RcRef;
use octant_runtime::{
    define_sys_rpc, future_return::DataReturn, octant_future::OctantFuture, peer::AsNative,
    DeserializePeer, PeerNew, SerializePeer,
};
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

use crate::object::{Object, ObjectValue};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ResponseValue {
    parent: ObjectValue,
    #[cfg(side = "client")]
    any_value: web_sys::Response,
}

#[class]
pub trait Response: Object {}

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
