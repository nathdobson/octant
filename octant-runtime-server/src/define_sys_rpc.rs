
#[macro_export]
macro_rules! define_sys_rpc {
    {
        fn $name:ident($runtime:ident $(, $input_name:ident: $input:ty)*) -> ( $( $output:ident, )* ) { $($imp:tt)* }
    } => {
        $crate::reexports::paste::paste! {
            #[cfg(side = "server")]
            fn $name(
                runtime: &::std::sync::Arc<$crate::runtime::Runtime>
                $(, $input_name: $input)*
            ) -> (
                $(
                    ::std::sync::Arc<dyn $output>
                ),*
            ) {
                $(
                    let [< output_ ${index()} >] = ::std::sync::Arc::new(<dyn $output as $crate::reexports::octant_object::class::Class>::Value::new(runtime.add_uninit()));
                )*
                runtime.send(Box::<[< $name:camel Request >]>::new([< $name:camel Request >] {
                    $($input_name,)*
                    $(
                        ${ignore($output)}
                        [< output_ ${index()} >]: $crate::handle::TypedHandle::new(
                            [< output_ ${index()} >].raw_handle()
                        ),
                    )*
                }));
                ( $(
                    ${ignore($output)}
                    [< output_ ${index()} >]
                ),* )
            }

            #[derive($crate::reexports::serde::Serialize,Debug)]
            pub struct [< $name:camel Request >] {
                $($input_name: $input,)*
                $(
                    [< output_ ${index()} >]: $crate::handle::TypedHandle<dyn $output>,
                )*
            }

            impl<'de> $crate::reexports::octant_serde::DeserializeWith<'de> for [< $name:camel Request >] {
                fn deserialize_with<D:$crate::reexports::serde::Deserializer<'de>>(ctx: &$crate::reexports::octant_serde::TypeMap, d:D)->Result<Self,D::Error>{
                    todo!();
                }
            }

            #[cfg(side="client")]
            $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: $crate::proto::DownMessage);
            #[cfg(side="client")]
            impl $crate::proto::DownMessage for [< $name:camel Request >] {
                fn run(self:Box<Self>, runtime:&Arc<$crate::runtime::Runtime>) -> $crate::reexports::anyhow::Result<()>{
                    todo!();
                }
            }

            #[cfg(side="server")]
            $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: $crate::proto::DownMessage);
            #[cfg(side="server")]
            impl $crate::proto::DownMessage for [< $name:camel Request >] {}

            // #[cfg(side = "client")]
            // #[$crate::reexports::catalog::register($crate::DOWN_MESSAGE_HANDLER_REGISTRY)]
            // fn [<handle_ $name>]() -> $crate::DownMessageHandler<[< $name:camel Request >]> {
            //     |runtime: &::std::sync::Arc<$crate::Runtime>, req: [< $name:camel Request >]| {
            //         let runtime = runtime.clone();
            //         let result = [<impl_ $name>](&runtime $(, req.$input_name)*)?;
            //         $(
            //             runtime.add_new(req.[< output_ ${index()} >], ::std::sync::Arc::new(<dyn $output as $crate::FromHandle>::from_handle(req.[< output_ ${index()} >], result.${index()})));
            //         )*
            //         Ok(())
            //     }
            // }

            #[cfg(side="client")]
            fn [<impl_ $name>](
                $runtime: &::std::sync::Arc<$crate::runtime::Runtime>,
                $($input_name: $input,)*
            ) -> $crate::reexports::anyhow::Result<
                ($(
                    Arc<dyn $output>,
                )*)
            >{
                $($imp)*
            }
        }
    };
}