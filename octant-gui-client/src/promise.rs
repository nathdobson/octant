use std::{
    marker::PhantomData,
    sync::{Arc, OnceLock},
};

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{JsFuture, spawn_local};

use octant_gui_core::{
    Error, HandleId, PromiseMethod, PromiseTag, PromiseUpMessage, TypedHandle, UpMessage,
    UpMessageList,
};
use octant_object::define_class;

use crate::{
    any_value::AnyValueValue,
    HasLocalType,
    object::{Object, ObjectValue},
    peer::ArcPeer, Runtime,
};

define_class! {
    pub class Promise extends Object {
        promise: js_sys::Promise,
        value: OnceLock<JsValue>,
    }
}

impl PromiseValue {
    pub fn new(handle: HandleId, promise: js_sys::Promise) -> Self {
        PromiseValue {
            parent: ObjectValue::new(handle, promise.clone().into()),
            promise,
            value: OnceLock::new(),
        }
    }
    pub fn native(&self) -> &js_sys::Promise {
        &self.promise
    }
    pub fn handle(&self) -> TypedHandle<PromiseTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Promise {
    pub fn invoke_with(
        self: &Arc<Self>,
        runtime: &Arc<Runtime>,
        method: &PromiseMethod,
        handle: HandleId,
    ) -> Option<ArcPeer> {
        match method {
            PromiseMethod::Wait => {
                self.wait(runtime);
                None
            }
            PromiseMethod::Get => Some(Arc::new(AnyValueValue::new(
                handle,
                self.value.get().unwrap().clone(),
            ))),
        }
    }
    fn wait(self: &Arc<Self>, runtime: &Arc<Runtime>) {
        log::info!("waiting");
        spawn_local({
            let this = self.clone();
            let runtime = runtime.clone();
            async move {
                let result = JsFuture::from(this.native().clone()).await;
                let result = match result {
                    Ok(x) => {
                        this.value.set(x).unwrap();
                        Ok(())
                    }
                    Err(e) => Err(Error {
                        content: (format!("{:?}", e)),
                    }),
                };
                log::info!("Sending response");
                if let Err(err) = runtime.send(UpMessageList {
                    commands: vec![UpMessage::Promise(
                        this.handle(),
                        PromiseUpMessage::Done(result),
                    )],
                }) {
                    log::error!("Cannot send event {:?}", err);
                };
            }
        });
    }
}

impl HasLocalType for PromiseTag {
    type Local = dyn Promise;
}
