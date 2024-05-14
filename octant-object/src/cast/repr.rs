//! Traits describing the <i>representation</i> of types. The <i>representation</i> of a type is
//! another type with the same layout (size and alignment). If two types have the same
//! representation, they are guaranteed to have the same layout on any platform.

use std::{
    alloc::Layout,
    ptr::{DynMetadata, Pointee},
    rc::Rc,
    sync::Arc,
};

/// A type used as a representation for other types.
pub trait IsRepr: 'static {}

/// A type with a representation.
pub trait HasRepr: Sized {
    /// The representation of this type.
    type Repr: IsRepr;
    /// A static assertion that the representation's size is valid. Callers requiring this assertion
    /// must explicitly evaluate it with `let _ : () = T::VALID_SIZE`.
    const VALID_SIZE: () =
        assert!(Layout::new::<Self>().size() == Layout::new::<Self::Repr>().size());
    /// A static assertion that the representation's alignment is valid. Callers requiring this
    /// assertion must explicitly evaluate it with `let _ : () = T::VALID_SIZE`.
    const VALID_ALIGN: () =
        assert!(Layout::new::<Self>().align() == Layout::new::<Self::Repr>().align());
}

impl IsRepr for () {}
impl HasRepr for () {
    type Repr = ();
}

impl<T: ?Sized> HasRepr for DynMetadata<T> {
    type Repr = PtrRepr<()>;
}

impl IsRepr for u8 {}
impl HasRepr for u8 {
    type Repr = u8;
}
impl HasRepr for i8 {
    type Repr = u8;
}

impl IsRepr for u16 {}
impl HasRepr for u16 {
    type Repr = u16;
}
impl HasRepr for i16 {
    type Repr = u16;
}

impl IsRepr for u32 {}
impl HasRepr for u32 {
    type Repr = u32;
}
impl HasRepr for i32 {
    type Repr = u32;
}

impl IsRepr for u64 {}
impl HasRepr for u64 {
    type Repr = u64;
}
impl HasRepr for i64 {
    type Repr = u64;
}

impl HasRepr for usize {
    type Repr = PtrRepr<()>;
}
impl HasRepr for isize {
    type Repr = PtrRepr<()>;
}

/// `PtrRepr<T>` is the representation of pointers and references where `T` is the representation of pointer metadata.
#[repr(C)]
pub struct PtrRepr<T>(*const (), T);

impl<T: IsRepr> IsRepr for PtrRepr<T> {}

impl<T: ?Sized> HasRepr for Box<T>
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = PtrRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized> HasRepr for Rc<T>
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = PtrRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized> HasRepr for Arc<T>
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = PtrRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized> HasRepr for *const T
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = PtrRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<T: ?Sized> HasRepr for *mut T
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = PtrRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<'a, T: ?Sized> HasRepr for &'a T
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = PtrRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

impl<'a, T: ?Sized> HasRepr for &'a mut T
where
    <T as Pointee>::Metadata: HasRepr,
{
    type Repr = PtrRepr<<<T as Pointee>::Metadata as HasRepr>::Repr>;
}

