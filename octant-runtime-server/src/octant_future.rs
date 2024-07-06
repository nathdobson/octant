use std::{cell::RefCell, fmt::Debug, future::Future, marker::PhantomData, rc::Rc};
#[cfg(side = "server")]
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use marshal::{
    context::OwnedContext,
    de::rc::DeserializeRc,
    decode::{AnyDecoder, Decoder},
    encode::{AnyEncoder, Encoder},
    ser::rc::SerializeRc,
    Deserialize, Serialize,
};
#[cfg(side = "server")]
use marshal_json::decode::full::JsonDecoderBuilder;
#[cfg(side = "client")]
use marshal_json::encode::full::JsonEncoderBuilder;
use marshal_object::derive_variant;
use marshal_pointer::{Rcf, RcfRef};
#[cfg(side = "server")]
use tokio::sync::oneshot;

#[cfg(side = "server")]
use octant_error::octant_error;
#[cfg(side = "server")]
use octant_error::OctantResult;
use octant_object::{class, DebugClass};

#[cfg(side = "server")]
use crate::immediate_return::AsTypedHandle;
use crate::{
    deserialize_peer,
    future_return::FutureReturn,
    handle::TypedHandle,
    immediate_return::ImmediateReturn,
    peer::{Peer, PeerFields},
    proto::{BoxUpMessage, UpMessage},
    runtime::Runtime,
    serialize_peer,
};

// pub struct BoxOctantFutureResult;
// derive_box_object!(BoxOctantFutureResult, OctantFutureResult);
// derive_serialize_provider!(BoxOctantFutureResult, JsonEncoder, BinEncoder);
// derive_deserialize_provider!(BoxOctantFutureResult, JsonDecoder, BinDecoder);
// pub trait OctantFutureResult: Debug + AsDiscriminant<BoxOctantFutureResult> + RawAny {}

// type Unit=();
// derive_variant!(BoxOctantFutureResult, Unit);
// impl OctantFutureResult for () {}

#[derive(DebugClass)]
pub struct AbstractOctantFutureFields {
    parent: PeerFields,
    #[cfg(side = "server")]
    sender: RefCell<Option<oneshot::Sender<Vec<u8>>>>,
}

#[class]
pub trait AbstractOctantFuture: Peer {}

//
// #[cfg(side = "server")]
// define_class! {
//     pub class AbstractOctantFuture extends Peer {
//         field sender: RefCell<Option<oneshot::Sender<RawEncoded>>>;
//     }
// }
//
// #[cfg(side = "client")]
// define_class! {
//     pub class AbstractOctantFuture extends Peer {
//     }
// }

#[cfg(side = "server")]
pub struct OctantFuture<T: FutureReturn> {
    parent: RcAbstractOctantFuture,
    retain: Option<T::Retain>,
    receiver: oneshot::Receiver<Vec<u8>>,
    phantom: PhantomData<T>,
}

impl<T: FutureReturn> Unpin for OctantFuture<T> {}

#[cfg(side = "client")]
pub struct OctantFuture<T: FutureReturn> {
    parent: RcAbstractOctantFuture,
    down: Rc<RefCell<Option<T::Down>>>,
    phantom: PhantomData<T>,
}

// impl<E: Encoder> Serialize<E> for Box<dyn OctantFutureResult> {
//     fn serialize<'w, 'en>(
//         &self,
//         e: AnyEncoder<'w, 'en, E>,
//         ctx: marshal::context::Context,
//     ) -> anyhow::Result<()> {
//         todo!()
//     }
// }
#[derive(Serialize, Debug, Deserialize)]
// #[serialize(bounds = Box<dyn OctantFutureResult>: Serialize<E>)]
// #[deserialize(bounds = Box<dyn OctantFutureResult>: Deserialize<D>)]
pub struct FutureResponse {
    promise: RcAbstractOctantFuture,
    value: Vec<u8>,
}

derive_variant!(BoxUpMessage, FutureResponse);
impl UpMessage for FutureResponse {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> OctantResult<()> {
        self.promise
            .sender
            .borrow_mut()
            .take()
            .ok_or_else(|| octant_error!("double return"))?
            .send(self.value)
            .ok();
        Ok(())
    }
}

