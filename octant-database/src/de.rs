use std::{any::Any, collections::HashMap, marker::PhantomData, sync::Arc};

use serde::{
    de::{DeserializeSeed, Error},
    Deserializer,
};

use crate::{
    tree::{Tree, TreeId},
    util::{deserializer_proxy::DeserializerProxy, unique_arc::UniqueArc},
};

pub(crate) trait DeserializeTree<DP: DeserializerProxy> {
    fn deserialize_update_by_proxy<'up, 'de>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: DP::DeserializerValue<'up, 'de>,
    ) -> Result<(), DP::Error>;
}

fn is_object_safe<T: DeserializeTree<DP>, DP: DeserializerProxy>(
    x: &T,
) -> &dyn DeserializeTree<DP> {
    x
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

pub struct DeserializeForest<DP: DeserializerProxy> {
    pub(crate) updaters: HashMap<TreeId, Arc<Tree<dyn DeserializeTree<DP>>>>,
    pub(crate) holes: HashMap<TreeId, UniqueArc<dyn Any + Send + Sync + 'static>>,
    pub(crate) values: HashMap<TreeId, Arc<dyn Any + Send + Sync + 'static>>,
}

pub trait DeserializeUpdate<'de>: Sized {
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error>;
    fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error>;
}

impl<DP: DeserializerProxy> DeserializeForest<DP> {
    pub fn new() -> Self {
        DeserializeForest {
            updaters: HashMap::new(),
            holes: HashMap::new(),
            values: HashMap::new(),
        }
    }
    pub fn deserialize_snapshot<'up, 'de: 'up, T: DeserializeUpdate<'de>>(
        &mut self,
        d: DP::DeserializerValue<'up, 'de>,
    ) -> Result<T, DP::Error> {
        T::deserialize_snapshot(self, d)
    }
    pub fn deserialize_update<'up, 'de>(
        &mut self,
        id: TreeId,
        d: DP::DeserializerValue<'up, 'de>,
    ) -> Result<(), DP::Error> {
        let updater = self
            .updaters
            .get(&id)
            .ok_or_else(|| DP::Error::custom("received update for unknown row"))?
            .clone();
        updater
            .try_write()
            .unwrap()
            .deserialize_update_by_proxy(self, d)?;
        Ok(())
    }
}

pub struct DeserializeUpdateSeed<'a, T, DP: DeserializerProxy>(
    &'a mut T,
    &'a mut DeserializeForest<DP>,
);

impl<'a, T, DP: DeserializerProxy> DeserializeUpdateSeed<'a, T, DP> {
    pub fn new(x: &'a mut T, table: &'a mut DeserializeForest<DP>) -> Self {
        DeserializeUpdateSeed(x, table)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>, DP: DeserializerProxy> DeserializeSeed<'de>
    for DeserializeUpdateSeed<'a, T, DP>
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_update(self.1, deserializer)
    }
}

pub struct DeserializeSnapshotSeed<'a, T, DP: DeserializerProxy>(
    &'a mut DeserializeForest<DP>,
    PhantomData<T>,
);

impl<'a, T, DP: DeserializerProxy> DeserializeSnapshotSeed<'a, T, DP> {
    pub fn new(table: &'a mut DeserializeForest<DP>) -> Self {
        DeserializeSnapshotSeed(table, PhantomData)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>, DP: DeserializerProxy> DeserializeSeed<'de>
    for DeserializeSnapshotSeed<'a, T, DP>
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_snapshot(self.0, deserializer)
    }
}

// impl<'de, T: 'static + Sync + Send> DeserializeUpdate<'de> for ArcOrWeak<Tree<T>> {
//     fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
//         table: &mut DeserializeForest<DP>,
//         d: D,
//     ) -> Result<Self, D::Error> {
//         #[derive(Deserialize)]
//         enum Tag {
//             Arc,
//             Weak,
//         }
//         struct V<'a, T: 'static + Sync + Send, DP: DeserializerProxy> {
//             table: &'a mut DeserializeForest<DP>,
//             phantom: PhantomData<T>,
//         }
//         impl<'a, 'de, T: 'static + Sync + Send, DP: DeserializerProxy> Visitor<'de> for V<'a, T, DP> {
//             type Value = ArcOrWeak<Tree<T>>;
//
//             fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
//             where
//                 A: EnumAccess<'de>,
//             {
//                 let (tag, access) = data.variant::<Tag>()?;
//                 match tag {
//                     Tag::Arc => Ok(ArcOrWeak::Arc(
//                         access.newtype_variant_seed(DeserializeSnapshotSeed::new(self.table))?,
//                     )),
//                     Tag::Weak => Ok(ArcOrWeak::Weak(
//                         access.newtype_variant_seed(DeserializeSnapshotSeed::new(self.table))?,
//                     )),
//                 }
//             }
//             fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
//                 write!(f, "enum")
//             }
//         }
//         d.deserialize_enum(
//             "ArcOrWeak",
//             &["Weak", "Arc"],
//             V {
//                 table,
//                 phantom: PhantomData,
//             },
//         )
//     }
//
//     fn deserialize_update<D: Deserializer<'de>, DP: DeserializerProxy>(
//         &mut self,
//         forest: &mut DeserializeForest<DP>,
//         d: D,
//     ) -> Result<(), D::Error> {
//         *self = Self::deserialize_snapshot(forest, d)?;
//         Ok(())
//     }
// }
