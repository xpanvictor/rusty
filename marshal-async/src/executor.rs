use crate::types::Task;
use futures::task::{ArcWake, waker_ref};
use std::sync::{Arc, mpsc::Receiver};
use std::task::{Context, Poll};
use std::sync::atomic::Ordering;
use std::sync::mpsc::TryRecvError;

pub struct Executor {
    rx: Receiver<Arc<Task<()>>>,
}

impl<T> ArcWake for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // push back to queue
        let _ = arc_self.sender.send(arc_self.clone());
    }
}

impl Executor {
    pub fn new(rx: Receiver<Arc<Task<()>>>) -> Self {
        Self { rx }
    }

    /// Blocking run loop: drains tasks until channel is closed and no active tasks remain
    pub fn run(&self, active: &std::sync::Arc<std::sync::atomic::AtomicUsize>) {
        loop {
            match self.rx.try_recv() {
                Ok(task) => self.poll(task),
                Err(TryRecvError::Empty) => {
                    if active.load(Ordering::SeqCst) == 0 {
                        break;
                    } else {
                        // let os do somn else
                        std::thread::yield_now();
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    // Channel closed; finish only when active zero
                    if active.load(Ordering::SeqCst) == 0 {
                        break;
                    } else {
                        std::thread::yield_now();
                    }
                }
            }
        }
    }

    // non blocking ver
    pub fn run_once(&self) {
        if let Ok(task) = self.rx.try_recv() {
            self.poll(task);
        }
    }

    // polling part
    pub fn poll(&self, t: Arc<Task<()>>) {
        let waker_ref = waker_ref(&t);
        let mut cx = Context::from_waker(&*waker_ref);

        let mut future_slot = t.future.lock().unwrap();
        if let Some(mut future) = future_slot.take() {
            match future.as_mut().poll(&mut cx) {
                Poll::Pending => {
                    *future_slot = Some(future);
                }
                Poll::Ready(_) => {
                    // finished: decrement shared counter
                    t.active.fetch_sub(1, Ordering::SeqCst);
                }
            }
        }
    }
}