use anyhow::{anyhow, Error};
use octant_serde::{DeserializeContext, DeserializeWith};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(side="client")]
use wasm_error::WasmError;

#[derive(Debug)]
pub struct OctantError(anyhow::Error);

impl Serialize for OctantError {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(s)
    }
}

impl<'de> Deserialize<'de> for OctantError {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(OctantError(anyhow!(String::deserialize(d)?)))
    }
}

impl<'de> DeserializeWith<'de> for OctantError {
    fn deserialize_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Self, D::Error> {
        Self::deserialize(d)
    }
}

impl From<anyhow::Error> for OctantError {
    fn from(value: Error) -> Self {
        OctantError(value)
    }
}

impl From<OctantError> for anyhow::Error {
    fn from(value: OctantError) -> Self {
        value.0
    }
}

#[cfg(side = "client")]
impl From<WasmError> for OctantError {
    fn from(value: WasmError) -> Self {
        Self::from(anyhow::Error::from(value))
    }
}
