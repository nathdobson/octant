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
                $vf $field : $crate::field::Field<$type>
            ),*
        }
        impl $crate::ser::SerializeUpdate for $str {
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
            fn serialize_update<S: $crate::reexports::serde::Serializer, SP: $crate::serializer_proxy::SerializerProxy>(
                &self,
                forest: &mut $crate::forest::Forest,
                ser_forest: &mut $crate::ser::SerializeForest<SP>,
                s: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                use $crate::reexports::serde::ser::SerializeStruct;
                let mut s = s.serialize_struct("MyStruct", 4)?;
                $(
                    s.serialize_field(
                        ::std::stringify!($field),
                        &$crate::field::SerializeFieldAdapter::new(&self.$field, forest, ser_forest),
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
        impl<'de> $crate::de::DeserializeUpdate<'de> for $str{
            fn deserialize_snapshot<D: $crate::reexports::serde::Deserializer<'de>, DP: $crate::deserializer_proxy::DeserializerProxy>(
                forest: &mut $crate::de::DeserializeForest<DP>,
                d: D,
            ) -> ::std::result::Result<Self, D::Error> {
                use $crate::reexports::serde::de::DeserializeSeed;
                struct V<'a, DP: $crate::deserializer_proxy::DeserializerProxy> {
                    forest: &'a mut $crate::de::DeserializeForest<DP>,
                }
                impl<'a, 'de, DP: $crate::deserializer_proxy::DeserializerProxy> $crate::struct_visitor::StructVisitor<'de> for V<'a, DP> {
                    type Value = $str;

                    fn visit<A: $crate::struct_visitor::StructAccess<'de>>(self, mut a: A) -> ::std::result::Result<Self::Value, A::Error> {
                        ::std::result::Result::Ok($str {
                            $(
                                $field: a.next_seed($crate::de::DeserializeSnapshotSeed::new(self.forest))?,
                            )*
                        })
                    }
                }
                $crate::struct_visitor::StructSeed::new(
                    "MyStruct",
                    &["this", "field1", "field2", "field3"],
                    V { forest },
                )
                    .deserialize(d)
            }
            fn deserialize_update<D: $crate::reexports::serde::Deserializer<'de>, DP: $crate::deserializer_proxy::DeserializerProxy>(
                &mut self,
                forest: &mut $crate::de::DeserializeForest<DP>,
                d: D,
            ) -> ::std::result::Result<(), D::Error> {
                use $crate::reexports::serde::de::DeserializeSeed;
                struct V<'a, DP: $crate::deserializer_proxy::DeserializerProxy> {
                    forest: &'a mut $crate::de::DeserializeForest<DP>,
                    this: &'a mut $str,
                }
                impl<'a, 'de, DP: $crate::deserializer_proxy::DeserializerProxy> $crate::struct_visitor::StructVisitor<'de> for V<'a, DP> {
                    type Value = ();

                    fn visit<A: $crate::struct_visitor::StructAccess<'de>>(self, mut a: A) -> ::std::result::Result<Self::Value, A::Error> {
                        $(
                            a.next_seed($crate::field::DeserializeFieldSeed::new(
                                &mut self.this.$field,
                                self.forest,
                            ))?;
                        )*
                        ::std::result::Result::Ok(())
                    }
                }
                $crate::struct_visitor::StructSeed::new(
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
        impl MyStruct {
            $(
                $vf fn $field<'a>(self: $crate::tack::Tack<'a, Self>) -> $crate::tack::Tack<'a, $type> {
                    $crate::tack::Tack::new(&mut self.into_inner_unchecked().$field)
                }
            )*
        }
    };
}
//
// #[derive(Debug)]
// struct MyStruct {
//     this: Field<Weak<Tree<MyStruct>>>,
//     field1: Field<Prim<u8>>,
//     field2: Field<Weak<Tree<Prim<u8>>>>,
//     field3: Field<Arc<Tree<Prim<u8>>>>,
// }
//
// impl SerializeUpdate for MyStruct {
//     fn begin_stream(&mut self) {
//         self.this.begin_stream();
//         self.field1.begin_stream();
//         self.field2.begin_stream();
//         self.field3.begin_stream();
//     }
//
//     fn begin_update(&mut self) -> bool {
//         self.this.begin_update()
//             || self.field1.begin_update()
//             || self.field2.begin_update()
//             || self.field3.begin_update()
//     }
//
//     fn serialize_update<S: Serializer, SP: SerializerProxy>(
//         &self,
//         forest: &mut Forest,
//         ser_forest: &mut SerializeForest<SP>,
//         s: S,
//     ) -> Result<S::Ok, S::Error> {
//         let mut s = s.serialize_struct("MyStruct", 4)?;
//         s.serialize_field(
//             "this",
//             &self
//                 .this
//                 .modified()
//                 .then_some(SerializeUpdateAdapter::new(&self.this, forest, ser_forest)),
//         )?;
//         s.serialize_field(
//             "field1",
//             &self
//                 .field1
//                 .modified()
//                 .then_some(SerializeUpdateAdapter::new(
//                     &self.field1,
//                     forest,
//                     ser_forest,
//                 )),
//         )?;
//         s.serialize_field(
//             "field2",
//             &self
//                 .field2
//                 .modified()
//                 .then_some(SerializeUpdateAdapter::new(
//                     &self.field2,
//                     forest,
//                     ser_forest,
//                 )),
//         )?;
//         s.serialize_field(
//             "field3",
//             &self
//                 .field3
//                 .modified()
//                 .then_some(SerializeUpdateAdapter::new(
//                     &self.field3,
//                     forest,
//                     ser_forest,
//                 )),
//         )?;
//         Ok(s.end()?)
//     }
//
//     fn end_update(&mut self) {
//         self.this.end_update();
//         self.field1.end_update();
//         self.field2.end_update();
//         self.field3.end_update();
//     }
// }
//
// impl MyStruct {
//     fn this_mut<'a>(self: Tack<'a, Self>) -> Tack<'a, Weak<Tree<MyStruct>>> {
//         Tack::new(&mut self.into_inner_unchecked().this)
//     }
//     fn field1_mut<'a>(self: Tack<'a, Self>) -> Tack<'a, Prim<u8>> {
//         Tack::new(&mut self.into_inner_unchecked().field1)
//     }
//     fn field2_mut<'a>(self: Tack<'a, Self>) -> Tack<'a, Weak<Tree<Prim<u8>>>> {
//         Tack::new(&mut self.into_inner_unchecked().field2)
//     }
//     fn field3_mut<'a>(self: Tack<'a, Self>) -> Tack<'a, Arc<Tree<Prim<u8>>>> {
//         Tack::new(&mut self.into_inner_unchecked().field3)
//     }
// }
//
// impl<'de> DeserializeUpdate<'de> for MyStruct {
//     fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
//         forest: &mut DeserializeForest<DP>,
//         d: D,
//     ) -> Result<Self, D::Error> {
//         struct V<'a, DP: DeserializerProxy> {
//             forest: &'a mut DeserializeForest<DP>,
//         }
//         impl<'a, 'de, DP: DeserializerProxy> StructVisitor<'de> for V<'a, DP> {
//             type Value = MyStruct;
//
//             fn visit<A: StructAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
//                 Ok(MyStruct {
//                     this: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
//                     field1: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
//                     field2: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
//                     field3: a.next_seed(DeserializeSnapshotSeed::new(self.forest))?,
//                 })
//             }
//         }
//         StructSeed::new(
//             "MyStruct",
//             &["this", "field1", "field2", "field3"],
//             V { forest },
//         )
//             .deserialize(d)
//     }
//
//     fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
//         &mut self,
//         forest: &mut DeserializeForest<DP>,
//         d: D,
//     ) -> Result<(), D::Error> {
//         struct V<'a, DP: DeserializerProxy> {
//             forest: &'a mut DeserializeForest<DP>,
//             this: &'a mut MyStruct,
//         }
//         impl<'a, 'de, DP: DeserializerProxy> StructVisitor<'de> for V<'a, DP> {
//             type Value = ();
//
//             fn visit<A: StructAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
//                 a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(
//                     &mut self.this.this,
//                     self.forest,
//                 )))?;
//                 a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(
//                     &mut self.this.field1,
//                     self.forest,
//                 )))?;
//                 a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(
//                     &mut self.this.field2,
//                     self.forest,
//                 )))?;
//                 a.next_seed(OptionSeed::new(DeserializeUpdateSeed::new(
//                     &mut self.this.field3,
//                     self.forest,
//                 )))?;
//                 Ok(())
//             }
//         }
//         StructSeed::new(
//             "MyStruct",
//             &["this", "field1", "field2", "field3"],
//             V { forest, this: self },
//         )
//             .deserialize(d)
//     }
// }
