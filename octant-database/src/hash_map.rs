use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use serde::{Serialize, Serializer};

use crate::stream_serialize::StreamSerialize;

enum EntryState {
    New,
    Deleted,
    Modified,
}

pub struct StreamHashMap<K, V> {
    table: HashMap<K, V>,
    modified: HashMap<K, EntryState>,
}

impl<K: Hash + Eq + Clone, V: StreamSerialize> StreamHashMap<K, V> {
    pub fn new() -> Self {
        todo!();
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!();
    }
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        todo!()
    }
}

impl<K: Serialize, V: StreamSerialize> Serialize for StreamHashMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<K: Hash + Eq + Clone + Serialize, V: StreamSerialize> StreamSerialize for StreamHashMap<K, V> {
    fn build_baseline(&mut self) {
        todo!()
    }

    fn build_target(&mut self) -> bool {
        todo!()
    }

    fn serialize_update<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        todo!()
    }
}
