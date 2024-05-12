use std::{
    any::Any,
    marker::Unsize,
    ptr,
    ptr::{DynMetadata, Pointee},
};

use crate::inlinebox::InlineBox;

pub struct RawTraitObject<M: ?Sized> {
    ptr: *const (),
    metadata: InlineBox<M, *const ()>,
}

impl<T: ?Sized> RawTraitObject<DynMetadata<T>>
where
    T: Pointee<Metadata = DynMetadata<T>>,
{
    pub fn new(ptr: *const T) -> Self {
        let metadata: DynMetadata<T> = std::ptr::metadata(ptr);
        let b = InlineBox::<<T as Pointee>::Metadata, *const ()>::new(metadata);
        RawTraitObject {
            ptr: ptr as *const (),
            metadata: b,
        }
    }
    pub fn unsize<U>(self) -> RawTraitObject<U>
    where
        DynMetadata<T>: Unsize<U>,
    {
        RawTraitObject {
            ptr: self.ptr,
            metadata: self.metadata.unsize(),
        }
    }
    pub fn deref_unchecked(&self) -> *const T {
        ptr::from_raw_parts(self.ptr, *self.metadata)
    }
}

impl RawTraitObject<dyn Any> {
    fn downcast<T: ?Sized + 'static>(this: Self) -> Result<RawTraitObject<DynMetadata<T>>, Self> {
        match InlineBox::downcast::<DynMetadata<T>>(this.metadata) {
            Ok(metadata) => Ok(RawTraitObject {
                ptr: this.ptr,
                metadata,
            }),
            Err(metadata) => Err(RawTraitObject {
                ptr: this.ptr,
                metadata,
            }),
        }
    }
}
