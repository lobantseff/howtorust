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
            commentary: r#"📚 INTRODUCTION
Rust's thread::spawn() creates a new OS thread that runs concurrently with the main thread. The spawned thread executes a closure, and we get back a JoinHandle to synchronize with it.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
In languages like Python (with the GIL) or JavaScript (single-threaded), true parallelism requires processes or workers. Rust provides lightweight OS threads that can run truly in parallel on multiple cores. Problems prevented:
- Data races (prevented by ownership system)
- Use-after-free in threaded code (prevented by lifetime checking)
- Forgetting to join threads (JoinHandle makes it explicit)
- Undefined behavior from concurrent access (enforced at compile time)

🔍 IMPORTANT DETAILS & INTRICACIES
- Each thread gets its own stack (default ~2MB on most platforms)
- The closure passed to spawn() must be 'static - it cannot borrow from parent scope without move
- JoinHandle::join() returns Result<T> where T is the closure's return type
- If you don't call join(), the spawned thread becomes detached and may be terminated when main exits
- Thread scheduling is handled by the OS, not Rust
- Panics in spawned threads don't crash the program but are captured in join()'s Result

💼 WHERE IT'S MOST USED
- Web servers handling requests concurrently (though async is more common now)
- CPU-intensive parallel computations (image processing, data analysis)
- Background workers for long-running tasks
- File I/O operations that can run in parallel

✅ TAKEAWAY
Rust's thread::spawn() creates real OS threads with compile-time guarantees against data races. Unlike managed languages, you get true parallelism, and unlike C/C++, the type system prevents you from shooting yourself in the foot. Always join() your threads unless you explicitly want them detached, and remember that the closure must own its data or use 'static references."#,
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
            commentary: r#"📚 INTRODUCTION
The move keyword forces a closure to take ownership of all variables it captures from the environment. In threading, this is essential because spawned threads can outlive the scope where they were created.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
In C++, capturing by reference in a thread can lead to dangling pointers if the parent scope ends. In Java, shared references can cause race conditions without careful synchronization. Rust's move semantics prevent both. Problems prevented:
- Dangling references when parent scope ends before thread completes
- Data races from multiple threads accessing the same mutable data
- Use-after-free bugs in concurrent code
- Implicit sharing that violates the "no shared mutable state" rule

🔍 IMPORTANT DETAILS & INTRICACIES
- Without move, closures try to borrow, but the compiler rejects this for spawn() because the thread's lifetime is unknown
- move transfers ownership completely - the original scope can no longer access the data
- For Copy types (like integers), move creates a copy rather than transferring ownership
- You can still share data across threads using Arc<T> (moved into each thread)
- The 'static lifetime bound on spawn's closure requires either move or 'static references

💼 WHERE IT'S MOST USED
- Passing configuration or input data to worker threads
- Moving database connections or file handles to dedicated threads
- Transferring ownership of large data structures to avoid copying
- Background tasks that need exclusive access to resources

✅ TAKEAWAY
The move keyword is your ticket to safe concurrent programming in Rust. It ensures that threads own their data, eliminating entire classes of concurrency bugs at compile time. While it might seem restrictive compared to languages that allow free sharing, this restriction prevents the bugs that make concurrent programming notoriously difficult. When you need sharing, use Arc; when you need exclusive access, use move."#,
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
            commentary: r#"📚 INTRODUCTION
Spawning multiple threads in a loop is a common pattern for parallel computation. Each thread gets a JoinHandle stored in a collection, allowing us to wait for all threads and collect their results.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
In single-threaded languages, processing multiple items sequentially wastes CPU time. Thread pools in other languages often require complex setup. Rust makes parallel processing straightforward while maintaining safety. Problems prevented:
- Sequential bottlenecks when tasks can run in parallel
- Complex thread pool configuration for simple tasks
- Lost results when threads complete (JoinHandle captures return values)
- Race conditions when collecting results (each thread has its own handle)

🔍 IMPORTANT DETAILS & INTRICACIES
- Each iteration of the loop captures i by value due to move, so each thread gets its own copy
- Threads execute in parallel (if cores are available) but may complete in any order
- join() blocks until the thread completes and returns Result<T, Box<dyn Any + Send>>
- The Err case happens if the thread panicked - you can handle thread panics gracefully
- Creating too many OS threads (thousands) can be expensive; consider thread pools for large workloads
- The results are collected in the order we join, not the order threads complete

