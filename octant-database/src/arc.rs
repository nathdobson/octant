use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    sync::{Arc, Weak},
};

use serde::{
    de::{EnumAccess, VariantAccess, Visitor},
    Deserialize, Deserializer, Serializer,
};

use crate::{
    de::{DeserializeContext, DeserializeSnapshotSeed, DeserializeUpdate},
    forest::ForestState,
    ser::{SerializeUpdate, SerializeUpdateAdapter},
    tree::Tree,
};

pub enum ArcOrWeak<T: ?Sized> {
    Arc(Arc<T>),
    Weak(Weak<T>),
}

impl<T: ?Sized> ArcOrWeak<T> {
    pub fn upgrade_cow<'a>(&'a self) -> Option<Cow<'a, Arc<T>>> {
        match self {
            ArcOrWeak::Arc(x) => Some(Cow::Borrowed(x)),
            ArcOrWeak::Weak(x) => x.upgrade().map(Cow::Owned),
        }
    }
}

impl SerializeUpdate for ArcOrWeak<Tree> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer>(
        &self,
        state: &ForestState,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            ArcOrWeak::Arc(x) => s.serialize_newtype_variant(
                "ArcOrWeak",
                0,
                "Arc",
                &SerializeUpdateAdapter::new(x, state),
            ),
            ArcOrWeak::Weak(x) => s.serialize_newtype_variant(
                "ArcOrWeak",
                0,
                "Weak",
                &SerializeUpdateAdapter::new(x, state),
            ),
        }
    }

    fn end_update(&mut self) {
        todo!()
    }
}

impl<'de> DeserializeUpdate<'de> for ArcOrWeak<Tree> {
    fn deserialize_snapshot<D: Deserializer<'de>>(
        table: DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        enum Tag {
            Arc,
            Weak,
        }
        struct V<'a> {
            table: DeserializeContext<'a>,
        }
        impl<'a, 'de> Visitor<'de> for V<'a> {
            type Value = ArcOrWeak<Tree>;

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (tag, access) = data.variant::<Tag>()?;
                match tag {
                    Tag::Arc => Ok(ArcOrWeak::Arc(
                        access.newtype_variant_seed(DeserializeSnapshotSeed::new(self.table))?,
                    )),
                    Tag::Weak => Ok(ArcOrWeak::Weak(
                        access.newtype_variant_seed(DeserializeSnapshotSeed::new(self.table))?,
                    )),
                }
            }
            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "enum")
            }
        }
        d.deserialize_enum("ArcOrWeak", &["Weak", "Arc"], V { table })
    }

    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        _table: DeserializeContext,
        _d: D,
    ) -> Result<(), D::Error> {
        // d.deserialize_
        todo!()
    }
}

pub fn arc_try_new_cyclic<T, E>(
    f: impl for<'a> FnOnce(&'a Weak<T>) -> Result<T, E>,
) -> Result<Arc<T>, E> {
    let mut err = None;
    match catch_unwind(AssertUnwindSafe(|| {
        Arc::new_cyclic(|x| match f(x) {
            Err(e) => {
                err = Some(e);
                resume_unwind(Box::new("arc_try_new_cyclic internal panic"));
            }
            Ok(x) => x,
        })
    })) {
        Err(p) => {
            if let Some(err) = err {
                return Err(err);
            } else {
                resume_unwind(p)
            }
        }
        Ok(x) => Ok(x),
    }
}

impl<T: Debug> Debug for ArcOrWeak<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArcOrWeak::Arc(x) => x.fmt(f),
            ArcOrWeak::Weak(x) => x.fmt(f),
        }
    }
}
