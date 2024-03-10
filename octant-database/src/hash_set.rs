use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;

use crate::stream_deserialize::StreamDeserialize;
use crate::stream_serialize::StreamSerialize;
use crate::tack::Tack;

pub struct StreamHashSet<K> {
    value: HashSet<K>,
    modified: HashSet<K>,
}

#[derive(Serialize, Deserialize)]
enum StreamHashEntry<K> {
    Insert(K),
    Remove(K),
}

impl<K: Hash + Eq + Clone> StreamHashSet<K> {
    pub fn new() -> StreamHashSet<K> {
        StreamHashSet {
            value: HashSet::new(),
            modified: HashSet::new(),
        }
    }
    pub fn insert(mut self: Tack<Self>, key: K) {
        let this = self.into_inner_unchecked();
        this.value.insert(key.clone());
        this.modified.insert(key);
    }

    pub fn remove<Q>(mut self: Tack<Self>, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let this = self.into_inner_unchecked();
        if let Some(key) = this.value.take(key) {
            this.modified.insert(key);
        }
    }
}

impl<K: Hash + Eq + Clone + Serialize> Serialize for StreamHashSet<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}
impl<K: Hash + Eq + Clone + Serialize> StreamSerialize for StreamHashSet<K> {
    fn build_baseline(&mut self) {
        self.modified.clear();
    }

    fn build_target(&mut self) -> bool {
        !self.modified.is_empty()
    }

    fn serialize_update<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.modified.len()))?;
        for key in &self.modified {
            if self.value.contains(key) {
                seq.serialize_element(&StreamHashEntry::Insert(key))?;
            } else {
                seq.serialize_element(&StreamHashEntry::Remove(key))?;
            }
        }
        seq.end()
    }
}

impl<'de, K: Eq + Hash + Deserialize<'de>> Deserialize<'de> for StreamHashSet<K> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(StreamHashSet {
            value: HashSet::deserialize(deserializer)?,
            modified: HashSet::new(),
        })
    }
}
impl<'de, K: Eq + Hash + Deserialize<'de>> StreamDeserialize<'de> for StreamHashSet<K> {
    fn deserialize_stream<D: Deserializer<'de>>(&mut self, d: D) -> Result<(), D::Error> {
        struct V<'a, K> {
            this: &'a mut StreamHashSet<K>,
        }
        impl<'a, 'de, K: Eq + Hash + Deserialize<'de>> Visitor<'de> for V<'a, K> {
            type Value = ();

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "expecting sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                while let Some(x) = seq.next_element::<StreamHashEntry<K>>()? {
                    match x {
                        StreamHashEntry::Insert(x) => self.this.value.insert(x),
                        StreamHashEntry::Remove(x) => self.this.value.remove(&x),
                    };
                }
                Ok(())
            }
        }
        d.deserialize_seq(V { this: self })
    }
}
