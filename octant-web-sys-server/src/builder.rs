use std::marker::Unsize;
use std::sync::Arc;
use octant_reffed::Arc2;

use crate::{
    element::Element,
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
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self;
}

impl<T: ?Sized + HtmlFormElement> HtmlFormElementExt for Arc2<T> {
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self {
        self.html_form_element().set_handler(handler);
        self
    }
}
