use crate::event_listener::RcEventListener;
use octant_object::cast::downcast_object;
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
use octant_reffed::rc::{Rc2, RcRef};

use crate::{html_element::HtmlElement, html_input_element::RcHtmlInputElement};

define_sys_class! {
    class HtmlFormElement;
    extends HtmlElement;
    wasm web_sys::HtmlFormElement;
    new_client _;
    new_server _;
    client_field closure: OnceCell<Closure<dyn Fn(Event)>> ;
    server_field listener: OnceLock<RcEventListener>;
    server_fn {
        fn set_listener(self: &RcRef<Self>, listener: RcEventListener){
            self.html_form_element().listener.get_or_init(||listener.clone());
            set_listener(self.runtime(),self.rc(),listener);
        }
    }
}

define_sys_rpc! {
    fn set_listener(runtime:_, element: RcHtmlFormElement, listener:RcEventListener) -> () {
        let cb = Closure::<dyn Fn(Event)>::new({
            let listener=Rc2::downgrade(&listener);
            let element=Rc2::downgrade(&element);
            move |e:Event|{
                e.prevent_default();
                if let Some(element)=element.upgrade(){
                    for child in element.children(){
                        if let Ok(child)=downcast_object::<_,RcHtmlInputElement>(child){
                            child.update_value();
                        }
                    }
                }
                if let Some(listener) = listener.upgrade(){
                    listener.fire();
                }
            }
        });
        element.native().add_event_listener_with_callback("submit", cb.as_ref().unchecked_ref()).unwrap();
        element.html_form_element().closure.get_or_init(||cb);
        Ok(())
    }
}
