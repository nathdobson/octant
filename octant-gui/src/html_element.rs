use octant_object::define_class;

use crate::{
    element,
    element::{Element, ElementValue},
    handle,
    handle::HandleValue,
};

define_class! {
    #[derive(Debug)]
    pub class HtmlElement extends Element {}
}

impl HtmlElementValue {
    pub fn new(handle: HandleValue) -> Self {
        HtmlElementValue {
            parent: ElementValue::new(handle),
        }
    }
}
