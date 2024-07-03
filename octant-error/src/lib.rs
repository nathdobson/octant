use std::fmt::{Debug, Display, Formatter};

use anyhow::anyhow;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[doc(hidden)]
pub mod reexports {
    pub use anyhow;
}

#[cfg(feature = "wasm")]
pub mod wasm;

pub struct OctantError(anyhow::Error);

#[macro_export]
macro_rules! octant_error {
    ($($x:tt)*) => {$crate::OctantError::from($crate::reexports::anyhow::anyhow!($($x)*))};
}

pub type OctantResult<T> = Result<T, OctantError>;

impl Display for OctantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for OctantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

// impl std::error::Error for OctantError {}

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

impl From<std::io::Error> for OctantError {
    fn from(value: std::io::Error) -> Self {
        OctantError(anyhow::Error::from(value))
    }
}

impl From<serde_json::Error> for OctantError {
    fn from(value: serde_json::Error) -> Self {
        OctantError(anyhow::Error::from(value))
    }
}

#[cfg(feature = "tokio")]
impl From<tokio::sync::oneshot::error::RecvError> for OctantError {
    fn from(value: tokio::sync::oneshot::error::RecvError) -> Self {
        OctantError(anyhow::Error::from(value))
    }
}

#[cfg(feature = "warp")]
impl From<warp::Error> for OctantError {
    fn from(value: warp::Error) -> Self {
        OctantError(anyhow::Error::from(value))
    }
}

#[cfg(feature = "url")]
impl From<url::ParseError> for OctantError {
    fn from(value: url::ParseError) -> Self {
        OctantError(anyhow::Error::from(value))
    }
}

#[cfg(feature = "webauthn-rs-core")]
impl From<webauthn_rs_core::error::WebauthnError> for OctantError {
    fn from(value: webauthn_rs_core::error::WebauthnError) -> Self {
        OctantError(anyhow::Error::from(value))
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
    OctantError: From<E>,
{
    fn context<C>(self, context: C) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|e| OctantError::from(e).context(context))
    }

    fn with_context<C, F>(self, context: F) -> Result<T, OctantError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| OctantError::from(e).with_context(context))
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
