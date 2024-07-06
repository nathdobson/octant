use marshal_pointer::raw_any::RawAny;
use std::fmt::{Debug, Formatter};

#[cfg(side = "server")]
pub trait EventHandler: 'static + RawAny + Fn() -> () {}

#[cfg(side = "server")]
impl<T: 'static + RawAny + Fn() -> ()> EventHandler for T {}

#[cfg(side = "server")]
impl Debug for dyn EventHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self as *const Self).raw_type_name())
    }
}
