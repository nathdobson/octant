//! "Object-oriented" programming in rust.
//! * ✅ Field inheritance.
//! * ✅ Method inheritance on value types.
//! * ✅ Upcasting coercions (implicitly with trait upcasting).
//! * ✅ Downcasting (explicitly with the [cast] module).
//! * ✅ Non-virtual methods.
//! * ✅ `#[derive]` and other attributes for value types.
//!
//! * ❌ Implicit method inheritance on `dyn` types. Methods are still accessible, but the object
//!     must be explicitly upcast.
//! * ❌ Virtual methods.
//! * ❌ Method inheritance.
//! * ❌ Generics.
//! * ❌ Subclasses as [subtypes](https://doc.rust-lang.org/reference/subtyping.html). E.g. `Vec<Subclass>` is not implicitly coercible to `Vec<Superclass>`.
//!
//! ```
//! #![feature(trait_upcasting)]
//! # use std::any::Any;
//! use octant_object::base::{Base, BaseValue};
//! use octant_object::define_class;
//!
//! define_class! {
//!     pub class Animal extends Base {
//!         num_legs: usize,
//!     }
//! }
//!
//! define_class! {
//!     pub class Dog extends Animal {
//!         name: String,
//!     }
//! }
//!
//! impl AnimalValue {
//!     pub fn new(num_legs: usize) -> Self {
//!         AnimalValue {
//!             parent: BaseValue::new(),
//!             num_legs
//!         }
//!     }
//!     fn num_toes(&self) -> usize {
//!         self.num_legs * 5
//!     }
//! }
//!
//! impl dyn Animal {
//!     // This method must be implemented on `&dyn Animal` instead of `&AnimalValue`, because
//!     // `&AnimalValue` doesn't know if it's a field inside a `DogValue`.
//!     fn is_dog(&self) -> bool{
//!         (self as &dyn Any).is::<DogValue>()
//!     }
//! }
//!
//! impl DogValue {
//!     pub fn new(name: String) -> Self {
//!         DogValue {
//!             parent: AnimalValue::new(4),
//!             name,
//!         }
//!     }
//! }
//!
//! let dog: Box<dyn Dog> = Box::new(DogValue::new("otto".to_string()));
//! // Methods on `dyn Animal` are not inherited, but can be accessed through explicit upcasts.
//! assert!((&*dog as &dyn Animal).is_dog());
//! // Methods on `AnimalValue` are inherited.
//! assert_eq!(dog.num_toes(), 20);
//! // Fields are inherited.
//! assert_eq!(dog.num_legs, 4);
//! // Upcast coercions are implicit.
//! let dog: Box<dyn Animal> = dog;
//! ```
//! # `impl FooValue {}` vs `impl dyn Foo {}`
//! Methods can be added to objects of the class `Foo` in two ways:
//! ## `impl FooValue {}`
//!  *  Used for constructors and other functions without a `self` parameter.
//!  *  Methods using `self` are not inherited.
//!  *  Methods using `&self` and `&mut self` are inherited.
//!  *  Methods using `self: Box<Self>`, `self: Rc<Self>`, or `self: Arc<Self>` are not inherited,
//!     so they're probably wrong. This prevents cloning of the pointer to this object.
//!  *  Methods cannot determine which subclass is being used.
//!  *  Methods cannot perform downcasts.
//!  *  Methods cannot access fields of subclasses.
//! ## `impl dyn Foo {}`
//!  *  All methods are inherited, but the caller must explicitly upcast to `dyn Foo`.
//!  *  Methods using `self` would require [unsized_locals](https://doc.rust-lang.org/unstable-book/language-features/unsized-locals.html), so are not recommended.
//!  *  Methods using `&self` and `&mut self` offer nothing over `impl FooValue {}`, so they are probably wrong.
//!  *  Methods using `self: Box<Self>`, `self: Rc<Self>`, or `self: Arc<Self>` may perform downcasts and may clone self.
//! # `implements`
//! A class `Foo` may declare that it <i>implements</i> an object-safe trait `Baz`, which makes
//! `Baz` a [supertrait](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html) of
//! `Foo`. This forces `FooValue` <b>and every subclass of `Foo`</b> to implement `Baz`. The macro
//! only accepts one trait, but multiple traits can be specified by combining them into a single
//! trait.
//! ```
//! # #![feature(trait_upcasting)]
//! # use std::fmt::Debug;
//! # use octant_object::base::{Base, BaseValue};
//! # use octant_object::define_class;
//! trait SendSyncDebug : Send + Sync + Debug {}
//! impl<T: Send + Sync + Debug> SendSyncDebug for T{}
//! define_class! {
//!     #[derive(Debug)]
//!     pub class Animal extends Base implements SendSyncDebug {
//!         num_legs: usize,
//!     }
//! }
//! define_class! {
//!     #[derive(Debug)]
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
//! impl DogValue {
//!     pub fn new(name: String) -> Self {
//!         DogValue {
//!             parent: AnimalValue::new(4),
//!             name,
//!         }
//!     }
//! }
//! let dog: Box<dyn Animal> = Box::new(DogValue::new("otto".to_string()));
//! assert_eq!(&format!("{:?}",dog), r#"DogValue { parent: AnimalValue { parent: BaseValue, num_legs: 4 }, name: "otto" }"#);
//! ```

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
