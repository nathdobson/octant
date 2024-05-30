use std::cell::RefCell;
use std::rc::Rc;

use serde::Serialize;

use octant_reffed::rc::RcRef;
use octant_runtime::{define_sys_class, proto::UpMessage, runtime::Runtime};
use octant_runtime::peer::AsNative;
use octant_serde::{define_serde_impl, DeserializeWith};

use crate::{event_listener::RcEventListener, html_element::HtmlElement};

define_sys_class! {
    class HtmlInputElement;
    extends HtmlElement;
    wasm web_sys::HtmlInputElement;
    new_client _;
    new_server _;
    server_field value: RefCell<Rc<String>>;
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
        fn input_value(&self) -> Rc<String> {
            self.html_input_element().value.borrow_mut().clone()
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
        *self.element.html_input_element().value.borrow_mut() = Rc::new(self.value);
        Ok(())
    }
}
