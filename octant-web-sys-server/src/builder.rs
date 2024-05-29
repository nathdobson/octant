use octant_reffed::arc::Arc2;
use std::{marker::Unsize, sync::Arc};
use octant_reffed::rc::Rc2;

use crate::{
    element::Element,
    event_listener::{RcEventListener, EventListener},
    html_form_element::HtmlFormElement,
    node::{RcNode, Node},
};

pub trait NodeExt {
    fn child(self, child: RcNode) -> Self;
}

pub trait ElementExt {
    fn attr(self, name: &str, value: &str) -> Self;
}

impl<T: ?Sized + Node> NodeExt for Rc2<T> {
    fn child(self, child: RcNode) -> Self {
        self.append_child(child);
        self
    }
}

impl<T: ?Sized + Element> ElementExt for Rc2<T> {
    fn attr(self, name: &str, value: &str) -> Self {
        self.set_attribute(name, value);
        self
    }
}

pub trait HtmlFormElementExt {
    fn handler(self, listener: RcEventListener) -> Self;
}

impl<T: ?Sized + HtmlFormElement> HtmlFormElementExt for Rc2<T> {
    fn handler(self, listener: RcEventListener) -> Self {
        self.set_listener(listener);
        self
    }
}
