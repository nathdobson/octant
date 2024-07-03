use marshal::{context::Context, encode::AnyEncoder};
use marshal_json::{decode::full::JsonDecoder, encode::full::JsonEncoder};
use marshal_object::{
    derive_box_object, derive_deserialize_provider, derive_serialize_provider, AsDiscriminant,
};
use marshal_pointer::RawAny;
use marshal_update::{
    de::DeserializeUpdate,
    ser::{SerializeStream, SerializeUpdate, SerializeUpdateDyn},
};

pub struct BoxTable;
derive_box_object!(BoxTable, Table);
derive_serialize_provider!(BoxTable, JsonEncoder);
derive_deserialize_provider!(BoxTable, JsonDecoder);
pub trait Table:
    Sync
    + Send
    + AsDiscriminant<BoxTable>
    + RawAny
    + SerializeUpdateDyn<JsonEncoder>
    + DeserializeUpdate<JsonDecoder>
{
}

impl SerializeStream for Box<dyn Table> {
    type Stream = Box<dyn Send + Sync + RawAny>;
    fn start_stream(&self, ctx: Context) -> anyhow::Result<Self::Stream> {
        (**self).start_stream_dyn(ctx)
    }
}

impl SerializeUpdate<JsonEncoder> for Box<dyn Table> {
    fn serialize_update(
        &self,
        stream: &mut Self::Stream,
        e: AnyEncoder<JsonEncoder>,
        ctx: Context,
    ) -> anyhow::Result<()> {
        (**self).serialize_update_dyn(stream, e, ctx)
    }
}
