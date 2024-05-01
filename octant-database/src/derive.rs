#[macro_export]
macro_rules! database_struct {
    {
        $(#[$a:meta])*
        $v:vis struct $str:ident {
            $(
                $vf:vis $field:ident : $type: ty
            ),*
            $(,)?
        }
    } => {
        $(#[$a])*
        $v struct $str {
            $(
                $vf $field : $crate::value::field::Field<$type>
            ),*
        }
        impl $crate::ser::update::SerializeUpdate for $str {
            fn begin_stream(&mut self) {
                $(
                    self.$field.begin_stream();
                )*
            }
            fn begin_update(&mut self) -> bool {
                $(
                    self.$field.begin_update() ||
                )* false
            }
            fn serialize_update<S: $crate::reexports::serde::Serializer, SP: $crate::ser::proxy::SerializerProxy>(
                &self,
                forest: &mut $crate::forest::Forest,
                ser_forest: &mut $crate::ser::forest::SerializeForest<SP>,
                s: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                use $crate::reexports::serde::ser::SerializeStruct;
                let mut s = s.serialize_struct(stringify!($str), 4)?;
                $(
                    s.serialize_field(
                        stringify!($field),
                        &$crate::value::field::SerializeFieldAdapter::new(&self.$field, forest, ser_forest),
                    )?;
                )*
                s.end()
            }
            fn end_update(&mut self) {
                $(
                    self.$field.end_update();
                )*
            }
        }
        impl<'de> $crate::de::update::DeserializeUpdate<'de> for $str{
            fn deserialize_snapshot<D: $crate::reexports::serde::Deserializer<'de>, DP: $crate::de::proxy::DeserializerProxy>(
                forest: &mut $crate::de::forest::DeserializeForest<DP>,
                d: D,
            ) -> ::std::result::Result<Self, D::Error> {
                use $crate::reexports::serde::de::DeserializeSeed;
                struct V<'a, DP: $crate::de::proxy::DeserializerProxy> {
                    forest: &'a mut $crate::de::forest::DeserializeForest<DP>,
                }
                impl<'a, 'de, DP: $crate::de::proxy::DeserializerProxy> $crate::de::seed::struct_seed::StructVisitor<'de> for V<'a, DP> {
                    type Value = $str;

                    fn visit<A: $crate::de::seed::struct_seed::StructAccess<'de>>(self, mut a: A) -> ::std::result::Result<Self::Value, A::Error> {
                        ::std::result::Result::Ok($str {
                            $(
                                $field: a.next_seed($crate::de::update::DeserializeSnapshotSeed::new(self.forest))?,
                            )*
                        })
                    }
                }
                $crate::de::seed::struct_seed::StructSeed::new(
                    stringify!($str),
                    &[$(stringify!($field),)*],
                    V { forest },
                )
                    .deserialize(d)
            }
            fn deserialize_update<D: $crate::reexports::serde::Deserializer<'de>, DP: $crate::de::proxy::DeserializerProxy>(
                &mut self,
                forest: &mut $crate::de::forest::DeserializeForest<DP>,
                d: D,
            ) -> ::std::result::Result<(), D::Error> {
                use $crate::reexports::serde::de::DeserializeSeed;
                struct V<'a, DP: $crate::de::proxy::DeserializerProxy> {
                    forest: &'a mut $crate::de::forest::DeserializeForest<DP>,
                    this: &'a mut $str,
                }
                impl<'a, 'de, DP: $crate::de::proxy::DeserializerProxy> $crate::de::seed::struct_seed::StructVisitor<'de> for V<'a, DP> {
                    type Value = ();

                    fn visit<A: $crate::de::seed::struct_seed::StructAccess<'de>>(self, mut a: A) -> ::std::result::Result<Self::Value, A::Error> {
                        $(
                            a.next_seed($crate::value::field::DeserializeFieldSeed::new(
                                &mut self.this.$field,
                                self.forest,
                            ))?;
                        )*
                        ::std::result::Result::Ok(())
                    }
                }
                $crate::de::seed::struct_seed::StructSeed::new(
                    ::std::stringify!($str),
                    &[
                        $(
                            ::std::stringify!($field),
                        )*
                    ],
                    V { forest, this: self },
                )
                    .deserialize(d)
            }
        }
        impl $str {
            pub fn new($($field: $type),*) -> Self {
                $str {
                    $($field: $crate::value::field::Field::new($field)),*
                }
            }
            $(
                $vf fn $field<'a>(self: $crate::tack::Tack<'a, Self>) -> $crate::tack::Tack<'a, $type> {
                    $crate::tack::Tack::new(&mut self.into_inner_unchecked().$field)
                }
            )*
        }
    };
}
