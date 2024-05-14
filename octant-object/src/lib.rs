//! "Object-oriented" programming in rust.
//! * ✅ Const field inheritance.
//! * ✅ Upcasting (implicitly with trait upcasting).
//! * ✅ Downcasting (explicitly with the [cast] module).
//! * ✅ Non-virtual methods.
//!
//! * ❌ Mutable field inheritance.
//! * ❌ Virtual methods.
//! * ❌ Method inheritance.
//! * ❌ Generics.
//!
//! ```
//! #![feature(trait_upcasting)]
//! use std::any::Any;
//! use octant_object::base::{Base, BaseValue};
//! use octant_object::define_class;
//! define_class! {
//!     pub class Animal extends Base {
//!         num_legs: usize,
//!     }
//! }
//! define_class! {
//!     pub class Dog extends Animal {
//!         name: String,
//!     }
//! }
//! impl AnimalValue {
//!     pub fn new(num_legs: usize) -> Self {
//!         AnimalValue {
//!             parent: BaseValue::new(),
//!             num_legs
//!         }
//!     }
//! }
//! impl dyn Animal {
//!     fn is_dog(&self) -> bool {
//!         (self as &dyn Any).is::<DogValue>()
//!     }
//! }
//! impl DogValue {
//!     pub fn new(name: String) -> Self {
//!         DogValue {
//!             parent: AnimalValue::new(4),
//!             name,
//!         }
//!     }
//! }
//! let dog: Box<dyn Dog> = Box::new(DogValue::new("otto".to_string()));
//! assert_eq!(dog.num_legs, 4);
//! let dog: Box<dyn Animal> = dog;
//! assert_eq!(dog.num_legs, 4);
//! assert!(dog.is_dog());
//! ```
//!
//!

#![feature(trait_upcasting)]
#![feature(trait_alias)]
#![feature(ptr_metadata)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(allocator_api)]
#![feature(const_type_id)]

pub mod base;
pub mod cast;
pub mod class;

#[doc(hidden)]
pub mod reexports {
    pub use paste;
}

/// Create a new class.
///
/// A <i>class</i> is a trait `Foo` and a struct `FooValue` where
/// `FooValue: Foo`.
///
/// A class `Foo` <i>extends</i> a class `Bar` if:
/// * `FooValue` contains a field named `parent` of type `BarValue`.
/// * the `Foo` trait extends the `Bar` trait.
/// * Dereferencing a `&FooValue` returns a `&BarValue`.
///
/// Values of type `FooValue` are direct <i>instances</i> of the `Foo` class.
/// Values of type `dyn Foo` are <i>instances</i> of the `Foo` class or one of its descendants.
///
/// A class `Foo` <i>implements</i> a trait `Baz` if:
/// * the `Foo` trait extends the `Baz` trait.
/// * `FooValue` and all descendants implement `Baz`. Note that this is fundamentally different
///   from interfaces in Java.
///
/// The following code:
/// ```
/// # use octant_object::define_class;
/// # use octant_object::base::Base;
/// # define_class! {
/// #     pub class Bar extends Base{}
/// # }
/// # trait Baz{}
/// # struct Field;
/// define_class! {
///     pub class Foo extends Bar implements Baz {
///         field: Field,
///     }
/// }
/// # impl Baz for FooValue {}
/// ```
/// expands to code similar to:
/// ```
/// # use std::ops::Deref;
/// # pub struct BarValue;
/// # struct Field;
/// # trait Bar{}
/// # trait Baz{}
/// # impl Baz for FooValue {}
/// pub struct FooValue {
///   parent: BarValue,
///   field: Field,
/// }
/// pub trait Foo: Bar + Baz {
/// }
///
/// impl Foo for FooValue {}
/// impl Bar for FooValue {}
/// // and one `impl T for FooValue` for each ancestor
///
/// // Make the fields of FooValue available to dyn Foo,
/// impl Deref for dyn Foo {
///     type Target = FooValue;
///     fn deref(&self) -> &Self::Target {
///         // ...
///         # todo!()
///     }
/// }
///
/// // Make the fields of `BarValue` available to `FooValue`,
/// impl Deref for FooValue {
///     type Target = BarValue;
///     fn deref(&self) -> &Self::Target {
///         &self.parent
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_class {
    {
        $(#[$metas:meta])?
        pub class $class:tt extends $parent:tt $(implements $interface:path)? {
            $($field:ident : $type:ty),* $(,)?
        }
    } => {
        $crate::reexports::paste::paste!{
            $(#[$metas])?
            pub struct [< $class Value >] {
                parent: <dyn $parent as $crate::class::Class>::Value,
                $($field : $type,)*
            }
            pub trait $class: $parent $(+ $interface)? {
                fn [< $class:snake >](&self) -> &[< $class Value >];
                fn [< $class:snake _mut >](&mut self) -> &mut [< $class Value >];
            }
            pub type [< Arc $class >] = ::std::sync::Arc<dyn $class>;
            impl $crate::class::Class for dyn $class{
                type Value = [< $class Value >];
            }
            impl $crate::class::ClassValue for [< $class Value >]{
                type Dyn = dyn $class;
            }
            impl $crate::class::Subclass for dyn $class {
                type Parent = dyn $parent;
            }
            impl $crate::class::Ranked for [< $class Value >]{
                type Rank = $crate::class::Succ<<<dyn $parent as $crate::class::Class>::Value as $crate::class::Ranked>::Rank>;
            }
            impl<T> $class for T where
                T: $parent,
                $(T: $interface,)?
                T: $crate::class::Ranked,
                T: $crate::class::DerefRanked<T::Rank, <[< $class Value >] as $crate::class::Ranked>::Rank, TargetRanked = [< $class Value >]>,
            {
                fn [< $class:snake >](&self) -> &[< $class Value >]{
                    self.deref_ranked()
                }
                fn [< $class:snake _mut >](&mut self) -> &mut [< $class Value >]{
                    self.deref_mut_ranked()
                }
            }

            impl ::std::ops::Deref for dyn $class {
                type Target = [< $class Value >];
                fn deref(&self) -> &Self::Target {
                    $class::[< $class:snake >](self)
                }
            }

            impl ::std::ops::DerefMut for dyn $class {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    $class::[< $class:snake _mut >](self)
                }
            }

            impl ::std::ops::Deref for [< $class Value >] {
                type Target = <dyn $parent as $crate::class::Class>::Value;
                fn deref(&self) -> &Self::Target {
                    &self.parent
                }
            }
            impl ::std::ops::DerefMut for [< $class Value >] {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.parent
                }
            }
        }
    };
}
