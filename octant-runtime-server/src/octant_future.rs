use crate::peer::Peer;
#[cfg(side = "client")]
use crate::peer::PeerValue;
use octant_object::define_class;
use std::{fmt::Debug, marker::PhantomData, sync::Arc};

#[cfg(side = "server")]
use anyhow::anyhow;
#[cfg(side = "client")]
use std::future::Future;

#[cfg(side = "server")]
use crate::immediate_return::AsTypedHandle;
use crate::{
    deserialize_object_with, future_return::FutureReturn, handle::TypedHandle,
    immediate_return::ImmediateReturn, proto::UpMessage, runtime::Runtime,
};
use octant_serde::{
    define_serde_impl, DeserializeArcWith, DeserializeContext, DeserializeWith, RawEncoded,
};

#[cfg(side = "client")]
use octant_serde::Format;
use parking_lot::Mutex;
use serde::{Deserializer, Serialize, Serializer};
#[cfg(side = "server")]
use tokio::sync::oneshot;

#[cfg(side = "server")]
define_class! {
    pub class AbstractOctantFuture extends Peer {
        field sender: Mutex<Option<oneshot::Sender<RawEncoded>>>;
    }
}

#[cfg(side = "client")]
define_class! {
    pub class AbstractOctantFuture extends Peer {
    }
}

#[cfg(side = "server")]
pub struct OctantFuture<T: FutureReturn> {
    parent: ArcAbstractOctantFuture,
    retain: T::Retain,
    receiver: oneshot::Receiver<RawEncoded>,
    phantom: PhantomData<T>,
}

#[cfg(side = "client")]
pub struct OctantFuture<T: FutureReturn> {
    parent: ArcAbstractOctantFuture,
    down: Arc<Mutex<Option<T::Down>>>,
    phantom: PhantomData<T>,
}

#[derive(Serialize, Debug, DeserializeWith)]
pub struct FutureResponse {
    promise: ArcAbstractOctantFuture,
    value: RawEncoded,
}

define_serde_impl!(FutureResponse: UpMessage);
impl UpMessage for FutureResponse {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()> {
        self.promise
            .sender
            .lock()
            .take()
            .ok_or_else(|| anyhow!("double return"))?
            .send(self.value)
            .ok();
        Ok(())
    }
}

#[cfg(side = "client")]
impl<T: Debug + FutureReturn> OctantFuture<T> {
    pub fn spawn<F: 'static + Future<Output = T>>(runtime: &Arc<Runtime>, f: F) -> Self {
        let parent = Arc::new(AbstractOctantFutureValue {
            parent: PeerValue::new(),
        });
        let down = Arc::new(Mutex::new(None));
        wasm_bindgen_futures::spawn_local({
            let parent = parent.clone();
            let down = down.clone();
            let runtime = runtime.clone();
            async move {
                let result = f.await;
                let down = down.lock().take().unwrap();
                let up = result.future_produce(&runtime, down);
                runtime.send(Box::<FutureResponse>::new(FutureResponse {
                    promise: parent,
                    value: Format::default().serialize_raw(&up).unwrap(),
                }))
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
impl<T: Sync + Send + Debug + FutureReturn> OctantFuture<T> {
    pub async fn recv(self) -> anyhow::Result<T> {
        let up = self
            .receiver
            .await
            .map_err(|e| anyhow!("Receive failed {:?}", e))?;
        let mut ctx = DeserializeContext::new();
        ctx.insert::<Arc<Runtime>>(self.parent.runtime().clone());
        let up = up.deserialize_as_with::<T::Up>(&ctx)?;
        Ok(T::future_return(self.parent.runtime(), self.retain, up))
    }
}

impl Serialize for dyn AbstractOctantFuture {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.raw_handle().serialize(s)
    }
}

impl<'de> DeserializeArcWith<'de> for dyn AbstractOctantFuture {
    fn deserialize_arc_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Arc<Self>, D::Error> {
        deserialize_object_with(ctx, d)
    }
}

impl<T: FutureReturn> ImmediateReturn for OctantFuture<T> {
    type Down = (TypedHandle<dyn AbstractOctantFuture>, T::Down);

    #[cfg(side = "server")]
    fn immediate_new(runtime: &Arc<Runtime>) -> (Self, Self::Down) {
        let (retain, down) = T::future_new(runtime);
        let (tx, rx) = oneshot::channel();
        let peer: Arc<dyn AbstractOctantFuture> =
            runtime.add::<AbstractOctantFutureValue>(AbstractOctantFutureValue {
                parent: runtime.add_uninit(),
                sender: Mutex::new(Some(tx)),
            });
        let handle = (*peer).typed_handle();
        (
            OctantFuture {
                parent: peer,
                retain,
                receiver: rx,
                phantom: PhantomData,
            },
            (handle, down),
        )
    }

    #[cfg(side = "client")]
    fn immediate_return(self, runtime: &Arc<Runtime>, down: Self::Down) {
        *self.down.lock() = Some(down.1);
        runtime.add(down.0, self.parent)
    }
}

//
// define_serde_impl!(LocationRequest: DownMessage);
// impl DownMessage for LocationRequest {
//     #[cfg(side = "client")]
//     fn run(self: Box<Self>, runtime: &Arc<Runtime>) -> anyhow::Result<()> {
//         log::info!("Received request");
//         let promise: ArcLocationPromise = Arc::new(LocationPromiseValue::new());
//         runtime.add(self.promise, promise.clone());
//         spawn_local({
//             let runtime = runtime.clone();
//             async move {
//                 log::info!("Sending response");
//                 runtime.send(Box::<LocationResponse>::new(LocationResponse {
//                     promise,
//                     location: location_impl(&runtime, self.document).await,
//                 }));
//             }
//         });
//         Ok(())
//     }
// }
//
