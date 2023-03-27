use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} from child thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number from main {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // block main thread execution or exist until handle thread ends
    handle.join().unwrap();
}
