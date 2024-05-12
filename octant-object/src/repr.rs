use std::{
    alloc::Layout,
    ptr::{DynMetadata, Pointee},
};
use std::rc::Rc;
use std::sync::Arc;

pub trait IsRepr: 'static {}

pub trait HasRepr: Sized {
    type Repr: IsRepr;
    const VALID_SIZE: () =
        assert!(Layout::new::<Self>().size() == Layout::new::<Self::Repr>().size());
    const VALID_ALIGN: () =
        assert!(Layout::new::<Self>().align() == Layout::new::<Self::Repr>().align());
}

impl IsRepr for () {}
impl HasRepr for () {
    type Repr = ();
}

#[repr(C)]
pub struct NarrowRepr(*const ());
impl IsRepr for NarrowRepr {}

impl<T: ?Sized> HasRepr for DynMetadata<T> {
    type Repr = NarrowRepr;
}

#[repr(C)]
pub struct FatRepr<T>(NarrowRepr, T);

impl<T: IsRepr> IsRepr for FatRepr<T> {}

impl<T: ?Sized> HasRepr for Box<T>
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = FatRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized> HasRepr for Rc<T>
    where
        <T as Pointee>::Metadata: HasRepr,
{
    type Repr = FatRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized> HasRepr for Arc<T>
    where
        <T as Pointee>::Metadata: HasRepr,
{
    type Repr = FatRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized> HasRepr for *const T
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = FatRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}
