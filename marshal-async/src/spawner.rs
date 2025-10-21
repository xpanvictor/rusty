use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{channel, Sender};
use std::sync::{atomic, Arc, Mutex};
use std::sync::atomic::AtomicUsize;

use crate::types::{JoinHandle, Task};

pub struct Spawner {
    pub sd_ch: Sender<Arc<Task<()>>>,
    pub index: usize,
    pub active: Arc<AtomicUsize>,
}

impl Spawner {
    pub fn new(sd_ch: Sender<Arc<Task<()>>>, active: Arc<AtomicUsize>) -> Self {
        Spawner { sd_ch, index: 0, active }
    }

    pub fn spawn<T, F>(&mut self, f: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = channel();

        // Wrap user future, deliver T to JoinHandle via channel; task future outputs ()
        let fut_wrapper = async move {
            let result = f.await;
            let _ = tx.send(result);
        };

        self.active.fetch_add(1, atomic::Ordering::SeqCst);

        let task = Task {
            future: Mutex::new(Some(
                Box::pin(fut_wrapper) as Pin<Box<dyn Future<Output = ()> + Send + 'static>>
            )),
            sender: self.sd_ch.clone(),
            id: self.index,
            active: self.active.clone(),
        };

        self.index += 1;
        self.sd_ch.send(Arc::from(task)).unwrap();

        JoinHandle::new(rx)
    }
}