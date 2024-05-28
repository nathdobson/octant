#[cfg(feature = "wasm")]
pub mod wasm;

use anyhow::anyhow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct OctantError(anyhow::Error);

pub type OctantResult<T> = Result<T, OctantError>;

impl Display for OctantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for OctantError {}

impl OctantError {
    pub fn new<T: 'static + Sync + Send + std::error::Error>(x: T) -> Self {
        OctantError(anyhow::Error::new(x))
    }
    pub fn msg<T: 'static + Sync + Send + Debug + Display>(x: T) -> Self {
        OctantError(anyhow::Error::msg(x))
    }
    pub fn context<T: 'static + Sync + Send + Display>(self, context: T) -> Self {
        OctantError(self.0.context(context))
    }
    pub fn with_context<T: 'static + Sync + Send + Display, F: FnOnce() -> T>(
        self,
        context: F,
    ) -> Self {
        OctantError(self.0.context(context()))
    }
    pub fn into_anyhow(self) -> anyhow::Error {
        self.0
    }
}

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

impl From<anyhow::Error> for OctantError {
    fn from(value: anyhow::Error) -> Self {
        OctantError(value)
    }
}

pub trait Context<T> {
    // Required methods
    fn context<C>(self, context: C) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static;
    fn with_context<C, F>(self, f: F) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> Context<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|e| OctantError::new(e).context(context))
    }

    fn with_context<C, F>(self, context: F) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| OctantError::new(e).with_context(context))
    }
}

impl<T> Context<T> for Option<T> {
    fn context<C>(self, context: C) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static,
    {
        anyhow::Context::context(self, context).map_err(OctantError)
    }

    fn with_context<C, F>(self, context: F) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        anyhow::Context::with_context(self, context).map_err(OctantError)
    }
}
