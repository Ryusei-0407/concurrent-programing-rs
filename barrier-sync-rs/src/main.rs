use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    // Save thread handler
    let mut v = Vec::new();

    let barrier = Arc::new(Barrier::new(10));

    for _ in 0..10 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            b.wait(); // Wait for all threads to reach this point
            println!("finished barrier");
        });
        v.push(th);
        println!("{}", v.len());
    }

    for th in v {
        th.join().unwrap();
    }
}
