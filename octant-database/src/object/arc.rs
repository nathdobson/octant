use crate::object::de::{
    DeserializeContext, DeserializeSnapshotAdapter, DeserializeTable, DeserializeUpdate,
};
use crate::object::option_combinator::OptionCombinator;
use crate::object::ser::{SerializeUpdate, SerializeUpdateAdapter};
use crate::object::{Row, RowTableState};
use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serializer};
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Formatter;
use std::mem;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::sync::{Arc, Weak};
use tokio::io::AsyncWriteExt;
//
// pub struct UniqueWeak<T>(Weak<T>);
//
// impl<T> UniqueWeak<T> {
//     pub fn new() -> UniqueWeak<T> {
//         unsafe {
//             let arc = Arc::new_uninit();
//             let weak: Weak<MaybeUninit<T>> = Arc::downgrade(&arc);
//             mem::drop(arc);
//             let weak_cast = mem::transmute::<Weak<MaybeUninit<T>>, Weak<T>>(weak);
//             UniqueWeak(weak_cast)
//         }
//     }
//     pub fn as_weak(&self) -> Weak<T> {
//         self.0.clone()
//     }
//     pub fn init(self, value: T) -> Arc<T> {
//         unsafe {
//             let weak = ManuallyDrop::new(self.0);
//             let mut ptr = Weak::as_ptr(&weak) as *mut T;
//             ptr.write(value);
//             Arc::increment_strong_count(ptr);
//             let result = Arc::from_raw(ptr);
//             result
//         }
//     }
// }
// #[test]
// fn test() {
//     let mut x = UniqueWeak::new();
//     let x = x.init(12);
//     assert_eq!(*x, 12);
// }

#[derive(Debug)]pub enum ArcOrWeak<T: ?Sized> {
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

impl SerializeUpdate for ArcOrWeak<Row> {
    fn begin_stream(&mut self) {}

    fn begin_update(&mut self) -> bool {
        true
    }

    fn serialize_update<S: Serializer>(
        &self,
        state: &RowTableState,
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

impl<'de> DeserializeUpdate<'de> for ArcOrWeak<Row> {
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
            type Value = ArcOrWeak<Row>;

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (tag, access) = data.variant::<Tag>()?;
                match tag {
                    Tag::Arc => Ok(ArcOrWeak::Arc(
                        access.newtype_variant_seed(DeserializeSnapshotAdapter::new(self.table))?,
                    )),
                    Tag::Weak => Ok(ArcOrWeak::Weak(
                        access.newtype_variant_seed(DeserializeSnapshotAdapter::new(self.table))?,
                    )),
                }
            }
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                todo!()
            }
        }
        d.deserialize_enum("ArcOrWeak", &["Weak", "Arc"], V { table })
    }

    fn deserialize_update<D: Deserializer<'de>>(
        &mut self,
        table: DeserializeContext,
        d: D,
    ) -> Result<(), D::Error> {
        // d.deserialize_
        todo!()
    }
}
