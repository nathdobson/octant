use crate::event_listener::ArcEventListener;
use octant_reffed::arc::{Arc2, ArcRef};
use octant_runtime::{define_sys_class, define_sys_rpc};
#[cfg(side = "server")]
use parking_lot::Mutex;
use safe_once::{
    api::once::OnceEntry,
    cell::OnceCell,
    sync::{OnceLock, RawFusedLock},
};
use serde::Serialize;
use std::{
    any::{type_name, Any},
    fmt::{Debug, Formatter},
    sync::Arc,
};
#[cfg(side = "client")]
use wasm_bindgen::closure::Closure;
#[cfg(side = "client")]
use wasm_bindgen::JsCast;
#[cfg(side = "client")]
use web_sys::Event;

use crate::html_element::HtmlElement;

define_sys_class! {
    class HtmlFormElement;
    extends HtmlElement;
    wasm web_sys::HtmlFormElement;
    new_client _;
    new_server _;
    client_field closure: OnceCell<Closure<dyn Fn()>> ;
    server_field listener: OnceLock<ArcEventListener>;
    server_fn {
        fn set_listener(self: &ArcRef<Self>, listener: ArcEventListener){
            self.html_form_element().listener.get_or_init(||listener.clone());
            set_listener(self.runtime(),self.arc(),listener);
        }
    }
}

define_sys_rpc! {
    pub fn set_listener(runtime:_, element: ArcHtmlFormElement, listener:ArcEventListener) -> () {
        let runtime=Arc::downgrade(runtime);
        let listener=Arc2::downgrade(&listener);
        let cb = Closure::<dyn Fn(Event)>::new(move |e:Event|{
            if let (Some(runtime),Some(listener)) = (runtime.upgrade(), listener.upgrade()){
                listener.fire(&runtime);
            }
        });
        element.native().add_event_listener_with_callback("submit", cb.as_ref().unchecked_ref()).unwrap();
        Ok(())
    }
}
