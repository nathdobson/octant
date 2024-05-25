use futures::future::BoxFuture;
use std::{
    mem::{ManuallyDrop, MaybeUninit},
    sync::Arc,
};

use crate::document::DocumentValue;
use octant_reffed::arc::{Arc2, ArcRef};
use octant_runtime::{define_sys_class, define_sys_rpc};
use safe_once::sync::OnceLock;
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    document::{ArcDocument, Document},
    navigator::{ArcNavigator, Navigator},
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
    server_fn {
        fn document<'a>(self: &'a ArcRef<Self>) -> &'a ArcRef<dyn Document> {
            self.window().document.get_or_init(|| document(self.runtime(), self.arc()))
        }
        fn fetch(self: &ArcRef<Self>, request: &ArcRequest) -> BoxFuture<anyhow::Result<ArcResponse>> {
            todo!();
        }
        fn navigator<'a>(self: &'a ArcRef<Self>) -> &'a ArcRef< dyn Navigator> {
            todo!();
        }
        fn alert(self: & ArcRef<Self>, message: String) {
            alert(self.runtime(), self.arc(), message);
        }
    }
}

define_sys_rpc! {
    fn alert(_runtime:_, window: Arc2<dyn Window>, message: String) -> () {
        window.native().alert_with_message(&message).unwrap();
        Ok(())
    }
}

define_sys_rpc! {
    fn document(_runtime:_, window: Arc2<dyn Window>) -> ArcDocument {
        Ok(Arc2::new(DocumentValue::new(window.native().document().unwrap())))
    }
}
