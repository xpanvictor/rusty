use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;

pub struct Task<T> {
    pub future: Mutex<Option<Pin<Box<dyn Future<Output = T> + Send + 'static>>>>,
    pub sender: Sender<Arc<Task<T>>>,
    pub id: usize,
    pub active: Arc<AtomicUsize>,
}

pub struct JoinHandle<T> {
    rx: Receiver<T>,
}

impl<T> JoinHandle<T> {
    pub fn new(rx: Receiver<T>) -> JoinHandle<T> { JoinHandle { rx } }
    pub fn join(self) -> T { self.rx.recv().unwrap() }
}