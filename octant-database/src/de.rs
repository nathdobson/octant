use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use serde::{
    de::{DeserializeSeed, Error},
    Deserialize, Deserializer,
};

use crate::{
    tree::{Tree, TreeId},
    util::{
        arc_or_empty::ArcOrEmpty,
        deserialize_pair::DeserializePair, map_seed::MapSeed,
    },
};

pub struct DeserializeForest {
    pub entries: HashMap<TreeId, ArcOrEmpty<Tree>>,
}

pub trait DeserializeUpdate<'de>: Sized {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        forest: &mut DeserializeForest,
        d: D,
    ) -> Result<Self, D::Error>;
    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        forest: &mut DeserializeForest,
        d: D,
    ) -> Result<(), D::Error>;
}

impl DeserializeForest {
    pub fn new() -> Self {
        DeserializeForest {
            entries: HashMap::new(),
        }
    }
    pub fn deserialize_snapshot<'de, D: Deserializer<'de>>(
        &mut self,
        d: D,
    ) -> Result<Arc<Tree>, D::Error> {
        Arc::<Tree>::deserialize_snapshot(self, d)
    }
    pub fn deserialize_update<'de, D: Deserializer<'de>>(&mut self, d: D) -> Result<(), D::Error> {
        return MapSeed::new(LogMap(self)).deserialize(d);
        struct LogMap<'a>(&'a mut DeserializeForest);
        impl<'a, 't, 'de> DeserializePair<'de> for LogMap<'a> {
            type First = TreeId;
            type Second = ();

            fn deserialize_first<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::First, D::Error> {
                TreeId::deserialize(d)
            }

            fn deserialize_second<D: Deserializer<'de>>(
                &mut self,
                first: Self::First,
                d: D,
            ) -> Result<Self::Second, D::Error> {
                if let Some(row) = self.0.entries.get(&first) {
                    let row = match row {
                        ArcOrEmpty::Arc(x) => x.clone(),
                        ArcOrEmpty::Empty(_) => {
                            return Err(D::Error::custom(format_args!(
                                "Received update for uninitialized row"
                            )))
                        }
                    };
                    row.try_write().unwrap().deserialize_update(self.0, d)?;
                } else {
                    return Err(D::Error::custom("Received update for missing row"));
                }
                Ok(())
            }
        }
    }
}

pub struct DeserializeUpdateSeed<'a, T>(&'a mut T, &'a mut DeserializeForest);

impl<'a, T> DeserializeUpdateSeed<'a, T> {
    pub fn new(x: &'a mut T, table: &'a mut DeserializeForest) -> Self {
        DeserializeUpdateSeed(x, table)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>> DeserializeSeed<'de> for DeserializeUpdateSeed<'a, T> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_update(self.1, deserializer)
    }
}

pub struct DeserializeSnapshotSeed<'a, T>(&'a mut DeserializeForest, PhantomData<T>);

impl<'a, T> DeserializeSnapshotSeed<'a, T> {
    pub fn new(table: &'a mut DeserializeForest) -> Self {
        DeserializeSnapshotSeed(table, PhantomData)
    }
}

impl<'a, 'de, T: DeserializeUpdate<'de>> DeserializeSeed<'de> for DeserializeSnapshotSeed<'a, T> {
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_snapshot(self.0, deserializer)
    }
}
