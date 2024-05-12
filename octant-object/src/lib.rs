#![feature(trait_upcasting)]
#![feature(trait_alias)]
#![feature(ptr_metadata)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(allocator_api)]
#![feature(const_type_id)]

use std::{
    any::Any
    ,
    marker::Unsize,
    ptr::{DynMetadata, Pointee},
};

pub mod base;
pub mod cast;
pub mod deref_ranked;
pub mod rank;
// pub mod stackbox;
pub mod inlinebox;
// mod raw_trait_object;
pub mod smart_pointer;
mod repr;

pub mod reexports {
    pub use paste;
}

pub trait Class: Any + Unsize<dyn Any> + Pointee<Metadata = DynMetadata<Self>> {
    type Value: ClassValue<Dyn = Self>;
}

pub trait Subclass: Class + Unsize<Self::Parent> {
    type Parent: ?Sized + Class;
}

pub trait ClassValue: Sized + Any + Unsize<Self::Dyn> {
    type Dyn: ?Sized + Class<Value = Self>;
}

#[macro_export]
macro_rules! define_class {
    (
        $(#[$metas:meta])?
        pub class $class:tt extends $parent:tt $(implements $interface:path)? {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
        $crate::reexports::paste::paste!{
            $(#[$metas])?
            pub struct [< $class Value >] {
                parent: <dyn $parent as $crate::Class>::Value,
                $($field : $type,)*
            }
            pub trait $class: $parent $(+ $interface)? {
                fn value(&self) -> &[< $class Value >];
            }
            pub type [< Arc $class >] = ::std::sync::Arc<dyn $class>;
            impl $crate::Class for dyn $class{
                type Value = [< $class Value >];
            }
            impl $crate::ClassValue for [< $class Value >]{
                type Dyn = dyn $class;
            }
            impl $crate::Subclass for dyn $class {
                type Parent = dyn $parent;
            }
            impl $crate::rank::Ranked for [< $class Value >]{
                type Rank = $crate::rank::Succ<<<dyn $parent as $crate::Class>::Value as $crate::rank::Ranked>::Rank>;
            }
            impl<T> $class for T where
                T: $parent,
                $(T: $interface,)?
                T: $crate::rank::Ranked,
                T: $crate::deref_ranked::DerefRanked<T::Rank, <[< $class Value >] as $crate::rank::Ranked>::Rank, TargetRanked = [< $class Value >]>,
            {
                fn value(&self) -> &[< $class Value >]{
                    self.deref_ranked()
                }
            }

            impl ::std::ops::Deref for dyn $class {
                type Target = [< $class Value >];
                fn deref(&self) -> &Self::Target {
                    $class::value(self)
                }
            }

            impl ::std::ops::Deref for [< $class Value >] {
                type Target = <dyn $parent as $crate::Class>::Value;
                fn deref(&self) -> &Self::Target {
                    &self.parent
                }
            }
        }
    };
}
