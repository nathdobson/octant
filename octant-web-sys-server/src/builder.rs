use std::marker::Unsize;
use std::sync::Arc;
use octant_reffed::Reffed;

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

impl<T: ?Sized + Node> NodeExt for Arc<T> {
    fn child(self, child: ArcNode) -> Self {
        self.reffed().append_child(child);
        self
    }
}

impl<T: ?Sized + Unsize<dyn Element>> ElementExt for Arc<T> {
    fn attr(self, name: &str, value: &str) -> Self {
        (self.clone() as Arc<dyn Element>).set_attribute(name, value);
        self
    }
}

pub trait HtmlFormElementExt {
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self;
}

impl<T: ?Sized + HtmlFormElement> HtmlFormElementExt for Arc<T> {
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self {
        self.html_form_element().set_handler(handler);
        self
    }
}
