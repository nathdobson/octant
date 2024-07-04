use std::rc::Rc;
use marshal_pointer::rc_ref::RcRef;

use octant_error::{OctantError, OctantResult};
use octant_object::{class, DebugClass};
use octant_runtime::{
    DeserializePeer, future_return::DataReturn, octant_future::OctantFuture, PeerNew,
    rpc, SerializePeer,
};
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;
use octant_runtime::runtime::Runtime;
use crate::object::{Object, ObjectFields};
use crate::octant_runtime::peer::AsNative;

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
