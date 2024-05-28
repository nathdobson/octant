use crate::DeserializeContext;
use octant_reffed::arc::Arc2;
use serde::{
    de::{DeserializeSeed, EnumAccess, Error, SeqAccess, VariantAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt::Formatter, marker::PhantomData};

pub trait DeserializeWith<'de>: Sized {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error>;
}

pub trait DeserializeArcWith<'de> {
    fn deserialize_arc_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Arc2<Self>, D::Error>;
}

impl<'de, T: ?Sized> DeserializeWith<'de> for Arc2<T>
where
    T: DeserializeArcWith<'de>,
{
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Arc2<T>, D::Error> {
        T::deserialize_arc_with(ctx, d)
    }
}

pub struct DeserializeWithSeed<'c, O>(&'c DeserializeContext, PhantomData<O>);

impl<'c, T> DeserializeWithSeed<'c, T> {
    pub fn new(c: &'c DeserializeContext) -> Self {
        DeserializeWithSeed(c, PhantomData)
    }
}

impl<'c, 'de, T> DeserializeSeed<'de> for DeserializeWithSeed<'c, T>
where
    T: DeserializeWith<'de>,
{
    type Value = T;
    fn deserialize<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_with(self.0, d)
    }
}

impl<'de> DeserializeWith<'de> for () {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        <()>::deserialize(d)
    }
}

impl<'de, T1> DeserializeWith<'de> for (T1,)
where
    T1: DeserializeWith<'de>,
{
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<(T1,), D::Error> {
        struct V<'c, T1>(&'c DeserializeContext, PhantomData<T1>);
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
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<(T1, T2), D::Error> {
        struct V<'c, T1, T2>(&'c DeserializeContext, PhantomData<(T1, T2)>);
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
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'c, T>(&'c DeserializeContext, PhantomData<T>);
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
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'c, T> {
            ctx: &'c DeserializeContext,
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

macro_rules! derive_deserialize_with {
    {$($type:ty;)*} => {
        $(
            impl<'de> DeserializeWith<'de> for $type {
                fn deserialize_with<D: Deserializer<'de>>(
                    ctx: &DeserializeContext,
                    d: D,
                ) -> Result<Self, D::Error> {
                    <$type>::deserialize(d)
                }
            }
        )*
    };
}

impl<'de, T, E> DeserializeWith<'de> for Result<T, E>
where
    T: DeserializeWith<'de>,
    E: DeserializeWith<'de>,
{
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        struct V<'c, T, E> {
            ctx: &'c DeserializeContext,
            phantom: PhantomData<(T, E)>,
        }
        #[derive(Deserialize)]
        enum Variant {
            Ok,
            Err,
        }
        impl<'c, 'de, T, E> Visitor<'de> for V<'c, T, E>
        where
            T: DeserializeWith<'de>,
            E: DeserializeWith<'de>,
        {
            type Value = Result<T, E>;
            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "Result")
            }
            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (variant, value) = data.variant::<Variant>()?;
                Ok(match variant {
                    Variant::Ok => {
                        Ok(value.newtype_variant_seed(DeserializeWithSeed::new(self.ctx))?)
                    }
                    Variant::Err => {
                        Err(value.newtype_variant_seed(DeserializeWithSeed::new(self.ctx))?)
                    }
                })
            }
        }
        d.deserialize_enum(
            "Result",
            &["Ok", "Err"],
            V {
                ctx,
                phantom: PhantomData,
            },
        )
    }
}

derive_deserialize_with! {
    bool;
    u8;u16;u32;u64;u128;
    i8;i16;i32;i64;i128;
    f32; f64;
    String;
}
