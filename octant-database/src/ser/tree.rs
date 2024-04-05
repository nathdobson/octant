use serde::Serializer;
use crate::forest::Forest;
use crate::ser::forest::SerializeForest;
use crate::ser::proxy::SerializerProxy;
use crate::ser::update::SerializeUpdate;

pub(crate) trait SerializeTree<SP: SerializerProxy> {
    fn tree_begin_stream(&mut self);
    fn tree_begin_update(&mut self) -> bool;
    fn tree_serialize_update<'up>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: SP::SerializerValue<'up>,
    ) -> Result<<SP::SerializerValue<'up> as Serializer>::Ok, SP::Error>;
    fn tree_end_update(&mut self);
}

impl<SP: SerializerProxy, T: SerializeUpdate> SerializeTree<SP> for T {
    fn tree_begin_stream(&mut self) {
        self.begin_stream();
    }

    fn tree_begin_update(&mut self) -> bool {
        self.begin_update()
    }

    fn tree_serialize_update<'up>(
        &self,
        forest: &mut Forest,
        ser_forest: &mut SerializeForest<SP>,
        s: SP::SerializerValue<'up>,
    ) -> Result<<SP::SerializerValue<'up> as Serializer>::Ok, SP::Error> {
        self.serialize_update(forest, ser_forest, s)
    }

    fn tree_end_update(&mut self) {
        self.end_update()
    }
}
