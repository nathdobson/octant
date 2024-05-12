use octant_object::define_class;

use crate::{
    element::{Element, ElementValue}
    ,
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
