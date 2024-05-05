#![deny(unused_must_use)]
#![feature(never_type)]

use std::{
    future::{poll_fn, Future},
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{Context, Poll, Wake, Waker},
};

use futures::future::BoxFuture;
use parking_lot::Mutex;
use tokio::{sync::mpsc, task::yield_now};

struct Task {
    woken: AtomicBool,
    inner: Mutex<BoxFuture<'static, anyhow::Result<()>>>,
    spawn: Arc<Spawn>,
}

pub struct Spawn {
    microtasks: mpsc::UnboundedSender<Arc<Task>>,
    macrotasks: mpsc::UnboundedSender<Arc<Task>>,
}

pub struct Pool {
    microtasks: mpsc::UnboundedReceiver<Arc<Task>>,
    macrotasks: mpsc::UnboundedReceiver<Arc<Task>>,
    on_yield: Box<dyn Fn() -> anyhow::Result<()>>,
}

impl Task {
    pub fn new<F: 'static + Send + Future<Output = anyhow::Result<()>>>(
        f: F,
        spawn: Arc<Spawn>,
    ) -> Arc<Self> {
        Arc::new(Task {
            woken: AtomicBool::new(true),
            inner: Mutex::new(Box::pin(f)),
            spawn,
        })
    }
    pub fn poll_once(self: &Arc<Self>) -> anyhow::Result<()> {
        self.woken.store(false, Ordering::SeqCst);
        match self
            .inner
            .lock()
            .as_mut()
            .poll(&mut Context::from_waker(&Waker::from(self.clone())))
        {
            Poll::Ready(r) => r?,
            Poll::Pending => {}
        }
        Ok(())
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        if !self.woken.swap(true, Ordering::SeqCst) {
            self.spawn.macrotasks.send(self.clone()).ok();
        }
    }
}

impl Pool {
    pub fn new<F: 'static + Sync + Fn() -> anyhow::Result<()>>(on_yield: F) -> (Arc<Spawn>, Self) {
        let (micro_tx, micro_rx) = mpsc::unbounded_channel();
        let (macro_tx, macro_rx) = mpsc::unbounded_channel();
        let spawn = Arc::new(Spawn {
            microtasks: micro_tx,
            macrotasks: macro_tx,
        });
        let pool = Pool {
            microtasks: micro_rx,
            macrotasks: macro_rx,
            on_yield: Box::new(move || on_yield()),
        };
        (spawn, pool)
    }
    fn poll_once(&mut self, cx: &mut Context) -> Poll<anyhow::Result<()>> {
        let mut progress = false;
        let mut pending = false;
        loop {
            match self.microtasks.poll_recv(cx) {
                Poll::Ready(Some(microtask)) => {
                    progress = true;
                    microtask.poll_once()?
                }
                Poll::Ready(None) => break,
                Poll::Pending => {
                    pending = true;
                    break;
                }
            }
        }
        match self.macrotasks.poll_recv(cx) {
            Poll::Ready(Some(macrotask)) => {
                progress = true;
                macrotask.poll_once()?;
            }
            Poll::Ready(None) => {}
            Poll::Pending => {
                pending = true;
            }
        };
        if progress {
            (self.on_yield)()?;
        }
        if pending {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        poll_fn(|cx| self.poll_once(cx)).await?;
        Ok(())
    }
}

impl Spawn {
    pub fn spawn<F: 'static + Send + Future<Output = anyhow::Result<()>>>(self: &Arc<Self>, f: F) {
        self.microtasks.send(Task::new(f, self.clone())).ok();
    }
    pub fn spawn_macro<F: 'static + Send + Future<Output = anyhow::Result<()>>>(
        self: &Arc<Self>,
        f: F,
    ) {
        self.macrotasks.send(Task::new(f, self.clone())).ok();
    }
}

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    static LOG: Mutex<Vec<String>> = Mutex::new(vec![]);
    let (spawn, mut pool) = Pool::new(|| {
        LOG.lock().push(format!("yield"));
        Ok(())
    });
    spawn.spawn(async move {
        LOG.lock().push(format!("starting"));
        yield_now().await;
        LOG.lock().push(format!("finishing"));
        Ok(())
    });
    mem::drop(spawn);
    pool.run().await?;
    assert_eq!(
        LOG.lock().iter().collect::<Vec<_>>(),
        vec!["starting", "yield", "finishing", "yield"]
    );
    Ok(())
}

#[tokio::test]
async fn test2() -> anyhow::Result<()> {
    static LOG: Mutex<Vec<String>> = Mutex::new(vec![]);
    let (spawn, mut pool) = Pool::new(|| {
        LOG.lock().push(format!("yield"));
        Ok(())
    });
    spawn.spawn({
        let spawn = spawn.clone();
        async move {
            LOG.lock().push(format!("a"));
            spawn.spawn(async move {
                LOG.lock().push(format!("c"));
                Ok(())
            });
            LOG.lock().push(format!("b"));
            yield_now().await;
            LOG.lock().push(format!("d"));
            Ok(())
        }
    });
    mem::drop(spawn);
    pool.run().await?;
    assert_eq!(
        LOG.lock().iter().collect::<Vec<_>>(),
        vec!["a", "b", "c", "yield", "d", "yield"]
    );
    Ok(())
}
