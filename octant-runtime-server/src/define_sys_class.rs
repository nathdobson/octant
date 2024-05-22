#[macro_export]
macro_rules! define_sys_class {
    {
        class $class:ident;
        extends $parent:path;
        wasm $wasm:path;
        $(new_client $new_client_dummy:tt;)?
        $(new_server $new_server_dummy:tt;)?
        $(client_field $client_field:ident : $client_field_type:ty ;)*
        $(server_field $server_field:ident : $server_field_type:ty ;)*
    } => {
        $crate::reexports::paste::paste! {
            #[cfg(side = "client")]
            $crate::reexports::octant_object::define_class! {
                pub class $class extends $parent implements ::std::fmt::Debug{
                    [< $class:snake >]: $wasm,
                    $($client_field : $client_field_type, )*
                }
            }

            #[cfg(any($(all() ${ignore($new_client_dummy)} )?))]
            #[cfg(side = "client")]
            impl [< $class Value >] {
                pub fn new([< $class:snake >]: $wasm) -> [< $class Value >]  {
                    [< $class Value >] {
                        parent: <dyn $parent as $crate::reexports::octant_object::class::Class>::Value::new(::std::clone::Clone::clone(&[< $class:snake >]).into()),
                        [< $class:snake >],
                        $($client_field : ::std::default::Default::default(), )*
                    }
                }
            }

            #[cfg(side="client")]
            impl [< $class Value >] {
                pub fn native(&self) -> &$wasm{
                    &self.[< $class:snake >]
                }
            }

            #[cfg(side = "server")]
            $crate::reexports::octant_object::define_class! {
                pub class $class extends $parent {
                    $($server_field : $server_field_type, )*
                }
            }
            #[cfg(any($(all() ${ignore($new_server_dummy)} )?))]
            #[cfg(side = "server")]
            impl [< $class Value >] {
                pub fn new(handle: $crate::peer::PeerValue) -> Self {
                    [< $class Value >] {
                        parent: <dyn $parent as $crate::reexports::octant_object::class::Class>::Value::new(handle),
                        $($server_field : ::std::default::Default::default(), )*
                    }
                }
            }

            impl $crate::reexports::serde::Serialize for dyn $class {
                fn serialize<S>(&self, s: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: $crate::reexports::serde::Serializer,
                {
                    return self.raw_handle().serialize(s);
                }
            }

            impl<'de> $crate::reexports::octant_serde::DeserializeArcWith<'de> for dyn $class {
                fn deserialize_arc_with<
                    D: $crate::reexports::serde::Deserializer<'de>
                >(
                    ctx: &$crate::reexports::octant_serde::TypeMap,
                    d: D
                ) -> ::std::result::Result<::std::sync::Arc<Self>, D::Error>{
                    $crate::deserialize_object_with(ctx, d)
                }
            }
        }
    };
}