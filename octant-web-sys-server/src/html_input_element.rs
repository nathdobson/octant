use std::rc::Rc;
#[cfg(side = "server")]
use parking_lot::Mutex;
use std::sync::Arc;

use crate::{event_listener::RcEventListener, html_element::HtmlElement};
use octant_reffed::arc::ArcRef;
use octant_runtime::{define_sys_class, proto::UpMessage, runtime::Runtime};
use octant_serde::{define_serde_impl, DeserializeWith};
use serde::Serialize;
use octant_reffed::rc::RcRef;

define_sys_class! {
    class HtmlInputElement;
    extends HtmlElement;
    wasm web_sys::HtmlInputElement;
    new_client _;
    new_server _;
    server_field value: Mutex<Arc<String>>;
    client_fn{
        fn update_value(self:&RcRef<Self>){
            let this=self as &RcRef<dyn HtmlInputElement>;
            this.sink().send(Box::<SetInput>::new(SetInput{
                element: self.rc(),
                value: this.native().value()
            }));
        }
    }
    server_fn{
        fn input_value(&self) -> Arc<String> {
            self.html_input_element().value.lock().clone()
        }
    }
}

#[cfg(side = "server")]
impl HtmlInputElementValue {}

#[cfg(side = "client")]
impl HtmlInputElementValue {}

#[derive(Serialize, Debug, DeserializeWith)]
struct SetInput {
    element: RcHtmlInputElement,
    value: String,
}

define_serde_impl!(SetInput : UpMessage);
impl UpMessage for SetInput {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> anyhow::Result<()> {
        *self.element.html_input_element().value.lock() = Arc::new(self.value);
        Ok(())
    }
}
