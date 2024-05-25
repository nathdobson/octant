#[macro_export]
macro_rules! define_sys_rpc {
    {
        $vis:vis fn $name:ident($runtime:ident $(, $input_name:ident: $input:ty)*) -> $output:ty { $($imp:tt)* }
    } => {
        $crate::reexports::paste::paste! {
            #[cfg(side = "server")]
            $vis fn $name(
                runtime: &::std::sync::Arc<$crate::runtime::Runtime>
                $(, $input_name: $input)*
            ) -> $output {
                let (output, down) = <$output as $crate::immediate_return::ImmediateReturn>::immediate_new(runtime);
                runtime.send(Box::<[< $name:camel Request >]>::new([< $name:camel Request >] {
                    $($input_name,)*
                    down
                }));
                output
            }

            #[derive($crate::reexports::serde::Serialize,Debug,$crate::reexports::octant_serde::DeserializeWith)]
            pub struct [< $name:camel Request >] {
                $($input_name: $input,)*
                down: <$output as $crate::immediate_return::ImmediateReturn>::Down
            }

            #[cfg(side="client")]
            $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: $crate::proto::DownMessage);
            #[cfg(side="client")]
            impl $crate::proto::DownMessage for [< $name:camel Request >] {
                fn run(self:Box<Self>, runtime:&::std::sync::Arc<$crate::runtime::Runtime>) -> $crate::reexports::anyhow::Result<()>{
                    let output=[<impl_ $name>](runtime $(, self.$input_name)*)?;
                    $crate::immediate_return::ImmediateReturn::immediate_return(output, runtime, self.down);
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
            ) -> $crate::reexports::anyhow::Result<$output>
            {
                $($imp)*
            }
        }
    };
}
