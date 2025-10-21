use crate::types::Task;
use futures::task::{ArcWake, waker_ref};
use std::mem::take;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::task::{Context, Poll, Wake, Waker};

pub struct Executor {}

impl<T> ArcWake for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // push back to queue
        arc_self.sender.send(arc_self.clone()).unwrap();
    }
}

impl Executor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute(&self, t: Arc<Task<()>>) {
        let waker_ref = waker_ref(&t);
        let mut cx = Context::from_waker(&waker_ref);
        let mut slot = t.future.lock().unwrap().take().unwrap();
        match slot.as_mut().poll(&mut cx) {
            Poll::Pending => {
                println!("Rerouting task {} to queue", t.id);
            }
            Poll::Ready(res) => {
                println!("Task of future {:?} ready with res: {:?}", t.id, res);
                t.active.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }
}
