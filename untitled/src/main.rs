/*
CONCURRENCY
RUST IS PARTICULARLY SUITED FOR CONCURRENCY BECAUSE OF ITS MEMORY GUARANTEES
    - Threads
    - Channels
    - Mutex

THREADS
    Allows to run code in parallel. Ownership/borrowing gives us
        - Memory safety
        - No data race (conditions)


CHANNELS
    Provides communication between threads

MUTEX
    Mutual exclusion lock
 */

use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let mut threads = vec![];

    // Use move keyword to access variables in threads
    for i in 0..10 {
        let thread_handler = thread::spawn(move ||{
            // Use sleep, but now main program finishes before the threads, blocking full execution.
            // The solution is to join all threads together before the main thread runs
            sleep(Duration::from_millis(i * 1000));
            println!("New thread {}", i);
        });
        threads.push(thread_handler);
    }

    for t in threads {
        t.join();
    }

    println!("Main thread");
}
