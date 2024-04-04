// use crate::{
//     forest::ForestState,
//     ser::{SerializeForest, SerializeUpdate, SerializeUpdateAdapter},
//     util::serializer_proxy::SerializerProxy,
// };
// use serde::Serializer;
//
// impl<T: SerializeUpdate> SerializeUpdate for Option<T> {
//     fn begin_stream(&mut self) {
//         if let Some(x) = self {
//             x.begin_stream();
//         }
//     }
//
//     fn begin_update(&mut self) -> bool {
//         if let Some(x) = self {
//             x.begin_update()
//         } else {
//             false
//         }
//     }
//
//     fn serialize_update<S: Serializer, SP: SerializerProxy>(
//         &self,
//         forest: &mut ForestState,
//         ser_forest: &mut SerializeForest<SP>,
//         s: S,
//     ) -> Result<S::Ok, S::Error> {
//         if let Some(x) = self {
//             s.serialize_some(&SerializeUpdateAdapter::new(x, forest, ser_forest))
//         } else {
//             s.serialize_none()
//         }
//     }
//
//     fn end_update(&mut self) {
//         if let Some(x) = self {
//             x.end_update()
//         }
//     }
// }
