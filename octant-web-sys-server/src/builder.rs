use octant_reffed::arc::Arc2;
use std::{marker::Unsize, sync::Arc};

use crate::{
    element::Element,
    event_listener::{ArcEventListener, EventListener},
    html_form_element::HtmlFormElement,
    node::{ArcNode, Node},
};

pub trait NodeExt {
    fn child(self, child: ArcNode) -> Self;
}

pub trait ElementExt {
    fn attr(self, name: &str, value: &str) -> Self;
}

impl<T: ?Sized + Node> NodeExt for Arc2<T> {
    fn child(self, child: ArcNode) -> Self {
        self.append_child(child);
        self
    }
}

impl<T: ?Sized + Element> ElementExt for Arc2<T> {
    fn attr(self, name: &str, value: &str) -> Self {
        self.set_attribute(name, value);
        self
    }
}

pub trait HtmlFormElementExt {
    fn handler(self, listener: ArcEventListener) -> Self;
}

impl<T: ?Sized + HtmlFormElement> HtmlFormElementExt for Arc2<T> {
    fn handler(self, listener: ArcEventListener) -> Self {
        self.set_listener(listener);
        self
    }
}
