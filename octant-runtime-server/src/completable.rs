use parking_lot::Mutex;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Completable<T> {
    tx: Mutex<Option<oneshot::Sender<T>>>,
    rx: Mutex<Option<oneshot::Receiver<T>>>,
}

impl<T: Send> Completable<T> {
    pub fn new() -> Self {
        let (tx, rx) = oneshot::channel();
        Completable {
            tx: Mutex::new(Some(tx)),
            rx: Mutex::new(Some(rx)),
        }
    }
    pub async fn recv(&self) -> T {
        let recv = self.rx.lock().take().unwrap();
        recv.await.unwrap()
    }
    pub fn send(&self, x: T) {
        self.tx.lock().take().unwrap().send(x).ok();
    }
}
