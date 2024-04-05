use serde::{ser, Serializer};

pub trait SerializerProxy {
    type Error: ser::Error;
    type SerializerValue<'up>: Serializer<Error = Self::Error>;
}
