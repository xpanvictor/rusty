use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};
use std::thread::JoinHandle;

fn main() {
    let (tx, rx) = mpsc::channel();
    // another transmitter
    let tx1 = tx.clone();

    let handle = thread::spawn(move || {
        let vals = vec![
            "hello",
            "from",
            "the",
            "thread"
        ];
        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals  = vec![
            "more",
            "messages",
            "coming",
            "your",
            "way"
        ];
        for val in vals {
            tx1.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    let received = rx;
    for rec in received {
        println!("Got this: {rec}")
    }

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        println!("Mutex while owned is {:?}", m);
        *num += 2;
    }
    println!("The mutated mutex is {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The value of the counter after 10 threads: {}", counter.lock().unwrap());

    // block main thread execution or exist until handle thread ends
    // handle.join().unwrap();
}
