#![feature(trait_upcasting)]
#![feature(trait_alias)]
#![feature(ptr_metadata)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(generic_nonzero)]
#![feature(allocator_api)]


pub mod base;
pub mod cast;
pub mod deref_ranked;
pub mod rank;
pub mod stackbox;

#[macro_export]
macro_rules! define_class {
    (
        pub class extends $parent:tt $(implements $interface:path)? {
            $($field:ident : $type:ty),* $(,)?
        }
    ) => {
            pub struct Value {
                parent: $parent::Value,
                $($field : $type,)*
            }
            pub trait Trait: $parent::Trait $(+ $interface)? {
                fn value(&self) -> &Value;
            }
            impl $crate::cast::CastValue for Value{
                fn into_leaf_rc<'a>(
                    self: ::std::rc::Rc<Self>,
                    result: &'a mut ::std::mem::MaybeUninit<$crate::stackbox::TraitObjectStorage>,
                ) -> $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>{
                    $crate::stackbox::StackBox::new(self as ::std::rc::Rc<dyn Trait>, result)
                }
                fn into_leaf_arc<'a>(
                    self: ::std::sync::Arc<Self>,
                    result: &'a mut ::std::mem::MaybeUninit<$crate::stackbox::TraitObjectStorage>,
                ) -> $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>{
                    $crate::stackbox::StackBox::new(self as ::std::sync::Arc<dyn Trait>, result)
                }
                fn into_leaf_box<'a>(
                    self: ::std::boxed::Box<Self>,
                    result: &'a mut ::std::mem::MaybeUninit<$crate::stackbox::TraitObjectStorage>,
                ) -> $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>{
                    $crate::stackbox::StackBox::new(self as ::std::boxed::Box<dyn Trait>, result)
                }
            }
            impl $crate::cast::CastTrait for dyn Trait {
                fn into_parent_object(
                    &self,
                ) -> for<'a> fn(
                    this: $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>,
                ) -> ::std::option::Option<$crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>> {
                    fn into_parent_object<'a>(
                        this: $crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>,
                    ) -> ::std::option::Option<$crate::stackbox::StackBox<'a, dyn $crate::cast::CastObject>> {
                        Some($crate::cast::coerce_unsized::<dyn Trait,dyn $parent::Trait>(this))
                    }
                    into_parent_object
                }
            }
            impl $crate::rank::Ranked for Value{
                type Rank = $crate::rank::Succ<<$parent::Value as $crate::rank::Ranked>::Rank>;
            }
            impl<T> Trait for T where
                T: $parent::Trait,
                $(T: $interface,)?
                T: $crate::rank::Ranked,
                T: $crate::deref_ranked::DerefRanked<T::Rank, <Value as $crate::rank::Ranked>::Rank, TargetRanked = Value>,
            {
                fn value(&self) -> &Value{
                    self.deref_ranked()
                }
            }

            impl ::std::ops::Deref for dyn Trait {
                type Target = Value;
                fn deref(&self) -> &Self::Target {
                    Trait::value(self)
                }
            }

            impl ::std::ops::Deref for Value {
                type Target = $parent::Value;
                fn deref(&self) -> &Self::Target {
                    &self.parent
                }
            }


    };
}
