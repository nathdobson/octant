use std::{marker::PhantomData, sync::Arc};

use js_sys::Promise;
use octant_gui_core::{
    CredentialPromiseMethod, CredentialPromiseTag, CredentialPromiseUpMessage, HandleId,
    TypedHandle, UpMessage, UpMessageList,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

use octant_object::define_class;

use crate::{import::Import, peer, promise, HasLocalType, Runtime};

define_class! {
    pub class extends promise {
    }
}

impl Value {
    pub fn new(handle: HandleId, promise: Promise) -> Self {
        Value {
            parent: promise::Value::new(handle, promise),
        }
    }
    pub fn handle(&self) -> TypedHandle<CredentialPromiseTag> {
        TypedHandle(self.raw_handle(), PhantomData)
    }
}

impl dyn Trait {
    pub fn invoke_with(
        self: &Arc<Self>,
        runtime: &Arc<Runtime>,
        method: &CredentialPromiseMethod,
        _handle: HandleId,
    ) -> Option<Arc<dyn peer::Trait>> {
        match method {
            CredentialPromiseMethod::Wait => {
                self.wait(runtime);
                None
            }
        }
    }
    fn wait(self: &Arc<Self>, runtime: &Arc<Runtime>) {
        log::info!("waiting");
        spawn_local({
            let this = self.clone();
            let runtime = runtime.clone();
            async move {
                let result = JsFuture::from(this.native().clone()).await;
                log::info!("Sending response");
                if let Err(err) = runtime.send(UpMessageList {
                    commands: vec![UpMessage::CredentialPromise(
                        this.handle(),
                        CredentialPromiseUpMessage::Done(
                            result
                                .map(|x| x.dyn_into::<web_sys::Credential>().unwrap())
                                .import(),
                        ),
                    )],
                }) {
                    log::error!("Cannot send event {:?}", err);
                };
            }
        });
    }
}

impl HasLocalType for CredentialPromiseTag {
    type Local = dyn Trait;
}
