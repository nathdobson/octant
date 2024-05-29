use std::{cell::RefCell, fmt::Debug, future::Future, marker::PhantomData, rc::Rc};
#[cfg(side = "server")]
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(side = "server")]
use anyhow::anyhow;
use serde::{Deserializer, Serialize, Serializer};
#[cfg(side = "server")]
use tokio::sync::oneshot;

use octant_object::define_class;
use octant_reffed::rc::Rc2;
#[cfg(side = "client")]
use octant_serde::Format;
use octant_serde::{
    define_serde_impl, DeserializeContext, DeserializeRcWith, DeserializeWith, RawEncoded,
};

#[cfg(side = "server")]
use crate::immediate_return::AsTypedHandle;
#[cfg(side = "client")]
use crate::peer::PeerValue;
use crate::{
    deserialize_object_with, future_return::FutureReturn, handle::TypedHandle,
    immediate_return::ImmediateReturn, peer::Peer, proto::UpMessage, runtime::Runtime,
};

#[cfg(side = "server")]
define_class! {
    pub class AbstractOctantFuture extends Peer {
        field sender: RefCell<Option<oneshot::Sender<RawEncoded>>>;
    }
}

#[cfg(side = "client")]
define_class! {
    pub class AbstractOctantFuture extends Peer {
    }
}

#[cfg(side = "server")]
pub struct OctantFuture<T: FutureReturn> {
    parent: RcAbstractOctantFuture,
    retain: Option<T::Retain>,
    receiver: oneshot::Receiver<RawEncoded>,
    phantom: PhantomData<T>,
}

impl<T: FutureReturn> Unpin for OctantFuture<T> {}

#[cfg(side = "client")]
pub struct OctantFuture<T: FutureReturn> {
    parent: RcAbstractOctantFuture,
    down: Rc<RefCell<Option<T::Down>>>,
    phantom: PhantomData<T>,
}

#[derive(Serialize, Debug, DeserializeWith)]
pub struct FutureResponse {
    promise: RcAbstractOctantFuture,
    value: RawEncoded,
}

define_serde_impl!(FutureResponse: UpMessage);
impl UpMessage for FutureResponse {
    #[cfg(side = "server")]
    fn run(self: Box<Self>, runtime: &Rc<Runtime>) -> anyhow::Result<()> {
        self.promise
            .sender
            .borrow_mut()
            .take()
            .ok_or_else(|| anyhow!("double return"))?
            .send(self.value)
            .ok();
        Ok(())
    }
}

#[cfg(side = "client")]
impl<T: Debug + FutureReturn> OctantFuture<T> {
    pub fn spawn<F: 'static + Future<Output = T>>(runtime: &Rc<Runtime>, f: F) -> Self {
        let parent = Rc2::new(AbstractOctantFutureValue {
            parent: PeerValue::new(),
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
                runtime
                    .sink()
                    .send(Box::<FutureResponse>::new(FutureResponse {
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
impl<T: FutureReturn> Future for OctantFuture<T> {
    type Output = anyhow::Result<T>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(up) = Pin::new(&mut (*self).receiver).poll(cx)? {
            let mut ctx = DeserializeContext::new();
            ctx.insert::<Rc<Runtime>>(self.parent.runtime().clone());
            let up = up.deserialize_as_with::<T::Up>(&ctx)?;
            let retain = self.retain.take().unwrap();
            return Poll::Ready(Ok(T::future_return(self.parent.runtime(), retain, up)));
        }
        Poll::Pending
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

impl<'de> DeserializeRcWith<'de> for dyn AbstractOctantFuture {
    fn deserialize_rc_with<D: Deserializer<'de>>(
        ctx: &DeserializeContext,
        d: D,
    ) -> Result<Rc2<Self>, D::Error> {
        deserialize_object_with(ctx, d)
    }
}

impl<T: FutureReturn> ImmediateReturn for OctantFuture<T> {
    type Down = (TypedHandle<dyn AbstractOctantFuture>, T::Down);

    #[cfg(side = "server")]
    fn immediate_new(runtime: &Rc<Runtime>) -> (Self, Self::Down) {
        let (retain, down) = T::future_new(runtime);
        let (tx, rx) = oneshot::channel();
        let peer: Rc2<dyn AbstractOctantFuture> =
            runtime.add::<AbstractOctantFutureValue>(AbstractOctantFutureValue {
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
