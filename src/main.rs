use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a mutex around the unit type
    let lock = Arc::new(Mutex::new(()));

    // Create a few threads that will use the mutex to synchronize
    let handles: Vec<_> = (0..100).map(|i| {

        println!("---- Thread {} is not in  the critical section", i);

        let lock = Arc::clone(&lock);
        thread::spawn( move || {
            // Acquire the lock before entering the critical section
            let _guard = lock.lock().unwrap();
            // Critical section
            println!("Thread {} is in the critical section", i);
            // The lock is automatically released when _guard goes out of scope
        })
    }).collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
