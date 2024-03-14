use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use crate::seed::SerializeUpdate;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::stream_deserialize::StreamDeserialize;

use crate::stream_serialize::StreamSerialize;
use crate::tack::Tack;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
enum EntryState {
    Inserted,
    Modified,
    Deleted,
}

pub struct StreamHashMap<K, V> {
    value: HashMap<K, V>,
    modified: HashMap<K, EntryState>,
}

impl<K: Hash + Eq + Clone, V: StreamSerialize> StreamHashMap<K, V> {
    pub fn new() -> Self {
        StreamHashMap {
            value: HashMap::new(),
            modified: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.modified.insert(key.clone(), EntryState::Inserted);
        self.value.insert(key, value)
    }
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some((key, value)) = self.value.remove_entry(key) {
            self.modified.insert(key, EntryState::Deleted);
            Some(value)
        } else {
            None
        }
    }
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.value.get(key)
    }
    pub fn get_mut<'a, Q>(self: Tack<'a, Self>, query: &Q) -> Option<Tack<'a, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut this = self.into_inner_unchecked();
        let (key, value) = this.value.get_key_value(query)?;
        this.modified
            .raw_entry_mut()
            .from_key::<Q>(query)
            .or_insert_with(|| (key.clone(), EntryState::Modified));
        Some(Tack::new(this.value.get_mut(query).unwrap()))
    }
}

impl<K: Serialize, V: StreamSerialize> Serialize for StreamHashMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<K: Hash + Eq + Clone + Serialize, V: StreamSerialize> StreamSerialize for StreamHashMap<K, V> {
    fn build_baseline(&mut self) {
        self.modified.clear();
    }

    fn build_target(&mut self) -> bool {
        self.modified.retain(|key, state| match *state {
            EntryState::Inserted => {
                self.value.get_mut(key).unwrap().build_baseline();
                return true;
            }
            EntryState::Modified => self.value.get_mut(key).unwrap().build_target(),
            EntryState::Deleted => true,
        });
        !self.modified.is_empty()
    }

    fn serialize_update<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut serializer = serializer.serialize_map(Some(self.modified.len()))?;
        for (key, state) in &self.modified {
            serializer.serialize_key(key)?;
            match state {
                EntryState::Inserted => {
                    serializer
                        .serialize_value(&(EntryState::Inserted, self.value.get(key).unwrap()))?;
                }
                EntryState::Modified => {
                    serializer.serialize_value(&(
                        EntryState::Modified,
                        SerializeUpdate::new(self.value.get(key).unwrap()),
                    ))?;
                }
                EntryState::Deleted => {
                    serializer.serialize_value(&(EntryState::Deleted, ()))?;
                }
            }
        }
        serializer.end()
    }
}

impl<'de,K: Hash + Eq + Clone + Serialize, V: StreamDeserialize<'de>> Deserialize<'de> for StreamHashMap<K,V>{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}
impl<'de,K: Hash + Eq + Clone + Serialize, V: StreamDeserialize<'de>> StreamDeserialize<'de> for StreamHashMap<K,V>{
    fn deserialize_stream<D: Deserializer<'de>>(&mut self, d: D) -> Result<(), D::Error> {
        todo!()
    }
}
