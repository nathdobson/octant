use std::marker::PhantomData;
use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::DeserializeSeed;

use crate::stream_deserialize::StreamDeserialize;
use crate::stream_deserializer::StreamDeserializer;
use crate::stream_serialize::StreamSerialize;
use crate::stream_serializer::StreamSerializer;
use crate::tack::Tack;

pub struct DatabaseReader<D, R> {
    des: D,
    read: R,
}
pub struct DatabaseWriter<S, W, T> {
    ser: S,
    write: W,
    value: T,
}

impl<'de, R, D: StreamDeserializer<'de, R>> DatabaseReader<D, R> {
    pub fn new(des: D, read: R) -> Self {
        DatabaseReader { des, read }
    }
    pub async fn read<T: Send + StreamDeserialize<'de>>(
        &mut self,
    ) -> Result<Option<T>, anyhow::Error> {
        struct SnapshotSeed<T> {
            phantom: PhantomData<T>,
        }
        impl<'de, T: Deserialize<'de>> DeserializeSeed<'de> for SnapshotSeed<T> {
            type Value = T;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer)
            }
        }
        let value = self
            .des
            .deserialize_one(
                &mut self.read,
                SnapshotSeed::<T> {
                    phantom: PhantomData,
                },
            )
            .await?;
        let mut value = if let Some(value) = value {
            value
        } else {
            return Ok(None);
        };
        struct UpdateSeed<'a, T> {
            value: &'a mut T,
        }
        impl<'a, 'de, T: StreamDeserialize<'de>> DeserializeSeed<'de> for UpdateSeed<'a, T> {
            type Value = ();
            fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
            where
                D: Deserializer<'de>,
            {
                self.value.deserialize_stream(deserializer)?;
                Ok(())
            }
        }
        while self
            .des
            .deserialize_one(&mut self.read, UpdateSeed { value: &mut value })
            .await?
            .is_some()
        {}
        Ok(Some(value))
    }
    pub fn into_inner(self) -> R {
        self.read
    }
}

impl<W, S: StreamSerializer<W>, T: Sync + Send + StreamSerialize> DatabaseWriter<S, W, T> {
    pub async fn new(mut ser: S, mut write: W, mut value: T) -> anyhow::Result<Self> {
        value.build_baseline();
        ser.serialize_one(&mut write, &value).await?;
        Ok(DatabaseWriter { ser, write, value })
    }
    pub fn new_append(ser: S, mut write: W, mut value: T) -> Self {
        value.build_baseline();
        DatabaseWriter { ser, write, value }
    }
    pub async fn write_update(&mut self) -> anyhow::Result<()> {
        if self.value.build_target() {
            struct S<'a, T>(&'a T);
            impl<'a, T: StreamSerialize> Serialize for S<'a, T> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    self.0.serialize_update(serializer)
                }
            }
            self.ser
                .serialize_one(&mut self.write, S(&self.value))
                .await?;
        }
        Ok(())
    }
    pub fn get_mut(&mut self) -> Tack<T> {
        Tack::new(&mut self.value)
    }
}

impl<S, W, T> Deref for DatabaseWriter<S, W, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
