use std::{
    collections::{btree_map::Entry, BTreeMap, HashSet},
    fmt::{Debug, Formatter},
    hash::Hash,
    marker::PhantomData,
};

use serde::{
    de::{DeserializeSeed, MapAccess, Visitor},
    Deserialize,
    Deserializer, ser::SerializeMap, Serialize, Serializer,
};

use crate::{
    de::{DeserializeForest, DeserializeSnapshotSeed, DeserializeUpdate, DeserializeUpdateSeed},
    forest::ForestState,
    ser::{SerializeForest, SerializeUpdate, SerializeUpdateAdapter}
    ,
    util::{
        deserialize_pair::DeserializePair,
        deserializer_proxy::DeserializerProxy, map_seed::MapSeed,
        serializer_proxy::SerializerProxy,
    },
};

pub struct Dict<K, V> {
    entries: BTreeMap<K, V>,
    modified: Option<HashSet<K>>,
}

impl<K: Eq + Ord + Hash + Clone, V> Dict<K, V> {
    pub fn new() -> Self {
        Dict {
            entries: Default::default(),
            modified: None,
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key.clone(), value);
        if let Some(modified) = &mut self.modified {
            modified.insert(key);
        }
    }
    pub fn remove(&mut self, key: &K) {
        self.entries.remove(key);
        if let Some(modified) = &mut self.modified {
            modified.insert(key.clone());
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(modified) = &mut self.modified {
            modified.insert(key.clone());
        }
        self.entries.get_mut(key)
    }
}

impl<'de, K: Ord + Deserialize<'de>, V: DeserializeUpdate<'de>> DeserializeUpdate<'de>
    for Dict<K, V>
{
    fn deserialize_snapshot<D: Deserializer<'de>, DP: DeserializerProxy>(
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<Self, D::Error> {
        struct Vis<'a, K, V, DP: DeserializerProxy> {
            forest: &'a mut DeserializeForest<DP>,
            phantom: PhantomData<(K, V)>,
        }
        impl<'a, 'de, K: Deserialize<'de>, V: DeserializeUpdate<'de>, DP: DeserializerProxy>
            DeserializePair<'de> for Vis<'a, K, V, DP>
        {
            type First = K;
            type Second = (K, V);

            fn deserialize_first<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::First, D::Error> {
                K::deserialize(d)
            }

            fn deserialize_second<D: Deserializer<'de>>(
                &mut self,
                key: Self::First,
                value: D,
            ) -> Result<Self::Second, D::Error> {
                Ok((key, V::deserialize_snapshot(self.forest, value)?))
            }
        }
        Ok(Dict {
            entries: MapSeed::new(Vis {
                forest,
                phantom: PhantomData,
            })
            .deserialize(d)?,
            modified: None,
        })
    }

    fn deserialize_update<'a, D: Deserializer<'de>, DP: DeserializerProxy>(
        &mut self,
        forest: &mut DeserializeForest<DP>,
        d: D,
    ) -> Result<(), D::Error> {
        struct M<'a, K, V, DP: DeserializerProxy> {
            dict: &'a mut Dict<K, V>,
            forest: &'a mut DeserializeForest<DP>,
        }
        impl<
                'a,
                'de,
                K: Ord + Deserialize<'de>,
                V: DeserializeUpdate<'de>,
                DP: DeserializerProxy,
            > Visitor<'de> for M<'a, K, V, DP>
        {
            type Value = ();

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "map")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<K>()? {
                    match self.dict.entries.entry(key) {
                        Entry::Vacant(x) => {
                            x.insert(
                                map.next_value_seed(DeserializeSnapshotSeed::new(self.forest))?,
                            );
                        }
                        Entry::Occupied(mut x) => map.next_value_seed(
                            DeserializeUpdateSeed::new(x.get_mut(), self.forest),
                        )?,
                    }
                }
                Ok(())
            }
        }
        d.deserialize_map(M { dict: self, forest })
    }
}

impl<K: Debug, V: Debug> Debug for Dict<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_map();
        for (k, v) in self.entries.iter() {
            m.entry(k, v);
            // m.key(k);
            // m.value_with(|f| {
            //     match v {
            //         ArcOrWeak::Arc(x) => {
            //             Debug::fmt(x, f)?;
            //         }
            //         ArcOrWeak::Weak(x) => {
            //             f.debug_tuple("Weak")
            //                 .field_with(|f| {
            //                     if let Some(x) = x.upgrade() {
            //                         x.fmt_weak(f)?;
            //                     }
            //                     Ok(())
            //                 })
            //                 .finish()?;
            //         }
            //     }
            //     Ok(())
            // });
        }
        m.finish()
    }
}

impl<K: Ord + Serialize, V: SerializeUpdate> SerializeUpdate for Dict<K, V> {
    fn begin_stream(&mut self) {
        self.modified = None;
    }

    fn begin_update(&mut self) -> bool {
        if let Some(modified) = &mut self.modified {
            !modified.is_empty()
        } else {
            true
        }
    }

    fn serialize_update<S: Serializer, SP: SerializerProxy>(
        &self,
        forest: &mut ForestState,
        ser_forest: &mut SerializeForest<SP>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(modified) = &self.modified {
            let mut s = s.serialize_map(Some(modified.len()))?;
            for k in modified.iter() {
                let v = self
                    .entries
                    .get(k)
                    .map(|v| SerializeUpdateAdapter::new(v, forest, ser_forest));
                s.serialize_entry(k, &v)?;
            }
            s.end()
        } else {
            let mut s = s.serialize_map(Some(self.entries.len()))?;
            for (k, v) in self.entries.iter() {
                s.serialize_entry(k, &Some(SerializeUpdateAdapter::new(v, forest, ser_forest)))?;
            }
            s.end()
        }
    }

    fn end_update(&mut self) {
        if let Some(modified) = &mut self.modified {
            for x in modified.iter() {
                if let Some(x) = self.entries.get_mut(x) {
                    x.end_update();
                }
            }
            modified.clear();
        } else {
            self.modified = Some(HashSet::new());
        }
    }
}
