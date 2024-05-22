use std::{
    mem::{ManuallyDrop, MaybeUninit},
    sync::Arc,
};

use safe_once::sync::OnceLock;
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize, Serializer};
use octant_runtime::{define_sys_class, define_sys_rpc};
use crate::document::DocumentValue;

use crate::{
    document::{ArcDocument, Document},
    navigator::ArcNavigator,
    object::Object,
    request::ArcRequest,
    response::ArcResponse,
};

define_sys_class! {
    class Window;
    extends Object;
    wasm web_sys::Window;
    new_client _;
    new_server _;
    server_field document : OnceLock<ArcDocument>;
}

#[cfg(side = "server")]
impl dyn Window {
    pub fn alert(self: &Arc<Self>, message: String) {
        alert(self.runtime(), self.clone(), message);
    }
    pub fn document<'a>(self: &'a Arc<Self>) -> &'a ArcDocument {
        self.document
            .get_or_init(|| document(self.runtime(), self.clone()))
    }
    pub fn navigator<'a>(self: &'a Arc<Self>) -> &'a ArcNavigator {
        todo!();
    }
    pub async fn fetch(self: &Arc<Self>, request: &ArcRequest) -> anyhow::Result<ArcResponse> {
        todo!();
    }
}

define_sys_rpc! {
    fn alert(_runtime, window: Arc<dyn Window>, message: String) -> () {
        window.native().alert_with_message(&message).unwrap();
        Ok(())
    }
}

define_sys_rpc! {
    fn document(_runtime, window: Arc<dyn Window>) -> (Document, ) {
        Ok((Arc::new(DocumentValue::new(window.native().document().unwrap())),))
    }
}
