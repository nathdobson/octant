#![deny(unused_must_use)]
#![feature(macro_metavar_expr)]
#![feature(unsize)]

pub use error::*;
pub use handle::*;
use octant_object::class::Class;

// #[doc(hidden)]
// pub mod reexports {
//     pub use anyhow;
//     pub use catalog;
//     pub use paste;
//     pub use serde;
//
//     pub use octant_object;
//     pub use octant_serde;
// }
//
// mod error;
// mod handle;
//
// #[macro_export]
// macro_rules! define_sys_class {
//     {
//         class $class:ident;
//         extends $parent:path;
//         wasm $wasm:path;
//         $(new_client $new_client_dummy:tt;)?
//         $(new_server $new_server_dummy:tt;)?
//         $(client_field $client_field:ident : $client_field_type:ty ;)*
//         $(server_field $server_field:ident : $server_field_type:ty ;)*
//     } => {
//         $crate::reexports::paste::paste! {
//             #[cfg(side = "client")]
//             $crate::reexports::octant_object::define_class! {
//                 #[derive(Debug)]
//                 pub class $class extends $parent implements ::std::fmt::Debug{
//                     [< $class:snake >]: $wasm,
//                     $($client_field : $client_field_type, )*
//                 }
//             }
//
//             #[cfg(any($(all() ${ignore($new_client_dummy)} )?))]
//             #[cfg(side = "client")]
//             impl $crate::FromHandle for dyn $class {
//                 type Builder = $wasm;
//                 fn from_handle(handle: $crate::NewTypedHandle<Self>, [< $class:snake >]: Self::Builder) -> [< $class Value >]  {
//                     [< $class Value >] {
//                         parent: <dyn $parent as $crate::FromHandle>::from_handle(handle.unsize(), ::std::clone::Clone::clone(&
//                         [< $class:snake >]).into()),
//                         [< $class:snake >],
//                         $($client_field : ::std::default::Default::default(), )*
//                     }
//                 }
//             }
//
//             #[cfg(side="client")]
//             impl [< $class Value >] {
//                 pub fn native(&self) -> &$wasm{
//                     &self.[< $class:snake >]
//                 }
//             }
//
//             #[cfg(side = "server")]
//             $crate::reexports::octant_object::define_class! {
//                 #[derive(Debug)]
//                 pub class $class extends $parent {
//                     $($server_field : $server_field_type, )*
//                 }
//             }
//             #[cfg(any($(all() ${ignore($new_server_dummy)} )?))]
//             #[cfg(side = "server")]
//             impl [< $class Value >] {
//                 pub fn new(handle: ::octant_gui::handle::HandleValue) -> Self {
//                     [< $class Value >] {
//                         parent: <dyn $parent as $crate::reexports::octant_object::class::Class>::Value::new(handle),
//                         $($server_field : ::std::default::Default::default(), )*
//                     }
//                 }
//             }
//
//             impl $crate::reexports::serde::Serialize for dyn $class {
//                 fn serialize<S>(&self, s: S) -> ::std::result::Result<S::Ok, S::Error>
//                 where
//                     S: $crate::reexports::serde::Serializer,
//                 {
//                     #[cfg(side = "server")]
//                     return self.handle().handle().serialize(s);
//                     #[cfg(side = "client")]
//                     return self.raw_handle().serialize(s);
//                 }
//             }
//         }
//     };
// }
//
// #[macro_export]
// macro_rules! define_sys_rpc {
//     {
//         fn $name:ident($runtime:ident $(, $input_name:ident: $input:ty)*) -> ( $( $output:ident, )* ) { $($imp:tt)* }
//     } => {
//         $crate::reexports::paste::paste! {
//             #[cfg(side = "server")]
//             fn $name(
//                 runtime: &::std::sync::Arc<::octant_gui::Runtime>
//                 $(, $input_name: $input)*
//             ) -> (
//                 $(
//                     ::std::sync::Arc<dyn $output>
//                 ),*
//             ) {
//                 $(
//                     let [< output_ ${index()} >] = ::std::sync::Arc::new(<dyn $output as $crate::reexports::octant_object::class::Class>::Value::new(runtime.add_uninit()));
//                 )*
//                 runtime.new_send(Box::<[< $name:camel Request >]>::new([< $name:camel Request >] {
//                     $($input_name,)*
//                     $(
//                         ${ignore($output)}
//                         [< output_ ${index()} >]: $crate::NewTypedHandle::new(
//                             [< output_ ${index()} >].handle()
//                         ),
//                     )*
//                 }));
//                 ( $(
//                     ${ignore($output)}
//                     [< output_ ${index()} >]
//                 ),* )
//             }
//
//             #[derive($crate::reexports::serde::Serialize,Debug)]
//             pub struct [< $name:camel Request >] {
//                 $($input_name: $input,)*
//                 $(
//                     [< output_ ${index()} >]: $crate::NewTypedHandle<dyn $output>,
//                 )*
//             }
//
//             impl<'de> $crate::reexports::octant_serde::DeserializeWith<'de> for [< $name:camel Request >] {
//                 fn deserialize_with<D:$crate::reexports::serde::Deserializer<'de>>(ctx: &$crate::reexports::octant_serde::TypeMap, d:D)->Result<Self,D::Error>{
//                     todo!();
//                 }
//             }
//
//             #[cfg(side="client")]
//             $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: octant_gui_client::ClientDownMessage);
//             #[cfg(side="client")]
//             impl octant_gui_client::ClientDownMessage for [< $name:camel Request >] {
//                 fn run(self:Box<Self>, runtime:&Arc<octant_gui_client::Runtime>) -> $crate::reexports::anyhow::Result<()>{
//                     todo!();
//                 }
//             }
//
//             #[cfg(side="server")]
//             $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: octant_gui::ServerDownMessage);
//             #[cfg(side="server")]
//             impl octant_gui::ServerDownMessage for [< $name:camel Request >] {}
//
//             // #[cfg(side = "client")]
//             // #[$crate::reexports::catalog::register(octant_gui_client::DOWN_MESSAGE_HANDLER_REGISTRY)]
//             // fn [<handle_ $name>]() -> octant_gui_client::DownMessageHandler<[< $name:camel Request >]> {
//             //     |runtime: &::std::sync::Arc<::octant_gui_client::Runtime>, req: [< $name:camel Request >]| {
//             //         let runtime = runtime.clone();
//             //         let result = [<impl_ $name>](&runtime $(, req.$input_name)*)?;
//             //         $(
//             //             runtime.add_new(req.[< output_ ${index()} >], ::std::sync::Arc::new(<dyn $output as $crate::FromHandle>::from_handle(req.[< output_ ${index()} >], result.${index()})));
//             //         )*
//             //         Ok(())
//             //     }
//             // }
//
//             #[cfg(side="client")]
//             fn [<impl_ $name>](
//                 $runtime: &::std::sync::Arc<octant_gui_client::Runtime>,
//                 $($input_name: $input,)*
//             ) -> $crate::reexports::anyhow::Result<
//                 ($(
//                     <dyn $output as $crate::FromHandle>::Builder,
//                 )*)
//             >{
//                 $($imp)*
//             }
//         }
//     };
// }

pub trait FromHandle: Class {
    type Builder;
    fn from_handle(handle: NewTypedHandle<Self>, builder: Self::Builder) -> Self::Value;
}
