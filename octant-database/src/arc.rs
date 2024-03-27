use std::borrow::Cow;
use std::sync::{Arc, Weak};

//
// pub struct UniqueWeak<T>(Weak<T>);
//
// impl<T> UniqueWeak<T> {
//     pub fn new() -> UniqueWeak<T> {
//         unsafe {
//             let arc = Arc::new_uninit();
//             let weak: Weak<MaybeUninit<T>> = Arc::downgrade(&arc);
//             mem::drop(arc);
//             let weak_cast = mem::transmute::<Weak<MaybeUninit<T>>, Weak<T>>(weak);
//             UniqueWeak(weak_cast)
//         }
//     }
//     pub fn as_weak(&self) -> Weak<T> {
//         self.0.clone()
//     }
//     pub fn init(self, value: T) -> Arc<T> {
//         unsafe {
//             let weak = ManuallyDrop::new(self.0);
//             let mut ptr = Weak::as_ptr(&weak) as *mut T;
//             ptr.write(value);
//             Arc::increment_strong_count(ptr);
//             let result = Arc::from_raw(ptr);
//             result
//         }
//     }
// }
// #[test]
// fn test() {
//     let mut x = UniqueWeak::new();
//     let x = x.init(12);
//     assert_eq!(*x, 12);
// }

#[derive(Debug)]
pub enum ArcOrWeak<T: ?Sized> {
    Arc(Arc<T>),
    Weak(Weak<T>),
}

impl<T: ?Sized> ArcOrWeak<T> {
    pub fn upgrade_cow<'a>(&'a self) -> Option<Cow<'a, Arc<T>>> {
        match self {
            ArcOrWeak::Arc(x) => Some(Cow::Borrowed(x)),
            ArcOrWeak::Weak(x) => x.upgrade().map(Cow::Owned),
        }
    }
}

