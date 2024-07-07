use marshal_pointer::raw_any::RawAny;
use std::fmt::{Debug, Formatter};
use octant_error::OctantResult;

#[cfg(side = "server")]
pub trait EventHandler<I>: 'static + RawAny + Fn(I) -> OctantResult<()> {}

#[cfg(side = "server")]
impl<I, T: 'static + RawAny + Fn(I) -> OctantResult<()>> EventHandler<I> for T {}

#[cfg(side = "server")]
impl<I: 'static> Debug for dyn EventHandler<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self as *const Self).raw_type_name())
    }
}
