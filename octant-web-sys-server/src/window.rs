use futures::future::BoxFuture;
use std::{
    future::Future,
    mem::{ManuallyDrop, MaybeUninit},
    sync::Arc,
};

use crate::document::DocumentValue;
use octant_reffed::arc::{Arc2, ArcRef};
use octant_runtime::{
    define_sys_class, define_sys_rpc, error::OctantError, future_return::FutureReturn,
    octant_future::OctantFuture, runtime::Runtime,
};
use safe_once::sync::OnceLock;
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use wasm_bindgen_futures::JsFuture;
use wasm_error::WasmError;

use crate::{
    document::{ArcDocument, Document},
    navigator::{ArcNavigator, Navigator, NavigatorValue},
    object::Object,
    request::{ArcRequest, Request},
    response::{ArcResponse, ResponseValue},
};

define_sys_class! {
    class Window;
    extends Object;
    wasm web_sys::Window;
    new_client _;
    new_server _;
    server_field document : OnceLock<ArcDocument>;
    server_field navigator : OnceLock<ArcNavigator>;
    server_fn {
        fn document<'a>(self: &'a ArcRef<Self>) -> &'a ArcRef<dyn Document> {
            self.window().document.get_or_init(|| document(self.runtime(), self.arc()))
        }
        fn navigator<'a>(self: &'a ArcRef<Self>) -> &'a ArcRef< dyn Navigator> {
            self.window().navigator.get_or_init(|| navigator(self.runtime(),self.arc()))
        }
        fn alert(self: & ArcRef<Self>, message: String) {
            alert(self.runtime(), self.arc(), message);
        }
    }
}

#[cfg(side = "server")]
impl dyn Window {
    pub fn fetch<'a>(
        self: &'a ArcRef<Self>,
        request: ArcRequest,
    ) -> impl 'a + Send + Future<Output = anyhow::Result<ArcResponse>> {
        async move { Ok(fetch_wrap(self.runtime(), self.arc(), request).await??) }
    }
}

#[cfg(side = "server")]
fn fetch_wrap(
    runtime: &Arc<Runtime>,
    window: ArcWindow,
    request: ArcRequest,
) -> impl Send + Future<Output = Result<Result<ArcResponse, OctantError>, anyhow::Error>> {
    fetch(runtime, window, request)
}

define_sys_rpc! {
    fn alert(_runtime:_, window: Arc2<dyn Window>, message: String) -> () {
        window.native().alert_with_message(&message).unwrap();
        Ok(())
    }
    fn document(_runtime:_, window: Arc2<dyn Window>) -> ArcDocument {
        Ok(Arc2::new(DocumentValue::new(window.native().document().unwrap())))
    }
    fn navigator(_runtime:_,window:ArcWindow)->ArcNavigator{
        Ok(Arc2::new(NavigatorValue::new(window.native().navigator())))
    }
    fn fetch(runtime: _, window:ArcWindow, req:ArcRequest) -> OctantFuture<Result<ArcResponse, OctantError>>{
        let fetch = window.native().fetch_with_request(req.native());
        Ok(OctantFuture::spawn(runtime, async move{
            Ok(Arc2::new(ResponseValue::new(JsFuture::from(fetch).await.map_err(WasmError::new)?.dyn_into().map_err(WasmError::new)?)) as ArcResponse)
        }))
    }
}
