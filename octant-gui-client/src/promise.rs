use std::{
    marker::PhantomData,
    sync::{Arc, OnceLock},
};

use js_sys::Promise;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};

use octant_gui_core::{
    Error, HandleId, PromiseMethod, PromiseTag, PromiseUpMessage, TypedHandle, UpMessage,
    UpMessageList,
};
use octant_object::define_class;

use crate::{any_value, object, peer, HasLocalType, Runtime};

define_class! {
    pub class extends object {
        promise: Promise,
        value: OnceLock<JsValue>,
    }
}

impl Value {
    pub fn new(handle: HandleId, promise: Promise) -> Self {
        Value {
            parent: object::Value::new(handle, promise.clone().into()),
            promise,
            value: OnceLock::new(),
        }
    }
    pub fn native(&self) -> &Promise {
        &self.promise
    }
    pub fn handle(&self) -> TypedHandle<PromiseTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Trait {
    pub fn invoke_with(
        self: &Arc<Self>,
        runtime: &Arc<Runtime>,
        method: &PromiseMethod,
        handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            PromiseMethod::Wait => {
                self.wait(runtime);
                None
            }
            PromiseMethod::Get => Some(Arc::new(any_value::Value::new(
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
    type Local = dyn Trait;
}
