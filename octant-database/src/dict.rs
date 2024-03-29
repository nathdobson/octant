use crate::arc::ArcOrWeak;
use crate::de::{
    DeserializeContext, DeserializeSnapshotAdapter, DeserializeUpdate, DeserializeUpdateAdapter,
};
use crate::map_combinator::{DeserializeEntry, MapCombinator};
use crate::ser::{SerializeUpdate, SerializeUpdateAdapter};
use crate::RowTableState;
use serde::de::{DeserializeSeed, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serializer};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashSet};
use std::fmt::{Debug, Formatter};
use crate::row::Row;

pub struct Dict {
    entries: BTreeMap<String, ArcOrWeak<Row>>,
    modified: Option<HashSet<String>>,
}

impl Dict {
    pub fn new() -> Self {
        Dict {
            entries: Default::default(),
            modified: None,
        }
    }
    pub fn insert(&mut self, key: String, value: ArcOrWeak<Row>) {
        self.entries.insert(key.clone(), value);
        if let Some(modified) = &mut self.modified {
            modified.insert(key);
        }
    }
    pub fn remove(&mut self, key: &str) {
        self.entries.remove(key);
        if let Some(modified) = &mut self.modified {
            modified.insert(key.to_string());
        }
    }
    pub fn get(&self, key: &str) -> Option<&ArcOrWeak<Row>> {
        self.entries.get(key)
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut ArcOrWeak<Row>> {
        if let Some(modified) = &mut self.modified {
            modified.insert(key.to_string());
        }
        self.entries.get_mut(key)
    }
}
impl<'de> DeserializeUpdate<'de> for Dict {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'a> {
            table: DeserializeContext<'a>,
        }
        impl<'a, 'de> DeserializeEntry<'de> for V<'a> {
            type Key = String;
            type Value = (String, ArcOrWeak<Row>);

            fn deserialize_key<D: Deserializer<'de>>(
                &mut self,
                d: D,
            ) -> Result<Self::Key, D::Error> {
                String::deserialize(d)
            }

            fn deserialize_value<D: Deserializer<'de>>(
                &mut self,
                key: Self::Key,
                value: D,
            ) -> Result<Self::Value, D::Error> {
                Ok((
                    key,
                    ArcOrWeak::deserialize_snapshot(self.table.reborrow(), value)?,
                ))
            }
        }
        Ok(Dict {
            entries: MapCombinator::new(V { table }).deserialize(d)?,
            modified: None,
        })
    }

    fn deserialize_update<'a, D: Deserializer<'de>>(
        &mut self,
        table: DeserializeContext,
        d: D,
    ) -> Result<(), D::Error> {
        struct M<'a> {
            dict: &'a mut Dict,
            table: DeserializeContext<'a>,
        }
        impl<'a, 'de> Visitor<'de> for M<'a> {
            type Value = ();

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "map")
            }
            fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<String>()? {
                    match self.dict.entries.entry(key) {
                        Entry::Vacant(x) => {
                            x.insert(map.next_value_seed(DeserializeSnapshotAdapter::new(
                                self.table.reborrow(),
                            ))?);
                        }
                        Entry::Occupied(mut x) => map.next_value_seed(
                            DeserializeUpdateAdapter::new(x.get_mut(), self.table.reborrow()),
                        )?,
                    }
                }
                Ok(())
            }
        }
        d.deserialize_map(M { dict: self, table })
    }
}

impl Debug for Dict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_map();
        for (k, v) in self.entries.iter() {
            m.entry(k, v);
        }
        m.finish()
    }
}

impl SerializeUpdate for Dict {
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

    fn serialize_update<S: Serializer>(
        &self,
        state: &RowTableState,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(modified) = &self.modified {
            let mut s = s.serialize_map(Some(modified.len()))?;
            for k in modified.iter() {
                let v = self
                    .entries
                    .get(k)
                    .map(|v| SerializeUpdateAdapter::new(v, state));
                s.serialize_entry(k, &v)?;
            }
            s.end()
        } else {
            let mut s = s.serialize_map(Some(self.entries.len()))?;
            for (k, v) in self.entries.iter() {
                s.serialize_entry(k, &Some(SerializeUpdateAdapter::new(v, state)))?;
            }
            s.end()
        }
    }

    fn end_update(&mut self) {
        if let Some(modified) = &mut self.modified {
            modified.clear();
        }
    }
}
