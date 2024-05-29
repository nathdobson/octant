use std::{
    future::Future,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::{
    runtime,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        oneshot,
    },
    task,
    task::{JoinError, JoinSet, LocalSet},
};

type Task = Box<dyn 'static + Sync + Send + FnOnce()>;

pub struct LocalSetSpawn {
    channels: Vec<UnboundedSender<Task>>,
    next: AtomicUsize,
}

pub struct LocalSetPool {
    joins: JoinSet<()>,
}

impl LocalSetPool {
    pub fn new(threads: usize) -> (Arc<LocalSetSpawn>, LocalSetPool) {
        let mut channels = Vec::with_capacity(threads);
        let mut joins = JoinSet::new();
        for _ in 0..threads {
            let (tx, mut rx) = unbounded_channel::<Task>();
            channels.push(tx);
            joins.spawn_blocking(move || {
                let locals = LocalSet::new();
                runtime::Handle::current().block_on(locals.run_until(async move {
                    while let Some(next) = rx.recv().await {
                        (next)()
                    }
                }));
            });
        }
        (
            Arc::new(LocalSetSpawn {
                channels,
                next: AtomicUsize::new(0),
            }),
            LocalSetPool { joins },
        )
    }
    pub async fn join(&mut self) -> Result<(), JoinError> {
        while let Some(x) = self.joins.join_next().await {
            x?;
        }
        Ok(())
    }
    pub fn detach(mut self) {
        self.joins.detach_all();
    }
}

impl LocalSetSpawn {
    pub fn spawn_fn<F: 'static + Sync + Send + FnOnce()>(&self, f: F) {
        let index = self.next.fetch_add(1, Ordering::Relaxed) % self.channels.len();
        self.channels[index].send(Box::<F>::new(f)).ok();
    }
    pub fn spawn_fut<Fu: 'static + Sync + Send + Future<Output = ()>>(&self, f: Fu) {
        self.spawn_fn(|| {
            task::spawn_local(f);
        })
    }
    pub fn spawn_async<F: 'static + Sync + Send + FnOnce() -> Fu, Fu: 'static + Future>(
        &self,
        f: F,
    ) -> oneshot::Receiver<Fu::Output>
    where
        Fu::Output: 'static + Sync + Send,
    {
        let (tx, rx) = oneshot::channel();
        self.spawn_fn(|| {
            let fu = f();
            task::spawn_local(async move {
                tx.send(fu.await).ok();
            });
        });
        rx
    }
}

#[cfg(test)]
mod test {
    use crate::local_set::LocalSetPool;
    use std::{future::pending, mem};
    use tokio::sync::mpsc::{error::TryRecvError, unbounded_channel};

    #[tokio::test]
    async fn test() {
        let (tx, mut rx) = unbounded_channel();
        let (s, mut p) = LocalSetPool::new(2);
        s.spawn_async(move || async move {
            tx.send("a").unwrap();
            pending::<!>().await;
        });
        assert_eq!(rx.recv().await.unwrap(), "a");
        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
        mem::drop(s);
        p.join().await.unwrap();
        assert!(rx.recv().await.is_none());
    }
}
