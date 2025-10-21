use crate::spawner::Spawner;
use std::sync::{atomic, mpsc, Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Receiver;
use std::thread::yield_now;
use crate::executor::Executor;
use crate::types::Task;

/// Mini Async runtime
struct Runtime {
    pub spawner: Spawner,
    pub executor: Executor,
    active: Arc<AtomicUsize>,
    src: Receiver<Arc<Task<()>>>,
}

impl Runtime {
    pub fn new() -> Self {
        let (stx, src) = mpsc::channel();
        let active = Arc::new(AtomicUsize::new(0));

         Runtime {
            spawner: Spawner::new(stx.clone(), active.clone()),
            executor: Executor::new(),
             active,
            src,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(task) = self.src.recv() {
                self.executor.execute(Arc::from(task));
            } else if self.active.load(Ordering::SeqCst) == 0 {
                break;
            } else {yield_now()}
        }
    }

    pub fn run_once(&mut self) {
        if let Ok(task) = self.src.try_recv() {
            self.executor.execute(Arc::from(task));
        }
    }

}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::yield_now;
    use std::time::Duration;
    use futures::join;
    use crate::runtime::Runtime;

    #[test]
    fn receives_task() {
        let mut runtime = Runtime::new();
        let jh = runtime.spawner.spawn(async {println!("hello"); 1});
        runtime.run_once();
        let res = jh.join();
        assert_eq!(res, 1);
    }

    #[test]
    fn executes_task() {
        let mut runtime = Runtime::new();
        println!("executing");
        let slow = async {
            println!("slow start");
            for _ in 0..100_000 {
                yield_now();
            }
            println!("slow done");
            1
        };

        let fast = async {
            println!("fast start");
            for _ in 0..1_000 {
                yield_now();
            }
            println!("fast done");
            2
        };

        let combined = async {
            let (a, b) = join!(slow, fast);
            a + b
        };
        let res_handle = runtime.spawner.spawn(combined);
        runtime.run();

        let res = res_handle.join();
        assert_eq!(res, 3);
    }
}
