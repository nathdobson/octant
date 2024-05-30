#[macro_export]
macro_rules! define_sys_class {
    {
        class $class:ident;
        extends $parent:path;
        $(wasm $wasm:path;)?
        $(new_client $new_client_dummy:tt;)?
        $(new_server $new_server_dummy:tt;)?
        $(client_field $client_field:ident : $client_field_type:ty ;)*
        $(server_field $server_field:ident : $server_field_type:ty ;)*
        $( client_fn{$($client_fn:tt)*} )*
        $( server_fn{$($server_fn:tt)*} )*
    } => {
        $crate::reexports::paste::paste! {
            #[cfg(side = "client")]
            $crate::reexports::octant_object::define_class! {
                pub class $class extends $parent implements ::std::fmt::Debug{
                    $( field [< $class:snake >]: $wasm; )?
                    $(field $client_field : $client_field_type;)*
                    $($($client_fn)*)*
                }
            }

            #[cfg(any($(all() ${ignore($new_client_dummy)} )?))]
            #[cfg(side = "client")]
            impl $crate::PeerNew for [< $class Value >] {
                type Builder = ($($wasm)?);
                fn peer_new(
                    [< $class:snake >]:($(  $wasm )?)
                ) -> [< $class Value >]  {
                    [< $class Value >] {
                        parent: <<dyn $parent as $crate::reexports::octant_object::class::Class>::Value as $crate::PeerNew>::peer_new(
                            ( $( ${ignore($wasm)} ::std::clone::Clone::clone(&[< $class:snake >]).into() )? )
                        ),
                        $( ${ignore($wasm)} [< $class:snake >], )?
                        $($client_field : ::std::default::Default::default(), )*
                    }
                }
            }

            $(
                #[cfg(side="client")]
                impl $crate::peer::AsNative for dyn $class {
                    type Native = $wasm;
                    fn native(&self) -> &$wasm{
                        &self.[< $class:snake >]
                    }
                }
            )?

            #[cfg(side = "server")]
            $crate::reexports::octant_object::define_class! {
                pub class $class extends $parent {
                    $(field $server_field : $server_field_type; )*
                    $($($server_fn)*)*
                }
            }

            #[cfg(any($(all() ${ignore($new_server_dummy)} )?))]
            #[cfg(side = "server")]
            impl $crate::PeerNew for [< $class Value >] {
                type Builder = $crate::peer::PeerValue;
                fn peer_new(handle: $crate::peer::PeerValue) -> Self {
                    [< $class Value >] {
                        parent: <<dyn $parent as $crate::reexports::octant_object::class::Class>::Value as $crate::PeerNew>::peer_new(handle),
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

            impl<'de> $crate::reexports::octant_serde::DeserializeRcWith<'de> for dyn $class {
                fn deserialize_rc_with<
                    D: $crate::reexports::serde::Deserializer<'de>
                >(
                    ctx: &$crate::reexports::octant_serde::DeserializeContext,
                    d: D
                ) -> ::std::result::Result<$crate::reexports::octant_reffed::rc::Rc2<Self>, D::Error>{
                    $crate::deserialize_object_with(ctx, d)
                }
            }
        }
    };
}
