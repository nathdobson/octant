use crate::{
    html_element::RcHtmlElement,
    location::RcLocation,
    object::{Object, ObjectFields},
    octant_runtime::peer::AsNative,
};
use marshal_pointer::RcfRef;
use octant_object::{class, DebugClass};
use octant_runtime::{rpc, runtime::Runtime, DeserializePeer, PeerNew, SerializePeer};
use safe_once::cell::OnceCell;
use std::rc::Rc;

#[derive(DebugClass, PeerNew, SerializePeer, DeserializePeer)]
pub struct CssStyleDeclarationFields {
    parent: ObjectFields,
    #[cfg(side = "client")]
    document: web_sys::CssStyleDeclaration,
}

#[class]
pub trait CssStyleDeclaration: Object {
    #[cfg(side = "server")]
    fn set_property(self: &RcfRef<Self>, name: &str, value: &str) {
        self.set_property_impl(name.to_owned(), value.to_owned());
    }
}

#[rpc]
impl dyn CssStyleDeclaration {
    #[rpc]
    fn set_property_impl(self: &RcfRef<Self>, runtime: &Rc<Runtime>, name: String, value: String) {
        self.native().set_property(&name, &value)?;
        Ok(())
    }
}
