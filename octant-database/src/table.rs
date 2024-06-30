use marshal::{
    context::Context, de::Deserialize, decode::AnyDecoder, encode::AnyEncoder, ser::Serialize,
};
use marshal_json::{decode::full::JsonDecoder, encode::full::JsonEncoder};
use marshal_object::{
    derive_box_object, derive_deserialize_provider, derive_serialize_provider, AsDiscriminant,
};
use marshal_pointer::RawAny;
use marshal_update::{
    de::DeserializeUpdate,
    forest::forest::{Forest, ForestRoot},
    object_map::ObjectMap,
    ser::{SerializeStream, SerializeUpdate},
};

pub struct BoxTable;
derive_box_object!(BoxTable, Table);
derive_serialize_provider!(BoxTable, JsonEncoder);
derive_deserialize_provider!(BoxTable, JsonDecoder);

pub trait Table: AsDiscriminant<BoxTable> + RawAny {}

pub struct Database {
    forest: ForestRoot<ObjectMap<BoxTable>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            forest: ForestRoot::new(Forest::new(), ObjectMap::new()),
        }
    }
    pub fn table<T: Table + Default>(&mut self) -> &T {
        self.forest.root_mut().get_or_default::<T>()
    }
}

impl Serialize<JsonEncoder> for Database {
    fn serialize<'w, 'en>(
        &self,
        e: AnyEncoder<'w, 'en, JsonEncoder>,
        ctx: Context,
    ) -> anyhow::Result<()> {
        self.forest.serialize(e, ctx)
    }
}

impl SerializeStream for Database {
    type Stream = <ForestRoot<ObjectMap<BoxTable>> as SerializeStream>::Stream;
    fn start_stream(&self, ctx: Context) -> anyhow::Result<Self::Stream> {
        self.forest.start_stream(ctx)
    }
}

impl SerializeUpdate<JsonEncoder> for Database {
    fn serialize_update(
        &self,
        stream: &mut Self::Stream,
        e: AnyEncoder<JsonEncoder>,
        ctx: Context,
    ) -> anyhow::Result<()> {
        self.forest.serialize_update(stream, e, ctx)
    }
}

impl Deserialize<JsonDecoder> for Database {
    fn deserialize<'p, 'de>(
        d: AnyDecoder<'p, 'de, JsonDecoder>,
        ctx: Context,
    ) -> anyhow::Result<Self> {
        Ok(Database {
            forest: ForestRoot::<ObjectMap<BoxTable>>::deserialize(d, ctx)?,
        })
    }
}

impl DeserializeUpdate<JsonDecoder> for Database {
    fn deserialize_update<'p, 'de>(
        &mut self,
        d: AnyDecoder<'p, 'de, JsonDecoder>,
        ctx: Context,
    ) -> anyhow::Result<()> {
        self.forest.deserialize_update(d, ctx)
    }
}
