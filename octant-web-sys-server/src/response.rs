use crate::{
    js_string::{JsString, RcJsString},
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use marshal_pointer::RcfRef;
use octant_error::{octant_error, OctantError, OctantResult};
use octant_object::{class, DebugClass};
use octant_runtime::{
    future_return::DataReturn, octant_future::OctantFuture, rpc, runtime::Runtime, DeserializePeer,
    PeerNew, SerializePeer,
};
use safe_once::cell::OnceCell;
#[cfg(side = "server")]
use safe_once_async::async_lazy::AsyncLazy;
#[cfg(side = "server")]
use safe_once_async::cell::AsyncLazyCell;
#[cfg(side = "server")]
use safe_once_async::cell::AsyncOnceCell;
use std::rc::Rc;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ResponseFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::Response,
    #[cfg(side = "server")]
    local_text: AsyncOnceCell<OctantFuture<OctantResult<String>>>,
    #[cfg(side = "server")]
    remote_text: AsyncOnceCell<OctantFuture<OctantResult<RcJsString>>>,
}

#[class]
pub trait Response: Object {}

#[cfg(side = "server")]
impl dyn Response {
    pub async fn local_text<'a>(self: &'a RcfRef<Self>) -> OctantResult<&'a str> {
        Ok(self
            .local_text
            .get_or_init_detached(|| self.local_text_impl())
            .await
            .as_ref()?
            .as_ref()?)
    }
    pub async fn remote_text<'a>(self: &'a RcfRef<Self>) -> OctantResult<&'a RcfRef<dyn JsString>> {
        Ok(self
            .remote_text
            .get_or_init_detached(|| self.remote_text_impl())
            .await
            .as_ref()?
            .as_ref()?)
    }
}

#[rpc]
impl dyn Response {
    #[rpc]
    fn local_text_impl(
        self: &RcfRef<Self>,
        runtime: &Rc<Runtime>,
    ) -> OctantFuture<OctantResult<String>> {
        let this = self.strong();
        Ok(OctantFuture::spawn(runtime, async move {
            try {
                let text = JsFuture::from(this.native().text().map_err(OctantError::from)?)
                    .await
                    .map_err(OctantError::from)?;
                text.as_string()
                    .ok_or_else(|| octant_error!("response text is not a string"))?
            }
        }))
    }
    #[rpc]
    fn remote_text_impl(
        self: &RcfRef<Self>,
        runtime: &Rc<Runtime>,
    ) -> OctantFuture<OctantResult<RcJsString>> {
        let this = self.strong();
        Ok(OctantFuture::spawn(runtime, async move {
            try {
                let status = this.native().status();
                if status != 200 {
                    return Err(OctantError::msg(format!("HTTP status {}", status)));
                }
                let text = JsFuture::from(this.native().text().map_err(OctantError::from)?)
                    .await
                    .map_err(OctantError::from)?;
                RcJsString::peer_new(text.dyn_into()?)
            }
        }))
    }
}
