
use by_address::ByAddress;
use std::{
    collections::HashSet,
    future::{poll_fn, Future},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Weak,
    },
    task::{ready, Context, Poll, Wake, Waker},
};

use futures::future::BoxFuture;
use parking_lot::Mutex;
use tokio::sync::mpsc;

struct Task {
    woken: AtomicBool,
    inner: Mutex<BoxFuture<'static, anyhow::Result<()>>>,
    spawn: Arc<EventSpawn>,
}

struct TaskSet {
    tasks: Mutex<HashSet<ByAddress<Arc<Task>>>>,
}

pub struct EventSpawn {
    microtasks: mpsc::UnboundedSender<Arc<Task>>,
    macrotasks: mpsc::UnboundedSender<Arc<Task>>,
    task_set: Weak<TaskSet>,
}

pub struct EventPool {
    microtasks: mpsc::UnboundedReceiver<Arc<Task>>,
    macrotasks: mpsc::UnboundedReceiver<Arc<Task>>,
    task_set: Arc<TaskSet>,
    flushing: bool,
    poll_flush: Box<dyn Send + FnMut(&mut Context<'_>) -> Poll<anyhow::Result<()>>>,
}

impl Task {
    pub fn new<F: 'static + Send + Future<Output = anyhow::Result<()>>>(
        f: F,
        spawn: Arc<EventSpawn>,
    ) -> Arc<Self> {
        let task = Arc::new(Task {
            woken: AtomicBool::new(true),
            inner: Mutex::new(Box::pin(f)),
            spawn: spawn.clone(),
        });
        if let Some(tasks) = spawn.task_set.upgrade() {
            tasks.tasks.lock().insert(ByAddress(task.clone()));
        }
        task
    }
    pub fn poll_once(self: &Arc<Self>) -> anyhow::Result<()> {
        self.woken.store(false, Ordering::SeqCst);
        match self
            .inner
            .lock()
            .as_mut()
            .poll(&mut Context::from_waker(&Waker::from(self.clone())))
        {
            Poll::Ready(r) => {
                if let Some(tasks) = self.spawn.task_set.upgrade() {
                    tasks.tasks.lock().remove(&ByAddress(self.clone()));
                }
                r?;
            }
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

impl EventPool {
    pub fn new<F: 'static + Send + FnMut(&mut Context<'_>) -> Poll<anyhow::Result<()>>>(
        poll_flush: F,
    ) -> (Arc<EventSpawn>, Self) {
        let (micro_tx, micro_rx) = mpsc::unbounded_channel();
        let (macro_tx, macro_rx) = mpsc::unbounded_channel();
        let task_set = Arc::new(TaskSet {
            tasks: Mutex::new(HashSet::new()),
        });
        let spawn = Arc::new(EventSpawn {
            microtasks: micro_tx,
            macrotasks: macro_tx,
            task_set: Arc::downgrade(&task_set),
        });
        let pool = EventPool {
            microtasks: micro_rx,
            macrotasks: macro_rx,
            task_set,
            flushing: false,
            poll_flush: Box::new(poll_flush),
        };
        (spawn, pool)
    }

    fn poll_step(&mut self, cx: &mut Context<'_>) -> Poll<anyhow::Result<()>> {
        'system: loop {
            if self.flushing {
                ready!((self.poll_flush)(cx))?;
                self.flushing = false;
            }
            'user: loop {
                let mut pending = false;
                pending |= match self.microtasks.poll_recv(cx) {
                    Poll::Ready(Some(task)) => {
                        task.poll_once()?;
                        self.flushing = true;
                        continue 'user;
                    }
                    Poll::Ready(None) => false,
                    Poll::Pending => true,
                };
                pending |= match self.macrotasks.poll_recv(cx) {
                    Poll::Ready(Some(task)) => {
                        task.poll_once()?;
                        self.flushing = true;
                        continue 'system;
                    }
                    Poll::Ready(None) => false,
                    Poll::Pending => true,
                };
                if self.flushing {
                    ready!((self.poll_flush)(cx))?;
                    self.flushing = false;
                }
                return if pending {
                    Poll::Pending
                } else {
                    Poll::Ready(Ok(()))
                };
            }
        }
    }
    pub async fn run(&mut self) -> anyhow::Result<()> {
        poll_fn(|cx| self.poll_step(cx)).await
    }
}

impl EventSpawn {
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

#[cfg(test)]
mod test {
    use std::{
        future::{pending, poll_fn, Future},
        mem,
        pin::pin,
        task::Poll,
    };

    use parking_lot::Mutex;
    use tokio::task::yield_now;
    use crate::event_loop::EventPool;


    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        static LOG: Mutex<Vec<String>> = Mutex::new(vec![]);
        let (spawn, mut pool) = EventPool::new(|_| {
            LOG.lock().push(format!("yield"));
            Poll::Ready(Ok(()))
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
        let (spawn, mut pool) = EventPool::new(|_| {
            LOG.lock().push(format!("yield"));
            Poll::Ready(Ok(()))
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

    #[tokio::test]
    async fn test3() -> anyhow::Result<()> {
        static LOG: Mutex<Vec<String>> = Mutex::new(vec![]);
        let (spawn, mut pool) = EventPool::new(|_| Poll::Ready(Ok(())));
        spawn.spawn({
            async move {
                struct Foo;
                let foo: Foo = Foo;
                impl Drop for Foo {
                    fn drop(&mut self) {
                        LOG.lock().push(format!("dropped"));
                    }
                }
                pending::<!>().await;
                mem::drop(foo);
                Ok(())
            }
        });
        mem::drop(spawn);
        {
            let mut pool_run = pin!(pool.run());
            poll_fn(|ctx| {
                match pool_run.as_mut().poll(ctx) {
                    Poll::Ready(_) => unreachable!(),
                    Poll::Pending => {}
                }
                Poll::Ready(())
            })
                .await;
        }
        assert_eq!(LOG.lock().iter().collect::<Vec<_>>(), Vec::<&str>::new());
        mem::drop(pool);
        assert_eq!(LOG.lock().iter().collect::<Vec<_>>(), vec!["dropped"]);

        Ok(())
    }
}
