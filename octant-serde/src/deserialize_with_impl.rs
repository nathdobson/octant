use crate::{DeserializeWith, DeserializeWithSeed};
use serde::{de::{SeqAccess, Visitor}, Deserialize, Deserializer};
use std::{fmt::Formatter, marker::PhantomData};
use serde::de::Error;
use type_map::TypeMap;

impl<'de> DeserializeWith<'de> for () {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        <()>::deserialize(d)
    }
}

impl<'de, T1> DeserializeWith<'de> for (T1,)
where
    T1: DeserializeWith<'de>,
{
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<(T1,), D::Error> {
        struct V<'c, T1>(&'c TypeMap, PhantomData<T1>);
        impl<'c, 'de, T1: DeserializeWith<'de>> Visitor<'de> for V<'c, T1> {
            type Value = (T1,);
            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "a tuple of length 1")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                Ok((seq
                    .next_element_seed(DeserializeWithSeed::new(self.0))?
                    .ok_or_else(|| A::Error::custom("missing tuple argument"))?,))
            }
        }
        d.deserialize_tuple(1, V(ctx, PhantomData))
    }
}

impl<'de, T1, T2> DeserializeWith<'de> for (T1, T2)
where
    T1: DeserializeWith<'de>,
    T2: DeserializeWith<'de>,
{
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<(T1, T2), D::Error> {
        struct V<'c, T1, T2>(&'c TypeMap, PhantomData<(T1, T2)>);
        impl<'c, 'de, T1: DeserializeWith<'de>, T2: DeserializeWith<'de>> Visitor<'de> for V<'c, T1, T2> {
            type Value = (T1, T2);
            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "a tuple of length 1")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                Ok((
                    seq.next_element_seed(DeserializeWithSeed::new(self.0))?
                        .ok_or_else(|| A::Error::custom("missing tuple argument"))?,
                    seq.next_element_seed(DeserializeWithSeed::new(self.0))?
                        .ok_or_else(|| A::Error::custom("missing tuple argument"))?,
                ))
            }
        }
        d.deserialize_tuple(2, V(ctx, PhantomData))
    }
}

impl<'de, T: DeserializeWith<'de>> DeserializeWith<'de> for Option<T> {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        struct V<'c, T>(&'c TypeMap, PhantomData<T>);
        impl<'c, 'de, T: DeserializeWith<'de>> Visitor<'de> for V<'c, T> {
            type Value = Option<T>;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "an option")
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
            fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Some(T::deserialize_with(self.0, d)?))
            }
        }
        d.deserialize_option(V(ctx, PhantomData))
    }
}

impl<'de, T: DeserializeWith<'de>> DeserializeWith<'de> for Vec<T> {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        struct V<'c, T> {
            ctx: &'c TypeMap,
            phantom: PhantomData<T>,
        }
        impl<'c, 'de, T: DeserializeWith<'de>> Visitor<'de> for V<'c, T> {
            type Value = Vec<T>;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "a vec")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = seq.size_hint().map_or_else(Vec::new, Vec::with_capacity);
                while let Some(next) =
                    seq.next_element_seed(DeserializeWithSeed::<T>::new(self.ctx))?
                {
                    vec.push(next);
                }
                Ok(vec)
            }
        }
        d.deserialize_seq(V {
            ctx,
            phantom: PhantomData,
        })
    }
}

impl<'de> DeserializeWith<'de> for String {
    fn deserialize_with<D: Deserializer<'de>>(ctx: &TypeMap, d: D) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}
