use crate::de::forest::DeserializeForest;
use crate::de::proxy::DeserializerProxy;
use crate::de::update::DeserializeUpdate;

pub(crate) trait DeserializeTree<DP: DeserializerProxy> {
    fn deserialize_update_by_proxy<'up, 'de>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: DP::DeserializerValue<'up, 'de>,
    ) -> Result<(), DP::Error>;
}

impl<T, DP> DeserializeTree<DP> for T
where
    for<'de> T: DeserializeUpdate<'de>,
    DP: DeserializerProxy,
{
    fn deserialize_update_by_proxy<'up, 'de>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: DP::DeserializerValue<'up, 'de>,
    ) -> Result<(), DP::Error> {
        self.deserialize_update(forest, d)
    }
}

fn is_object_safe<T: DeserializeTree<DP>, DP: DeserializerProxy>(
    x: &T,
) -> &dyn DeserializeTree<DP> {
    x
}

