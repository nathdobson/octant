use crate::{element, html_form_element, HtmlFormElement, Node};
use std::sync::Arc;

pub trait ElementExt {
    fn child(self, child: Node) -> Self;
    fn attr(self, name: &str, value: &str) -> Self;
}

impl<T: ?Sized + element::Trait> ElementExt for Arc<T> {
    fn child(self, child: Node) -> Self {
        element::Trait::value(&*self).append_child(child);
        self
    }
    fn attr(self, name: &str, value: &str) -> Self {
        element::Trait::value(&*self).set_attribute(name, value);
        self
    }
}

pub trait HtmlFormElementExt {
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self;
}
impl<T: ?Sized + html_form_element::Trait> HtmlFormElementExt for Arc<T> {
    fn handler(self, handler: impl 'static + Send + Sync + Fn()) -> Self {
        html_form_element::Trait::value(&*self).set_handler(handler);
        self
    }
}
