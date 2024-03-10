use std::fmt::{Debug, Display, Formatter};
use std::marker::Unsize;
use std::ops::{CoerceUnsized, Deref, DerefMut};

#[repr(transparent)]
pub struct Tack<'a, T: ?Sized>(&'a mut T);

pub auto trait Untack {}

impl<'a, T: ?Sized + Untack> Tack<'a, T> {
    // pub fn new(p: &'a mut T) -> Self {
    //     Tack(p)
    // }
    pub fn into_inner(self) -> &'a mut T {
        self.0
    }
}

impl<'a, T: ?Sized> Tack<'a, T> {
    pub fn new(p: &'a mut T) -> Self {
        Tack(p)
    }
    pub fn into_inner_unchecked(self) -> &'a mut T {
        self.0
    }
}

impl<'a, T: ?Sized> Tack<'a, T> {
    pub fn get_ref(self) -> &'a T {
        self.0
    }
    pub fn as_mut(&mut self) -> Tack<T> {
        Tack::new(&mut *self.0)
    }
    pub fn as_ref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T: ?Sized + Untack> Tack<'a, T> {
    pub fn get_mut(self) -> &'a mut T {
        self.0
    }
}

impl<'a, T: ?Sized, U: ?Sized> CoerceUnsized<Tack<'a, U>> for Tack<'a, T> where T: Unsize<U> {}

impl<'a, T: ?Sized + Debug> Debug for Tack<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<'a, T: ?Sized + Display> Display for Tack<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, T: ?Sized> Deref for Tack<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: ?Sized + Untack> DerefMut for Tack<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut().get_mut()
    }
}
