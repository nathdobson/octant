use octant_error::{OctantError, OctantResult};
use octant_object::{class, DebugClass};
use octant_reffed::rc::RcRef;
use octant_runtime::{
    future_return::DataReturn, octant_future::OctantFuture, peer::AsNative, rpc, runtime::Runtime,
    DeserializePeer, PeerNew, SerializePeer,
};
use std::rc::Rc;
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

use crate::object::{Object, ObjectFields};

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ResponseFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::Response,
}

#[class]
pub trait Response: Object {}

#[cfg(side = "server")]
impl dyn Response {
    pub async fn text(self: &RcRef<Self>) -> OctantResult<String> {
        Ok(self.text_impl().await?.into_inner()?)
    }
}

#[rpc]
impl dyn Response {
    #[rpc]
    fn text_impl(
        self: &RcRef<Self>,
        runtime: &Rc<Runtime>,
    ) -> OctantFuture<DataReturn<Result<String, OctantError>>> {
        let this = self.rc();
        Ok(OctantFuture::spawn(runtime, async move {
            DataReturn::new(
                try {
                    let text = JsFuture::from(this.native().text().map_err(OctantError::from)?)
                        .await
                        .map_err(OctantError::from)?;
                    text.as_string().unwrap()
                },
            )
        }))
    }
}