💼 WHERE IT'S MOST USED
- Parallel data processing (transforming items in a collection)
- Running independent computations simultaneously
- Batch processing where order doesn't matter
- Dividing work across CPU cores for performance

✅ TAKEAWAY
The pattern of spawning multiple threads and collecting results demonstrates Rust's approach to parallelism: simple, explicit, and safe. By storing JoinHandles in a Vec, you maintain control over thread lifecycle and can extract return values without shared state. This is perfect for embarrassingly parallel problems, but remember that creating many OS threads has overhead - for fine-grained parallelism, consider libraries like rayon that use thread pools."#,
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
            commentary: r#"📚 INTRODUCTION
Arc<Mutex<T>> is Rust's idiomatic pattern for shared mutable state across threads. Arc provides thread-safe reference counting, while Mutex ensures exclusive access to the inner data.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
In C++, sharing mutable state requires manual mutex management with risk of forgetting locks. In Go, shared memory can cause races if not carefully synchronized. Java requires volatile/synchronized keywords that are easy to miss. Rust enforces synchronization at compile time. Problems prevented:
- Data races from concurrent reads and writes (Mutex makes this impossible)
- Use-after-free from reference counting bugs (Arc uses atomic operations)
- Forgetting to unlock mutexes (MutexGuard uses RAII to auto-unlock)
- Deadlocks from exception/early return (guard drops even on panic)

🔍 IMPORTANT DETAILS & INTRICACIES
- Arc uses atomic operations for thread-safe reference counting (unlike Rc which is single-threaded)
- Mutex<T> implements interior mutability - you can mutate T through a shared reference
- lock() returns Result<MutexGuard<T>> - it can fail if the mutex is poisoned (previous holder panicked)
- MutexGuard implements Deref/DerefMut, so you access the inner value with *
- The guard automatically releases the lock when dropped, even in panic scenarios
- Arc::clone() increments the reference count; the data itself is not cloned
- Mutex uses OS primitives and can be expensive - not suitable for fine-grained locking

💼 WHERE IT'S MOST USED
- Shared counters and statistics across threads
- Thread-safe caches and shared data structures
- Connection pools in servers
- Shared configuration that multiple threads read/write
- Any scenario requiring "multiple readers OR one writer" pattern

✅ TAKEAWAY
Arc<Mutex<T>> is Rust's answer to shared mutable state in concurrent programs. The type system ensures you cannot accidentally create data races - if you want to share mutable state, you must use synchronization primitives. While verbose compared to languages with implicit locking, this explicitness prevents subtle bugs. The automatic lock release via RAII means you cannot forget to unlock, eliminating a whole class of errors common in C/C++."#,
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
            commentary: r#"📚 INTRODUCTION
Channels provide message-passing communication between threads. The mpsc (multiple producer, single consumer) channel allows multiple threads to send messages to a single receiver.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Go popularized "Don't communicate by sharing memory; share memory by communicating." Channels avoid the complexity of locks while enabling thread communication. In languages without channels, you'd use shared queues with manual locking. Problems prevented:
- Deadlocks from complex lock ordering (no locks needed)
- Data races from shared mutable state (ownership transferred through channel)
- Lost messages or race conditions in queue implementations
- Difficulty coordinating producer/consumer patterns

🔍 IMPORTANT DETAILS & INTRICACIES
- send() transfers ownership of the message to the channel - the sender can no longer access it
- recv() blocks until a message arrives, transferring ownership to the receiver
- The receiver can be iterated with for received in rx, blocking on each iteration
- Channel capacity is unbounded by default - messages queue up if receiver is slow
- When all senders are dropped, the channel closes and iteration ends
- try_recv() and try_send() provide non-blocking alternatives
- Channels are FIFO - messages arrive in the order they were sent (from each producer)

💼 WHERE IT'S MOST USED
- Producer-consumer patterns (workers processing tasks)
- Event loops and message dispatching
- Streaming data from background threads to main thread
- Coordinating work between pipeline stages
- Actor-like patterns where threads communicate via messages

✅ TAKEAWAY
Channels embody Rust's philosophy of "fearless concurrency" by using ownership to prevent data races. Instead of sharing memory protected by locks, you transfer ownership of data through channels. This makes concurrent code easier to reason about and prevents entire classes of bugs. The automatic channel closure when senders drop provides clean shutdown semantics, and the ability to iterate over received messages makes consumer code elegant and simple."#,
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
            commentary: r#"📚 INTRODUCTION
