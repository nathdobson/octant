use base64urlsafedata::Base64UrlSafeData;
use std::{marker::PhantomData, sync::Arc};

use js_sys::{ArrayBuffer, Promise, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{console, PublicKeyCredential};

use octant_gui_core::{
    AuthenticationExtensionsClientOutputs, AuthenticatorAttestationResponse, AuthenticatorResponse,
    Credential, CredentialPromiseMethod, CredentialPromiseTag, CredentialPromiseUpMessage, Error,
    HandleId, PromiseMethod, PromiseTag, PromiseUpMessage, TypedHandle, UpMessage, UpMessageList,
};
use octant_object::define_class;

use crate::{import::Import, object, peer, promise, HasLocalType, Runtime};

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
                        CredentialPromiseUpMessage::Done(match result {
                            Ok(cred) => {
                                console::log_2(&JsValue::from_str("credential="), &cred);
                                Ok(cred.dyn_into::<web_sys::Credential>().unwrap().import())
                            }
                            Err(x) => Err(Error {
                                content: x.as_string().unwrap(),
                            }),
                        }),
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
