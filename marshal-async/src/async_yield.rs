use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// A future that yields once to the executor.
pub fn yield_now() -> YieldNow {
    YieldNow {
        yielded: Arc::new(AtomicBool::new(false)),
    }
}

pub struct YieldNow {
    yielded: Arc<AtomicBool>,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // First poll: mark yielded and wake
        if !self.yielded.swap(true, Ordering::SeqCst) {
            let waker: &Waker = cx.waker();
            waker.wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}