use std::{
    cell::RefCell,
    future::{Future, poll_fn},
    rc,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, ready, Wake, Waker},
};

use futures::future::LocalBoxFuture;
use slab::Slab;
use tokio::sync::mpsc;
use octant_error::OctantResult;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
struct EventTaskId(usize);

struct EventWaker {
    woken: AtomicBool,
    id: EventTaskId,
    queue: Arc<EventQueue>,
}

struct EventTask {
    waker: Arc<EventWaker>,
    spawn: Rc<EventSpawn>,
    inner: RefCell<LocalBoxFuture<'static, OctantResult<()>>>,
}

struct TaskSet {
    tasks: RefCell<Slab<Rc<EventTask>>>,
}

struct EventQueue {
    microtasks: mpsc::UnboundedSender<EventTaskId>,
    macrotasks: mpsc::UnboundedSender<EventTaskId>,
}

pub struct EventSpawn {
    queue: Arc<EventQueue>,
    task_set: rc::Weak<TaskSet>,
}

pub struct EventPool {
    microtasks: mpsc::UnboundedReceiver<EventTaskId>,
    macrotasks: mpsc::UnboundedReceiver<EventTaskId>,
    task_set: Rc<TaskSet>,
    flushing: bool,
    poll_flush: Box<dyn FnMut(&mut Context<'_>) -> Poll<OctantResult<()>>>,
}

impl EventTask {}

impl Wake for EventWaker {
    fn wake(self: Arc<Self>) {
        if !self.woken.swap(true, Ordering::SeqCst) {
            self.queue.macrotasks.send(self.id).ok();
        }
    }
}

impl EventPool {
    pub fn new<F: 'static + FnMut(&mut Context<'_>) -> Poll<OctantResult<()>>>(
        poll_flush: F,
    ) -> (Rc<EventSpawn>, Self) {
        let (micro_tx, micro_rx) = mpsc::unbounded_channel();
        let (macro_tx, macro_rx) = mpsc::unbounded_channel();
        let task_set = Rc::new(TaskSet {
            tasks: RefCell::new(Slab::new()),
        });
        let spawn = Rc::new(EventSpawn {
            queue: Arc::new(EventQueue {
                microtasks: micro_tx,
                macrotasks: macro_tx,
            }),
            task_set: Rc::downgrade(&task_set),
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

    fn poll_once(&mut self, id: EventTaskId) -> OctantResult<()> {
        let task = self.task_set.tasks.borrow_mut().get(id.0).unwrap().clone();
        task.waker.woken.store(false, Ordering::SeqCst);
        let polled = task
            .inner
            .borrow_mut()
            .as_mut()
            .poll(&mut Context::from_waker(&Waker::from(task.waker.clone())));
        match polled {
            Poll::Ready(r) => {
                self.task_set.tasks.borrow_mut().remove(id.0);
                r?;
            }
            Poll::Pending => {}
        }
        Ok(())
    }

    fn poll_step(&mut self, cx: &mut Context<'_>) -> Poll<OctantResult<()>> {
        'system: loop {
            if self.flushing {
                ready!((self.poll_flush)(cx))?;
                self.flushing = false;
            }
            'user: loop {
                let mut pending = false;
                pending |= match self.microtasks.poll_recv(cx) {
                    Poll::Ready(Some(task)) => {
                        self.poll_once(task)?;
                        self.flushing = true;
                        continue 'user;
                    }
                    Poll::Ready(None) => false,
                    Poll::Pending => true,
                };
                pending |= match self.macrotasks.poll_recv(cx) {
                    Poll::Ready(Some(task)) => {
                        self.poll_once(task)?;
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
    pub async fn run(&mut self) -> OctantResult<()> {
        poll_fn(|cx| self.poll_step(cx)).await
    }
}

impl EventSpawn {
    fn try_insert<F: 'static + Future<Output = OctantResult<()>>>(
        self: &Rc<EventSpawn>,
        f: F,
    ) -> Option<EventTaskId> {
        if let Some(tasks) = self.task_set.upgrade() {
            let ref mut tasks = *tasks.tasks.borrow_mut();
            let id = EventTaskId(tasks.vacant_key());
            let waker = Arc::new(EventWaker {
                woken: AtomicBool::new(true),
                queue: self.queue.clone(),
                id,
            });
            tasks.insert(Rc::new(EventTask {
                inner: RefCell::new(Box::pin(f)),
                waker: waker.clone(),
                spawn: self.clone(),
            }));
            Some(id)
        } else {
            None
        }
    }
    pub fn spawn<F: 'static + Future<Output = OctantResult<()>>>(self: &Rc<Self>, f: F) {
        if let Some(id) = self.try_insert(f) {
            self.queue.microtasks.send(id).ok();
        }
    }
    pub fn spawn_macro<F: 'static + Future<Output = OctantResult<()>>>(self: &Rc<Self>, f: F) {
        if let Some(id) = self.try_insert(f) {
            self.queue.macrotasks.send(id).ok();
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        future::{Future, pending, poll_fn},
        mem,
        pin::pin,
        task::Poll,
    };

    use parking_lot::Mutex;
    use tokio::task::yield_now;
    use octant_error::OctantResult;

    use crate::event_loop::EventPool;

    #[tokio::test]
    async fn test() -> OctantResult<()> {
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
    async fn test2() -> OctantResult<()> {
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
    async fn test3() -> OctantResult<()> {
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
