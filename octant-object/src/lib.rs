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
//! # use octant_object::base::{Base, BaseFields};
//! # use octant_object_derive::class;
//!
//! pub struct AnimalFields {
//!     parent: BaseFields,
//!     num_legs: usize,
//! }
//! #[class]
//! pub trait Animal: Base{
//!     fn num_toes(&self) -> usize {
//!         self.animal().num_legs * 5
//!     }
//!     fn is_dog(&self) -> bool{
//!         (self as &dyn Any).is::<DogFields>()
//!     }
//! }
//!
//! pub struct DogFields {
//!     parent: AnimalFields,
//!     name: String,
//! }
//! #[class]
//! pub trait Dog: Animal {
//!
//! }
//!
//! impl AnimalFields {
//!     // Constructors should return value types so that subclasses can call constructors.
//!     pub fn new(num_legs: usize) -> Self {
//!         AnimalFields {
//!             parent: BaseFields::new(),
//!             num_legs
//!         }
//!     }
//! }
//!
//! impl DogFields {
//!     pub fn new(name: String) -> Self {
//!         DogFields {
//!             parent: AnimalFields::new(4),
//!             name,
//!         }
//!     }
//! }
//!
//! let dog: Box<dyn Dog> = Box::new(DogFields::new("otto".to_string()));
//! // Methods are inherited
//! assert_eq!(dog.num_toes(), 20);
//! // Fields are inherited.
//! assert_eq!(dog.num_legs, 4);
//! // Upcast coercions are implicit.
//! let dog: Box<dyn Animal> = dog;
//! ```
//! # Including additional trait bounds
//! A class `Foo` may include additional object-safe supertraits. This forces `FooValue` <b>and
//! every subclass of `Foo`</b> to meet those bounds.
//! ```
//! # #![feature(trait_upcasting)]
//! # use std::fmt::Debug;
//! # use octant_object::base::{Base, BaseFields};
//! # use octant_object_derive::{class, DebugClass};
//! #[derive(DebugClass)]
//! pub struct AnimalFields {
//!     parent: BaseFields,
//!     num_legs: usize,
//! }
//! #[class]
//! pub trait Animal: Base + Send + Sync + Debug {
//! }
//!
//! #[derive(DebugClass)]
//! pub struct DogFields {
//!     parent: AnimalFields,
//!     name: String,
//! }
//! #[class]
//! pub trait Dog: Animal {}
//!
//! impl AnimalFields {
//!     pub fn new(num_legs: usize) -> Self {
//!         AnimalFields {
//!             parent: BaseFields::new(),
//!             num_legs
//!         }
//!     }
//! }
//! impl DogFields {
//!     pub fn new(name: String) -> Self {
//!         DogFields {
//!             parent: AnimalFields::new(4),
//!             name,
//!         }
//!     }
//! }
//! let dog: Box<dyn Debug> = Box::new(DogFields::new("otto".to_string()));
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

pub use octant_object_derive::*;
