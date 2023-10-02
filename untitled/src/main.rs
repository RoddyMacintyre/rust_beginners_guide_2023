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
    MPSC - Multiple producer, single receiver.
            So, data comes from multiple sources with a single receiver

MUTEX
    Mutual exclusion lock
 */

use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[allow(unused_variables)]
#[allow(unused_assignments)]

const NUM_THREADS: usize = 20;  // Usize because we need data that is local to the machine for the threads

fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("Setting timer {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("Sending {}", d);
        tx.send(d).unwrap();
    });
}

fn main() {
    // THREADS
    let mut threads = vec![];

    // Use move keyword to access variables in threads
    for i in 0..10 {
        let thread_handler = thread::spawn(move ||{
            // Use sleep, but now main program finishes before the threads, blocking full execution.
            // The solution is to join all threads together before the main thread runs
            sleep(Duration::from_millis(i * 100));
            println!("New thread {}", i);
        });
        threads.push(thread_handler);
    }

    for t in threads {
        t.join();
    }

    println!("Main thread");

    // CHANNELS
    // let (transmitter, receiver) = mpsc::channel();

    // thread::spawn(move || {
    //     transmitter.send(42).unwrap() }
    // );
    //
    // println!("Received {}", receiver.recv().unwrap());

    // Now start multiple threads and have each thread send messages to the channel
    let (transmitter, receiver) = mpsc::channel();

    for i in 0..NUM_THREADS {
        start_thread(i, transmitter.clone());   // Pass a copy of the transmitter channel
    }

    for j in receiver.iter().take(NUM_THREADS) {
        println!("Received {}", j);
    }

}