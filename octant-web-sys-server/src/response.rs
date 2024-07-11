use crate::{
    js_string::JsString,
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use marshal_pointer::RcfRef;
use octant_error::{OctantError, OctantResult};
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
use wasm_bindgen_futures::JsFuture;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct ResponseFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    any_value: web_sys::Response,
    #[cfg(side = "server")]
    local_text: AsyncOnceCell<OctantFuture<DataReturn<OctantResult<String>>>>,
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
            .inner()
            .as_ref()?)
    }
}

#[rpc]
impl dyn Response {
    #[rpc]
    fn local_text_impl(
        self: &RcfRef<Self>,
        runtime: &Rc<Runtime>,
    ) -> OctantFuture<DataReturn<OctantResult<String>>> {
        let this = self.strong();
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
