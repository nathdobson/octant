use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::stream_deserialize::StreamDeserialize;
use crate::stream_serialize::StreamSerialize;
use crate::tack::Tack;

#[derive(Clone)]
pub struct Field<T> {
    modified: bool,
    value: T,
}

impl<T> Field<T> {
    pub fn new(value: T) -> Self {
        Field {
            modified: false,
            value,
        }
    }
    pub fn into_inner(self) -> T {
        self.value
    }
    pub fn as_mut<'a>(self: Tack<'a, Self>) -> Tack<'a, T> {
        let this = self.into_inner_unchecked();
        this.modified = true;
        Tack::new(&mut this.value)
    }
}

impl<T> Deref for Field<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Field<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.modified = true;
        &mut self.value
    }
}

impl<T> Borrow<T> for Field<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<T> BorrowMut<T> for Field<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T> AsRef<T> for Field<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for Field<T> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T> From<T> for Field<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Serialize> Serialize for Field<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<T: StreamSerialize> StreamSerialize for Field<T> {
    fn build_baseline(&mut self) {
        self.modified = false;
    }

    fn build_target(&mut self) -> bool {
        self.modified
    }

    fn serialize_update<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize_update(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Field<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Field {
            modified: false,
            value: T::deserialize(d)?,
        })
    }
}

impl<'de, T: StreamDeserialize<'de>> StreamDeserialize<'de> for Field<T> {
    fn deserialize_stream<D: Deserializer<'de>>(&mut self, d: D) -> Result<(), D::Error> {
        self.deref_mut().deserialize_stream(d)
    }
}
