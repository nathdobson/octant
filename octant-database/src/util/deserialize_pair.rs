use serde::{de::DeserializeSeed, Deserializer};

pub trait DeserializePair<'de> {
    type First;
    type Second;
    fn deserialize_first<D: Deserializer<'de>>(&mut self, d: D) -> Result<Self::First, D::Error>;
    fn deserialize_second<D: Deserializer<'de>>(
        &mut self,
        first: Self::First,
        d: D,
    ) -> Result<Self::Second, D::Error>;
}

pub struct DeserializePairFirst<'a, T>(&'a mut T);

impl<'a, T> DeserializePairFirst<'a, T> {
    pub fn new(x: &'a mut T) -> Self {
        DeserializePairFirst(x)
    }
}

impl<'de, 'a, T: DeserializePair<'de>> DeserializeSeed<'de> for DeserializePairFirst<'a, T> {
    type Value = T::First;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_first(deserializer)
    }
}

pub struct DeserializePairSecond<'a, T, F>(&'a mut T, F);

impl<'a, T, F> DeserializePairSecond<'a, T, F> {
    pub fn new(x: &'a mut T, f: F) -> Self {
        DeserializePairSecond(x, f)
    }
}

impl<'de, 'a, T: DeserializePair<'de>> DeserializeSeed<'de>
    for DeserializePairSecond<'a, T, T::First>
{
    type Value = T::Second;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.0.deserialize_second(self.1, deserializer)
    }
}
