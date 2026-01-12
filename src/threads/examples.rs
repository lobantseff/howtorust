use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        Example {
            name: "basic_thread",
            description: "Creating and joining a basic thread",
            difficulty: Difficulty::Beginner,
            code: r#"use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
    });

    println!("Hello from main thread!");

    // Wait for the thread to finish
    handle.join().unwrap();
}"#,
            commentary: r#"The simplest way to create a thread in Rust:
- thread::spawn() takes a closure and runs it in a new thread
- Returns a JoinHandle that can be used to wait for completion
- handle.join() blocks until the thread finishes
- Without join(), the main thread might exit before the spawned thread completes"#,
        },
        Example {
            name: "thread_move",
            description: "Moving data into threads with move closures",
            difficulty: Difficulty::Beginner,
            code: r#"use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Data from thread: {:?}", data);
    });

    // data is no longer available here - it was moved
    handle.join().unwrap();
}"#,
            commentary: r#"The move keyword is crucial for thread safety:
- Transfers ownership of captured variables to the thread
- Prevents data races by ensuring only one owner
- Required when the spawned thread outlives the scope of borrowed values
- Without move, the closure would try to borrow, which might not be safe"#,
        },
        Example {
            name: "multiple_threads",
            description: "Spawning multiple threads and collecting results",
            difficulty: Difficulty::Intermediate,
            code: r#"use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            i * 2
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("Thread result: {}", result);
    }
}"#,
            commentary: r#"Spawning multiple threads:
- Store JoinHandle instances in a Vec
- Each thread runs independently and concurrently
- join() returns Result<T> containing the thread's return value
- The move keyword captures i by value for each iteration
- Threads may complete in any order, but we collect results sequentially"#,
        },
        Example {
            name: "shared_state",
            description: "Sharing state between threads with Arc and Mutex",
            difficulty: Difficulty::Intermediate,
            code: r#"use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

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

    println!("Result: {}", *counter.lock().unwrap());
}"#,
            commentary: r#"Safe shared mutable state:
- Arc (Atomic Reference Counting) enables multiple ownership across threads
- Mutex ensures only one thread can access the data at a time
- lock() acquires the mutex, blocking if another thread holds it
- The lock is automatically released when the MutexGuard goes out of scope
- This pattern prevents data races at compile time"#,
        },
        Example {
            name: "message_passing",
            description: "Communication between threads using channels",
            difficulty: Difficulty::Intermediate,
            code: r#"use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec!["hello", "from", "thread"];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}"#,
            commentary: r#"Message passing with channels:
- mpsc = Multiple Producer, Single Consumer
- tx (transmitter) sends values, rx (receiver) receives them
- Ownership of tx is moved into the thread
- send() transfers ownership of the value to the channel
- The receiver can be iterated over, blocking until values arrive
- Channel automatically closes when all senders are dropped"#,
        },
        Example {
            name: "multiple_producers",
            description: "Multiple threads sending to a single channel",
            difficulty: Difficulty::Advanced,
            code: r#"use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let msg = format!("Message from thread {}", i);
            tx_clone.send(msg).unwrap();
        });
    }

    // Drop the original transmitter so the receiver knows when to stop
    drop(tx);

    for received in rx {
        println!("Received: {}", received);
    }
}"#,
            commentary: r#"Multiple producers:
- Clone the transmitter for each thread
- Each clone can send independently
- drop(tx) closes the original transmitter
- The channel closes when ALL transmitters are dropped
- The receiver iteration ends when the channel closes
- Order of messages is not guaranteed"#,
        },
        Example {
            name: "thread_pool",
            description: "Simple thread pool pattern for parallel work",
            difficulty: Difficulty::Advanced,
            code: r#"use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];

    // Create worker threads
    for id in 0..4 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(num) => {
                        println!("Worker {} processing {}", id, num);
                        // Simulate work
                        let result = num * 2;
                        println!("Worker {} result: {}", id, result);
                    }
                    Err(_) => break,
                }
            }
        });
        handles.push(handle);
    }

    // Send work to the pool
    for i in 0..10 {
        tx.send(i).unwrap();
    }

    drop(tx); // Close channel to signal workers to stop

    for handle in handles {
        handle.join().unwrap();
    }
}"#,
            commentary: r#"Thread pool pattern:
- Workers share a single receiver wrapped in Arc<Mutex<>>
- Each worker competes to lock and receive from the channel
- Automatically distributes work across available threads
- drop(tx) signals workers to exit their loops
- More efficient than spawning a thread per task
- Real production code would use the threadpool or rayon crate"#,
        },
        Example {
            name: "scoped_threads",
            description: "Borrowing data in threads with scoped threads",
            difficulty: Difficulty::Advanced,
            code: r#"use std::thread;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        s.spawn(|| {
            println!("First thread reading: {:?}", data);
        });

        s.spawn(|| {
            println!("Second thread reading: {:?}", data);
        });

        // All spawned threads complete before scope ends
    });

    // Safe to mutate after scope ends
    data.push(6);
    println!("Final data: {:?}", data);
}"#,
            commentary: r#"Scoped threads (stable since Rust 1.63):
- Threads can borrow from the parent scope
- No need for move or Arc when borrowing is sufficient
- thread::scope ensures all spawned threads complete before returning
- Allows multiple immutable borrows across threads
- More ergonomic than Arc when you don't need shared ownership
- Cannot mutate borrowed data without synchronization"#,
        },
        Example {
            name: "thread_parking",
            description: "Thread parking and unparking for synchronization",
            difficulty: Difficulty::Advanced,
            code: r#"use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        println!("Thread parking...");
        thread::park();
        println!("Thread unparked!");
    });

    // Give the thread time to park
    thread::sleep(Duration::from_millis(100));

    println!("Unparking thread...");
    handle.thread().unpark();

    handle.join().unwrap();
}"#,
            commentary: r#"Thread parking:
- park() blocks the current thread until unparked
- unpark() wakes up a parked thread
- Lightweight synchronization primitive
- Can be called before park() (doesn't block if already unparked)
- Useful for building higher-level synchronization primitives
- No spurious wakeups like with condition variables
- Use with caution - easy to create deadlocks"#,
        },
        Example {
            name: "thread_builder",
            description: "Configuring threads with ThreadBuilder",
            difficulty: Difficulty::Intermediate,
            code: r#"use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("worker-thread".to_string())
        .stack_size(4 * 1024 * 1024); // 4 MB stack

    let handle = builder.spawn(|| {
        let name = thread::current().name().unwrap();
        println!("Running in thread: {}", name);

        // Simulate work that needs large stack
        let large_array = vec![0; 100000];
        println!("Array length: {}", large_array.len());
    }).unwrap();

    handle.join().unwrap();
}"#,
            commentary: r#"ThreadBuilder for advanced configuration:
- Set custom thread names for debugging
- Configure stack size for threads with deep recursion
- spawn() returns Result<JoinHandle> (can fail if OS can't create thread)
- thread::current() gets a handle to the current thread
- Useful for profiling and debugging multi-threaded applications
- Default stack size is typically 2MB but varies by platform"#,
        },
    ]
}