The "multiple producer" part of mpsc channels means you can clone the sender and have many threads sending to one receiver. This is perfect for collecting results or messages from multiple workers.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
In threading models without channels, collecting data from multiple threads requires shared data structures with complex locking. mpsc simplifies this to "clone the sender, send from anywhere." Problems prevented:
- Complex synchronization when multiple threads produce data
- Race conditions in manual queue implementations
- Memory leaks from not knowing when all producers are done
- Need for explicit "shutdown" signals (channel closes automatically)

🔍 IMPORTANT DETAILS & INTRICACIES
- Cloning the sender is cheap - it's just incrementing a reference count
- Each cloned sender must be dropped for the channel to close
- drop(tx) explicitly drops the original sender - important if you're only using clones
- The channel stays open as long as ANY sender exists
- Messages from different senders can interleave in any order (non-deterministic)
- The receiver has no way to know which sender a message came from
- If you forget to drop all senders, the receiver will block forever waiting for closure

💼 WHERE IT'S MOST USED
- Collecting results from a pool of worker threads
- Fan-in patterns where multiple data sources merge into one stream
- Logging systems where many threads send log messages to one logger
- Event aggregation where multiple sources produce events
- Load balancing where multiple workers send completed tasks to a coordinator

✅ TAKEAWAY
Multiple producers with a single consumer is a fundamental concurrency pattern that Rust makes safe and ergonomic. The requirement to explicitly drop all senders might seem tedious, but it ensures you never forget to signal completion - a common bug in other languages. The non-deterministic interleaving of messages is inherent to concurrent systems, but Rust's type system ensures that each message is safely transferred from producer to consumer without races."#,
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
            commentary: r#"📚 INTRODUCTION
A thread pool maintains a fixed set of worker threads that process tasks from a shared queue. This example shows the core pattern: workers sharing a receiver wrapped in Arc<Mutex<>>.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Creating OS threads is expensive (milliseconds and megabytes per thread). Spawning thousands of threads for short tasks wastes resources and thrashes the scheduler. Thread pools amortize thread creation cost. Problems prevented:
- Resource exhaustion from creating too many threads
- OS scheduler overhead when thread count exceeds core count
- Slow response times from thread creation latency
- Thread creation/destruction churn in high-throughput systems

🔍 IMPORTANT DETAILS & INTRICACIES
- Arc<Mutex<Receiver>> allows multiple threads to share ownership of the receiver
- Workers compete for the lock - only one can recv() at a time
- This naturally distributes work: whichever worker gets the lock first processes the next task
- recv() blocks while holding the lock, but releases it immediately after getting a message
- When the sender is dropped, recv() returns Err and workers break their loops
- The Mutex here doesn't protect data - it coordinates access to the recv() operation
- Real thread pools would use crossbeam or rayon for better performance and ergonomics

💼 WHERE IT'S MOST USED
- Web servers handling HTTP requests (one thread pool per server)
- Database connection pools
- Background job processors (sidekiq-style task queues)
- Parallel compilation and build systems
- Any system with high task throughput where tasks are short-lived

✅ TAKEAWAY
Thread pools are a fundamental building block of scalable systems, and this example shows how Rust's Arc<Mutex<>> pattern enables safe work sharing. While you'd use a library like rayon or tokio in production, understanding this pattern reveals how thread pools work under the hood. The key insight is that the mutex doesn't protect shared data - it coordinates access to the channel, ensuring work is distributed without races."#,
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
            commentary: r#"📚 INTRODUCTION
Scoped threads (stable since Rust 1.63) allow threads to borrow from their parent scope instead of requiring move or Arc. The scope guarantees all threads complete before returning.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Before scoped threads, spawning threads required either moving data or wrapping it in Arc, even for simple read-only access. This was verbose and inefficient for short-lived parallel work. Scoped threads solve this. Problems prevented:
- Unnecessary Arc allocations for temporary parallel work
- Verbose code when threads only need to read parent data
- Lifetime errors when trying to borrow across regular threads
- Manual synchronization just to access immutable data

🔍 IMPORTANT DETAILS & INTRICACIES
- thread::scope() takes a closure that spawns threads within a bounded lifetime
- The scope waits for ALL spawned threads to complete before returning
- Threads can borrow &T from the parent scope because their lifetime is bounded
- Multiple threads can have &T simultaneously (shared immutable access)
- For &mut T, you still need synchronization (Mutex/RwLock)
- Panics in scoped threads propagate to scope() which panics if any thread panicked
- The scope's lifetime ensures borrowed references can't outlive the data

