//! "Object-oriented" programming in rust.
//! * ✅ Field inheritance.
//! * ✅ Method inheritance.
//! * ✅ Upcasting coercions (implicitly with trait upcasting).
//! * ✅ Downcasting (explicitly with the [cast] module).
//! * ✅ `#[derive]` and other attributes for value types.
//! * ✅ Generics and where bounds.
//!
//! * ❌ Virtual methods.
//! * ❌ Guaranteed static dispatch.
//! * ❌ Subclasses as [subtypes](https://doc.rust-lang.org/reference/subtyping.html). E.g.
//!   `Vec<Box<Subclass>>` is not implicitly coercible to `Vec<Box<Superclass>>` because the
//!   vtable pointer would need to be modified on each box.
//!
//! ```
//! #![feature(trait_upcasting)]
//! # use std::any::Any;
//! use octant_object::base::{Base, BaseValue};
//! use octant_object::define_class;
//!
//! define_class! {
//!     pub class Animal extends Base {
//!         field num_legs: usize;
//!         fn num_toes(&self) -> usize {
//!             self.animal().num_legs * 5
//!         }
//!         fn is_dog(&self) -> bool{
//!             (self as &dyn Any).is::<DogValue>()
//!         }
//!     }
//! }
//!
//! define_class! {
//!     pub class Dog extends Animal {
//!         field name: String;
//!     }
//! }
//!
//! impl AnimalValue {
//!     // Constructors return value types so that subclasses can call constructors.
//!     pub fn new(num_legs: usize) -> Self {
//!         AnimalValue {
//!             parent: BaseValue::new(),
//!             num_legs
//!         }
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
//! // Methods are inherited
//! assert_eq!(dog.num_toes(), 20);
//! // Fields are inherited.
//! assert_eq!(dog.num_legs, 4);
//! // Upcast coercions are implicit.
//! let dog: Box<dyn Animal> = dog;
//! ```
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
//!     pub class Animal extends Base implements SendSyncDebug {
//!         field num_legs: usize;
//!     }
//! }
//! define_class! {
//!     pub class Dog extends Animal {
//!         field name: String;
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
//! let dog: Box<dyn Debug> = Box::new(DogValue::new("otto".to_string()));
//! assert_eq!(&format!("{:?}",dog), r#"Dog { num_legs: 4, name: "otto" }"#);
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

    pub use octant_reffed;
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
/// # #[derive(Debug)]
/// # struct Field;
/// define_class! {
///     pub class Foo extends Bar implements Baz {
///         field field: Field;
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
/// // and one `impl T for FooValue` for each ancestor `T`
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
        pub class $class:tt $(<$($generics:ident),*>)?
            extends $parent:tt $(< $($parent_generics:ty),* >)?
            $(implements $interface:path)?
            $(where [$($where:tt)*])? {
            $( field $field_vis:vis $field:ident : $type:ty;)*
            $(
                fn $method:ident
                $(<
                    $(
                    $lifetime:lifetime
                    ),*
                >)?
                ($($params:tt)*)
                $(-> $return_type:ty)? $body:block
            )*
        }
    } => {
        $crate::reexports::paste::paste!{
            $(#[$metas])?
            pub struct [< $class Value >] $(< $($generics:'static),*>)? $(where $($where)*)?{
                parent: <dyn $parent $(< $($parent_generics),*>)? as $crate::class::Class>::Value,
                $($field_vis $field : $type,)*
            }
            impl $(< $($generics:'static + ::std::fmt::Debug),*>)? ::std::fmt::Debug for [< $class Value >] $(< $($generics),*>)? $(where $($where)*)? {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    let mut f = f.debug_struct(::std::stringify!($class));
                    $crate::class::DebugClass::fmt_class(self, &mut f);
                    f.finish()
                }
            }
            impl $(< $($generics:'static + ::std::fmt::Debug),*>)? $crate::class::DebugClass for [< $class Value >] $(< $($generics),*>)? $(where $($where)*)? {
                fn fmt_class(&self, f: &mut ::std::fmt::DebugStruct) {
                    $crate::class::DebugClass::fmt_class(&self.parent, f);
                    $(
                        f.field(std::stringify!($field), &self.$field);
                    )*
                }
            }
            pub trait $class $(< $($generics:'static),*>)? : $parent $(< $($parent_generics),*>)? $(+ $interface)? $(where $($where)*)? {
                fn [< $class:snake >](&self) -> &[< $class Value >] $(< $($generics),*>)?;
                fn [< $class:snake _mut >](&mut self) -> &mut [< $class Value >] $(< $($generics),*>)?;
                $(
                    fn $method $(<$($lifetime),*>)?(
                        $( $params )*
                    ) $(-> $return_type)?;
                )*
            }
            pub type [< Rc $class >] $(< $($generics),*>)? = $crate::reexports::octant_reffed::rc::Rc2<dyn 'static + $class $(< $($generics),*>)?>;
            impl $(< $($generics:'static),*>)? $crate::class::Class for dyn $class $(< $($generics),*>)? $(where $($where)*)? {
                type Value = [< $class Value >] $(< $($generics),*>)?;
            }
            impl $(< $($generics:'static),*>)? $crate::class::ClassValue for [< $class Value >] $(< $($generics),*>)? $(where $($where)*)?{
                type Dyn = dyn $class $(< $($generics),*>)?;
            }
            impl $(< $($generics:'static),*>)? $crate::class::Subclass for dyn $class $(< $($generics),*>)? $(where $($where)*)?{
                type Parent = dyn $parent $(< $($parent_generics),*>)?;
            }
            impl $(< $($generics:'static),*>)? $crate::class::Ranked for [< $class Value >]$(< $($generics),*>)? $(where $($where)*)? {
                type Rank = $crate::class::Succ<<<dyn $parent $(< $($parent_generics),*>)? as $crate::class::Class>::Value as $crate::class::Ranked>::Rank>;
            }
            impl<__super_secret__T $(, $($generics:'static)*)?> $class $(< $($generics),*>)? for __super_secret__T where
                __super_secret__T: $parent $(< $($parent_generics),*>)?,
                $(__super_secret__T: $interface,)?
                __super_secret__T: $crate::class::Ranked,
                __super_secret__T: $crate::class::DerefRanked<__super_secret__T::Rank, <[< $class Value >] $(< $($generics),*>)? as $crate::class::Ranked>::Rank, TargetRanked = [< $class Value >] $(< $($generics),*>)?>,
                $( $($where)*)?
            {
                fn [< $class:snake >](&self) -> &[< $class Value >] $(< $($generics),*>)? {
                    self.deref_ranked()
                }
                fn [< $class:snake _mut >](&mut self) -> &mut [< $class Value >] $(< $($generics),*>)?{
                    self.deref_mut_ranked()
                }
                $(
                    fn $method $(<$($lifetime),*>)?(
                        $($params)*
                    ) $(-> $return_type)?
                    $body
                )*
            }

            impl $(< $($generics:'static),*>)? ::std::ops::Deref for dyn $class $(< $($generics),*>)? $(where $($where)*)? {
                type Target = [< $class Value >] $(< $($generics),*>)?;
                fn deref(&self) -> &Self::Target {
                    $class$(::< $($generics),*>)?::[< $class:snake >](self)
                }
            }

            impl $(< $($generics:'static),*>)? ::std::ops::DerefMut for dyn $class $(< $($generics),*>)? $(where $($where)*)? {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    $class$(::< $($generics),*>)?::[< $class:snake _mut >](self)
                }
            }

            impl $(< $($generics:'static),*>)? ::std::ops::Deref for [< $class Value >] $(< $($generics),*>)? $(where $($where)*)? {
                type Target = <dyn $parent $(< $($parent_generics),*>)? as $crate::class::Class>::Value;
                fn deref(&self) -> &Self::Target {
                    &self.parent
                }
            }
            impl $(< $($generics:'static),*>)? ::std::ops::DerefMut for [< $class Value >] $(< $($generics),*>)? $(where $($where)*)? {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.parent
                }
            }
        }
    };
}
