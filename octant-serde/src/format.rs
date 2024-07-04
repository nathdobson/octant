use octant_error::OctantResult;
use serde::Serialize;

use crate::{
    encoded::Encoded,
    RawEncoded,
    registry::{SerializeDyn, SerializeType},
};

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub enum Format {
    Text,
}

impl Default for Format {
    fn default() -> Self {
        Format::Text
    }
}

impl Format {
    pub fn serialize_raw<T: Serialize>(&self, value: &T) -> OctantResult<RawEncoded> {
        match self {
            Format::Text => Ok(RawEncoded::Text(serde_json::to_string_pretty(value)?)),
        }
    }
    pub fn serialize<U: ?Sized + SerializeType + SerializeDyn>(
        &self,
        value: &U,
    ) -> OctantResult<Encoded<U>> {
        Ok(Encoded::new(
            value.serialize_type().to_string(),
            value.serialize_dyn(*self)?,
        ))
    }
}