💼 WHERE IT'S MOST USED
- Parallel iteration over slices or collections (read-only access)
- Dividing read-only computation across cores
- Temporary parallelism where Arc overhead is wasteful
- Processing chunks of data in parallel without ownership transfer
- Short-lived parallel tasks in a hot loop

✅ TAKEAWAY
Scoped threads bring Rust's threading model to parity with languages like Go and Java for simple cases, while maintaining safety guarantees. Instead of fighting the borrow checker with Arc for every thread, you can now borrow naturally when the thread lifetime is bounded. This makes parallel programming more ergonomic without sacrificing safety - the scope ensures threads can't outlive borrowed data, preventing the dangling reference bugs that plague other languages."#,
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
            commentary: r#"📚 INTRODUCTION
Thread parking is a low-level synchronization primitive where a thread can block itself until another thread explicitly unparks it. It's simpler than condition variables but requires careful use.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Condition variables in other languages suffer from spurious wakeups and require complex predicate loops. Semaphores are too heavy for simple signaling. Thread parking provides minimal overhead signaling. Problems prevented:
- Spurious wakeups that complicate condition variable logic
- Lost signals when notification happens before wait (unpark is idempotent)
- Overhead of mutex+condition variable for simple cases
- Need for external state to coordinate simple thread synchronization

🔍 IMPORTANT DETAILS & INTRICACIES
- park() blocks until unpark() is called on this thread's handle
- unpark() can be called BEFORE park() - the next park() will immediately return
- The unpark token is binary: multiple unparks don't stack, only one park() is unblocked
- park() doesn't require a mutex or condition variable - it's built into the thread itself
- No protection against deadlocks - if no one calls unpark(), the thread sleeps forever
- park_timeout() provides a bounded wait with Duration
- Unlike condition variables, there's no spurious wakeup - park() returns only when unparked

💼 WHERE IT'S MOST USED
- Building higher-level synchronization primitives (Barrier, Once, etc.)
- Simple producer-consumer signaling without data transfer
- Waiting for external events in event-driven systems
- Custom thread coordination in specialized data structures
- Rare in application code - more common in library implementations

✅ TAKEAWAY
Thread parking is a sharp tool - powerful but dangerous. It's the building block for many higher-level synchronization primitives in Rust's standard library, but most application code should use channels, mutexes, or condition variables instead. The key advantage is simplicity and zero overhead, but this comes at the cost of easy deadlock scenarios if you're not careful. Think of it as the assembly language of thread synchronization - use it when implementing your own primitives, not for everyday coordination."#,
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
            commentary: r#"📚 INTRODUCTION
ThreadBuilder provides fine-grained control over thread creation, allowing you to set names, stack sizes, and other parameters before spawning. It's the advanced interface to thread::spawn().

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Default thread settings aren't always appropriate - some applications need larger stacks for deep recursion, or named threads for debugging. In C/C++, you'd use pthread_attr_t with error-prone manual setup. Rust's builder pattern makes it ergonomic. Problems prevented:
- Stack overflows from deep recursion with default stack size
- Debugging nightmares when all threads are named "unnamed"
- Resource exhaustion from over-sized default stacks when spawning many threads
- Platform-specific pthread configuration code

🔍 IMPORTANT DETAILS & INTRICACIES
- Builder::spawn() returns Result<JoinHandle, io::Error> - thread creation can fail
- Default stack size is platform-dependent (typically 2MB on Linux, 8MB on macOS)
- Stack size is a lower bound - the OS may round up to page size multiples
- Thread names are limited to 15 characters on Linux, longer on other platforms
- thread::current().name() returns Option<&str> - nameless threads return None
- Stack size affects how many threads you can create (address space is limited)
- Large stacks (>8MB) can cause virtual memory exhaustion with hundreds of threads

💼 WHERE IT'S MOST USED
- Recursive algorithms that need more stack space (parsers, tree traversals)
- Memory-constrained systems where default stack is too large
- Profiling and debugging where thread names help identify bottlenecks
- Embedded systems with specific stack size requirements
- Applications that spawn many threads and need to conserve virtual memory

✅ TAKEAWAY
ThreadBuilder is your tool for threading scenarios that don't fit the defaults. While thread::spawn() works for most cases, Builder gives you the control needed for production systems where thread naming aids debugging and stack sizing prevents crashes or waste. The builder pattern makes advanced configuration discoverable and type-safe, avoiding the error-prone flag-passing of lower-level thread APIs. Remember that thread creation can fail, so handle the Result properly."#,
        },
    ]
}
