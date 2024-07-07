use crate::{
    element::Element,
    event_handler::EventHandler,
    // form_submit_listener::{
    //     new_form_submit_listener, FormSubmitListener, FormSubmitListenerFields,
    //     RcFormSubmitListener,
    // },
    html_form_element::HtmlFormElement,
    node::{Node, RcNode},
};
use marshal_pointer::Rcf;

pub trait NodeExt {
    fn child(self, child: RcNode) -> Self;
}

pub trait ElementExt {
    fn attr(self, name: &str, value: &str) -> Self;
}

impl<T: ?Sized + Node> NodeExt for Rcf<T> {
    fn child(self, child: RcNode) -> Self {
        self.append_child(child);
        self
    }
}

impl<T: ?Sized + Element> ElementExt for Rcf<T> {
    fn attr(self, name: &str, value: &str) -> Self {
        self.set_attribute(name, value);
        self
    }
}

pub trait HtmlFormElementExt {
    fn form_submit_handler(self, listener: Box<dyn EventHandler<()>>) -> Self;
}

impl<T: ?Sized + HtmlFormElement> HtmlFormElementExt for Rcf<T> {
    fn form_submit_handler(self, handler: Box<dyn EventHandler<()>>) -> Self {
        self.set_form_submit_handler(handler);
        self
    }
}
