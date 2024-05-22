#[macro_export]
macro_rules! define_sys_rpc {
    {
        $vis:vis fn $name:ident($runtime:ident $(, $input_name:ident: $input:ty)*) -> ( $( $output:ident, )* ) { $($imp:tt)* }
    } => {
        $crate::reexports::paste::paste! {
            #[cfg(side = "server")]
            $vis fn $name(
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

            $crate::reexports::octant_serde::derive_deserialize_with_for_struct!{
                struct [< $name:camel Request >] {
                    $($input_name: $input,)*
                    $(
                        [< output_ ${index()} >]: $crate::handle::TypedHandle<dyn $output>,
                    )*
                }
            }

            #[cfg(side="client")]
            $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: $crate::proto::DownMessage);
            #[cfg(side="client")]
            impl $crate::proto::DownMessage for [< $name:camel Request >] {
                fn run(self:Box<Self>, runtime:&::std::sync::Arc<$crate::runtime::Runtime>) -> $crate::reexports::anyhow::Result<()>{
                    let output=[<impl_ $name>](runtime $(, self.$input_name)*)?;
                    $(
                        ${ignore($output)}
                        runtime.add(self.[< output_ ${index()} >], output.${index()});
                    )*
                    Ok(())
                }
            }

            #[cfg(side="server")]
            $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: $crate::proto::DownMessage);
            #[cfg(side="server")]
            impl $crate::proto::DownMessage for [< $name:camel Request >] {}

            #[cfg(side="client")]
            fn [<impl_ $name>](
                $runtime: &::std::sync::Arc<$crate::runtime::Runtime>,
                $($input_name: $input,)*
            ) -> $crate::reexports::anyhow::Result<
                ($(
                    ::std::sync::Arc<dyn $output>,
                )*)
            >{
                $($imp)*
            }
        }
    };
}
