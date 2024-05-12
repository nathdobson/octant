use std::sync::Arc;

use crate::element::Element;
use crate::html_form_element::HtmlFormElement;
use crate::node::ArcNode;

pub trait ElementExt {
    fn child(self, child: ArcNode) -> Self;
    fn attr(self, name: &str, value: &str) -> Self;
}

impl<T: ?Sized + Element> ElementExt for Arc<T> {
    fn child(self, child: ArcNode) -> Self {
        Element::value(&*self).append_child(child);
        self
    }
    fn attr(self, name: &str, value: &str) -> Self {
        Element::value(&*self).set_attribute(name, value);
        self
    }
}

pub trait HtmlFormElementExt {
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self;
}

impl<T: ?Sized + HtmlFormElement> HtmlFormElementExt for Arc<T> {
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self {
        HtmlFormElement::value(&*self).set_handler(handler);
        self
    }
}