#[cfg(side = "client")]
impl<T: Debug + FutureReturn> OctantFuture<T> {
    pub fn spawn<F: 'static + Future<Output = T>>(runtime: &Rc<Runtime>, f: F) -> Self {
        let parent = Rcf::new(AbstractOctantFutureFields {
            parent: PeerFields::new(),
        });
        let down = Rc::new(RefCell::new(None));
        wasm_bindgen_futures::spawn_local({
            let parent = parent.clone();
            let down = down.clone();
            let runtime = runtime.clone();
            async move {
                let result = f.await;
                let down = down.borrow_mut().take().unwrap();
                let up = result.future_produce(&runtime, down);
                let up = JsonEncoderBuilder::new()
                    .serialize(&up, OwnedContext::new().borrow())
                    .unwrap();
                runtime
                    .sink()
                    .send(Box::<FutureResponse>::new(FutureResponse {
                        promise: parent,
                        value: up.into_bytes(),
                    }));
            }
        });
        OctantFuture {
            parent,
            down,
            phantom: PhantomData,
        }
    }
}

#[cfg(side = "server")]
impl<T: FutureReturn> Future for OctantFuture<T> {
    type Output = OctantResult<T>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(up) = Pin::new(&mut (*self).receiver).poll(cx)? {
            let mut ctx = OwnedContext::new();
            ctx.insert_const((*self).parent.runtime());
            let up = JsonDecoderBuilder::new(&up).deserialize::<T::Up>(ctx.borrow())?;
            let retain = self.retain.take().unwrap();
            return Poll::Ready(Ok(T::future_return(self.parent.runtime(), retain, up)));
        }
        Poll::Pending
    }
}

impl<E: Encoder> SerializeRc<E> for dyn AbstractOctantFuture {
    fn serialize_rc<'w, 'en>(
        this: &RcfRef<Self>,
        e: AnyEncoder<'w, 'en, E>,
        ctx: marshal::context::Context,
    ) -> anyhow::Result<()> {
        serialize_peer::<E, Self>(this, e, ctx)
    }
}

impl<D: Decoder> DeserializeRc<D> for dyn AbstractOctantFuture {
    fn deserialize_rc<'p, 'de>(
        d: AnyDecoder<'p, 'de, D>,
        ctx: marshal::context::Context,
    ) -> anyhow::Result<Rcf<Self>> {
        deserialize_peer::<D, Self>(d, ctx)
    }
}

// impl Serialize for dyn AbstractOctantFuture {
//     fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         self.raw_handle().serialize(s)
//     }
// }
//
// impl<'de> DeserializeRcWith<'de> for dyn AbstractOctantFuture {
//     fn deserialize_rc_with<D: Deserializer<'de>>(
//         ctx: &DeserializeContext,
//         d: D,
//     ) -> Result<Rc2<Self>, D::Error> {
//         deserialize_object_with(ctx, d)
//     }
// }

impl<T: FutureReturn> ImmediateReturn for OctantFuture<T> {
    type Down = (TypedHandle<dyn AbstractOctantFuture>, T::Down);

    #[cfg(side = "server")]
    fn immediate_new(runtime: &Rc<Runtime>) -> (Self, Self::Down) {
        let (retain, down) = T::future_new(runtime);
        let (tx, rx) = oneshot::channel();
        let peer: Rcf<dyn AbstractOctantFuture> =
            runtime.add::<AbstractOctantFutureFields>(AbstractOctantFutureFields {
                parent: runtime.add_uninit(),
                sender: RefCell::new(Some(tx)),
            });
        let handle = (*peer).typed_handle();
        (
            OctantFuture {
                parent: peer,
                retain: Some(retain),
                receiver: rx,
                phantom: PhantomData,
            },
            (handle, down),
        )
    }

    #[cfg(side = "client")]
    fn immediate_return(self, runtime: &Rc<Runtime>, down: Self::Down) {
        *self.down.borrow_mut() = Some(down.1);
        runtime.add(down.0, self.parent)
    }
}
