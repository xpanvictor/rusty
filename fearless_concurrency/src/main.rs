use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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
            thread::sleep(Duration::from_secs(1));
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
            thread::sleep(Duration::from_secs(1));
        }
    });

   let received = rx;
    for rec in received {
        println!("Got this: {rec}")
    }

    // block main thread execution or exist until handle thread ends
    // handle.join().unwrap();
}
