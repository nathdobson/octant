use crate::{DeserializeContext, DeserializeWith, Error, RawEncoded, DESERIALIZE_REGISTRY};
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

#[derive(Serialize, Deserialize)]
pub struct Encoded<U: ?Sized> {
    #[serde(rename = "type")]
    typ: String,
    value: RawEncoded,
    #[serde(skip)]
    phantom: PhantomData<U>,
}

impl<U: ?Sized> Encoded<U> {
    pub fn new(typ: String, value: RawEncoded) -> Self {
        Encoded {
            typ,
            value,
            phantom: PhantomData,
        }
    }
    pub fn as_raw(&self) -> &RawEncoded {
        &self.value
    }
    pub fn deserialize_with(&self, ctx: &DeserializeContext) -> Result<Box<U>, Error>
    where
        U: 'static,
    {
        (*DESERIALIZE_REGISTRY).deserialize_box(ctx, &self.typ, &self.value)
    }
}

impl<'de, U: ?Sized> DeserializeWith<'de> for Encoded<U> {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}

impl<U: ?Sized> Debug for Encoded<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", &self.typ)?;
        self.value.fmt(f)
    }
}
