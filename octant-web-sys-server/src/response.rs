use octant_error::OctantError;
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
    pub async fn text(self: &RcRef<Self>) -> anyhow::Result<String> {
        Ok(text(self.runtime(), self.rc()).await?.into_inner()?)
    }
}

#[rpc]
fn text(
    runtime: &Rc<Runtime>,
    response: RcResponse,
) -> OctantFuture<DataReturn<Result<String, OctantError>>> {
    Ok(OctantFuture::spawn(runtime, async move {
        DataReturn::new(
            try {
                let text = JsFuture::from(response.native().text().map_err(OctantError::from)?)
                    .await
                    .map_err(OctantError::from)?;
                text.as_string().unwrap()
            },
        )
    }))
}
