use marshal_pointer::rc_ref::RcRef;
use marshal_pointer::rcf::{Rcf, RcfWeak};

// use std::{
//     fmt::{Debug, Formatter},
//     hash::Hash,
//     marker::Unsize,
//     ops::{CoerceUnsized, Deref, DispatchFromDyn},
//     rc::{Rc, Weak},
// };
//
// use marshal::{
//     context::Context,
//     de::{rc::DeserializeRc, Deserialize},
//     decode::{AnyDecoder, Decoder},
//     encode::{AnyEncoder, Encoder},
//     reexports::marshal_pointer::{rc_ref::RcRef, AsFlatRef},
//     ser::{rc::SerializeRc, Serialize},
// };
// use weak_table::traits::{WeakElement, WeakKey};
//
pub type Rc2<T> = Rcf<T>;
pub type Weak2<T> = RcfWeak<T>;
pub type Rc2Ref<T> = RcRef<T>;
//
// pub struct Rc2<T: ?Sized> {
//     rc: Rc<T>,
// }
//
// pub struct Weak2<T: ?Sized> {
//     weak: Weak<T>,
// }
//
// impl<T: ?Sized> Deref for Rc2<T> {
//     type Target = RcRef<T>;
//     fn deref(&self) -> &Self::Target {
//         self.rc.as_flat_ref()
//     }
// }
//
// impl<T: ?Sized> Rc2<T> {
//     pub fn new(x: T) -> Self
//     where
//         T: Sized,
//     {
//         Rc2 { rc: Rc::new(x) }
//     }
//     pub unsafe fn from_raw(ptr: *const T) -> Self {
//         Rc2 {
//             rc: Rc::from_raw(ptr),
//         }
//     }
//     pub fn into_raw(this: Self) -> *const T {
//         Rc::into_raw(this.rc) as *const T
//     }
//     pub fn downgrade(&self) -> Weak2<T> {
//         Weak2 {
//             weak: Rc::downgrade(&self.rc),
//         }
//     }
//     pub fn into_rc(self) -> Rc<T> {
//         self.rc
//     }
// }
//
// impl<T: ?Sized> Weak2<T> {
//     pub fn upgrade(&self) -> Option<Rc2<T>> {
//         Some(Rc2 {
//             rc: self.weak.upgrade()?,
//         })
//     }
// }
//
// pub trait Rc2Ref {
//     type Inner: ?Sized;
//     fn rc2(&self) -> Rc2<Self::Inner>;
// }
//
// impl<T: ?Sized> Rc2Ref for RcRef<T> {
//     type Inner = T;
//     fn rc2(&self) -> Rc2<Self::Inner> {
//         Rc2 { rc: self.rc() }
//     }
// }
//
// impl<T: ?Sized> From<Rc<T>> for Rc2<T> {
//     fn from(rc: Rc<T>) -> Self {
//         Rc2 { rc }
//     }
// }
//
// impl<E: Encoder, T: ?Sized> Serialize<E> for Rc2<T>
// where
//     T: SerializeRc<E>,
// {
//     fn serialize<'w, 'en>(&self, e: AnyEncoder<'w, 'en, E>, ctx: Context) -> anyhow::Result<()> {
//         <T as SerializeRc<E>>::serialize_rc(self, e, ctx)
//     }
// }
//
// impl<D: Decoder, T: ?Sized> Deserialize<D> for Rc2<T>
// where
//     T: DeserializeRc<D>,
// {
//     fn deserialize<'p, 'de>(d: AnyDecoder<'p, 'de, D>, ctx: Context) -> anyhow::Result<Self> {
//         Ok(Rc2::from(<T as DeserializeRc<D>>::deserialize_rc(d, ctx)?))
//     }
// }
//
// impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Rc2<U>> for Rc2<T> {}
//
// impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Rc2<U>> for Rc2<T> {}
//
// impl<T: ?Sized> Clone for Rc2<T> {
//     fn clone(&self) -> Self {
//         Rc2 {
//             rc: self.rc.clone(),
//         }
//     }
// }
//
// impl<T: ?Sized> Clone for Weak2<T> {
//     fn clone(&self) -> Self {
//         Weak2 {
//             weak: self.weak.clone(),
//         }
//     }
// }
//
// impl<T: ?Sized + Debug> Debug for Rc2<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.rc.fmt(f)
//     }
// }
//
// impl<T: ?Sized> WeakElement for Weak2<T> {
//     type Strong = Rc2<T>;
//
//     fn new(view: &Self::Strong) -> Self {
//         Rc2::downgrade(view)
//     }
//
//     fn view(&self) -> Option<Self::Strong> {
//         self.upgrade()
//     }
//
//     fn clone(view: &Self::Strong) -> Self::Strong {
//         view.clone()
//     }
// }
//
// impl<T: ?Sized + Eq + Hash> WeakKey for Weak2<T> {
//     type Key = T;
//     fn with_key<F, R>(view: &Self::Strong, f: F) -> R
//     where
//         F: FnOnce(&Self::Key) -> R,
//     {
//         f(&*view)
//     }
// }
