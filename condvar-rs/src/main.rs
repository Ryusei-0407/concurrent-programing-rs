use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/*
 * conditional variable is Condvar
 */

fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap(); // lock
    cvar.wait_while(started, |started| !*started).unwrap();
    /*
    while !*started {
        started = cvar.wait(started).unwrap(); // wait
    }
    */

    println!("child {id}");
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap(); // lock
    *started = true; // Update shared variables
    cvar.notify_all();
    println!("parent");
}

fn main() {
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    // spawn
    let c0 = thread::spawn(move || child(0, pair0));
    let c1 = thread::spawn(move || child(1, pair1));
    let p = thread::spawn(move || parent(pair2));

    // join
    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}
