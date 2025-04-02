use std::sync::{mpsc, Arc, Mutex};
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

    // Using Arc for shared ownership
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
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

    println!("Final counter value: {}", *counter.lock().unwrap());
}
