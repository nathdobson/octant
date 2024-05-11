#![feature(trait_upcasting)]
#![feature(trait_alias)]
#![feature(ptr_metadata)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(allocator_api)]

pub mod base;
pub mod cast;
pub mod deref_ranked;
pub mod rank;
pub mod stackbox;

pub mod reexports {
    pub use paste;
}

pub trait Class {
    type Value;
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
            impl $crate::cast::CastValue for [< $class Value >] {
                fn into_leaf_rc<'a>(
                    self: ::std::rc::Rc<Self>,
                    result: &'a mut ::std::mem::MaybeUninit<$crate::stackbox::TraitObjectStorage>,
                ) -> $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>{
                    $crate::stackbox::StackBox::new(self as ::std::rc::Rc<dyn $class>, result)
                }
                fn into_leaf_arc<'a>(
                    self: ::std::sync::Arc<Self>,
                    result: &'a mut ::std::mem::MaybeUninit<$crate::stackbox::TraitObjectStorage>,
                ) -> $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>{
                    $crate::stackbox::StackBox::new(self as ::std::sync::Arc<dyn $class>, result)
                }
                fn into_leaf_box<'a>(
                    self: ::std::boxed::Box<Self>,
                    result: &'a mut ::std::mem::MaybeUninit<$crate::stackbox::TraitObjectStorage>,
                ) -> $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>{
                    $crate::stackbox::StackBox::new(self as ::std::boxed::Box<dyn $class>, result)
                }
            }
            impl $crate::cast::CastTrait for dyn $class {
                fn into_parent_object(
                    &self,
                ) -> for<'a> fn(
                    this: $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>,
                ) -> ::std::option::Option<$crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>> {
                    fn into_parent_object<'a>(
                        this: $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>,
                    ) -> ::std::option::Option<$crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>> {
                        Some($crate::cast::coerce_unsized::<dyn $class,dyn $parent>(this))
                    }
                    into_parent_object
                }
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
