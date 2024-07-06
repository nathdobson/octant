use marshal_pointer::Rcf;
use crate::{
    element::Element,
    event_listener::{EventListener, RcEventListener},
    html_form_element::HtmlFormElement,
    node::{Node, RcNode},
};

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
    fn handler(self, listener: RcEventListener) -> Self;
}

impl<T: ?Sized + HtmlFormElement> HtmlFormElementExt for Rcf<T> {
    fn handler(self, listener: RcEventListener) -> Self {
        self.set_listener(listener);
        self
    }
}
