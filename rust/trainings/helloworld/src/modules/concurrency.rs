use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Demonstrates concurrency in Rust using threads and message passing.
pub fn concurrency_example() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..5 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
