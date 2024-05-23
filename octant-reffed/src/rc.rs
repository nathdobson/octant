use crate::Reffed;
use std::{
    marker::{PhantomData, Unsize},
    ops::{CoerceUnsized, Deref, DispatchFromDyn},
    rc::Rc,
};

pub struct RcRef<'a, T: ?Sized>(&'a T, PhantomData<*const ()>);

impl<'a, T> RcRef<'a, T> {
    pub fn rc(&self) -> Rc<T> {
        unsafe {
            Rc::increment_strong_count(self.0);
            Rc::from_raw(self.0)
        }
    }
}

impl<'a, T: ?Sized> Reffed for &'a Rc<T> {
    type ReffedTarget = RcRef<'a, T>;
    fn reffed(self) -> Self::ReffedTarget {
        RcRef(&*self, PhantomData)
    }
}

impl<'a, T: ?Sized> Deref for RcRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, T: ?Sized, U: ?Sized> CoerceUnsized<RcRef<'a, U>> for RcRef<'b, T>
where
    'b: 'a,
    T: Unsize<U>,
{
}

impl<'a, T: ?Sized, U: ?Sized> DispatchFromDyn<RcRef<'a, U>> for RcRef<'a, T> where T: Unsize<U> {}
