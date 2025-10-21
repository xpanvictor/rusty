use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, TryRecvError};
use std::future::Future;

use crate::spawner::Spawner;
use crate::executor::Executor;

/// Mini Async runtime
pub struct Runtime {
    pub spawner: Spawner,
    pub executor: Executor,
    pub active: Arc<AtomicUsize>,
}

impl Runtime {
    pub fn new() -> Self {
        let (stx, rx) = mpsc::channel();
        let active = Arc::new(AtomicUsize::new(0));

        Runtime {
            spawner: Spawner::new(stx.clone(), active.clone()),
            executor: Executor::new(rx),
            active,
        }
    }

    /// Drive a future to completion on this runtime and return its output.
    pub fn block_on<F>(&mut self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let (result_sender, result_receiver) = channel();

        // Wrap user future to send its result back through a channel
        let fut_wrapper = async move {
            let result = future.await;
            let _ = result_sender.send(result);
        };

        let _jh = self.spawner.spawn(fut_wrapper);

        // Drive the executor until the result arrives
        loop {
            // could also just use the jh
            match result_receiver.try_recv() {
                Ok(val) => break val,
                Err(TryRecvError::Empty) => {
                    self.run_once();
                }
                Err(TryRecvError::Disconnected) => panic!("block_on channel disconnected"),
            }
        }
    }

    /// Run until no active tasks remain (checked via `active`).
    pub fn run(&mut self) {
        self.executor.run(&self.active);
    }

    /// Poll at most one ready task.
    pub fn run_once(&mut self) {
        self.executor.run_once();
    }
}
