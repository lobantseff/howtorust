use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        Example {
            name: "basic_async",
            description: "Basic async function and await",
            difficulty: Difficulty::Beginner,
            code: r#"use tokio;

async fn say_hello() {
    println!("Hello from async function!");
}

#[tokio::main]
async fn main() {
    say_hello().await;
    println!("After await");
}"#,
            commentary: r#"📚 INTRODUCTION
Async/await is Rust's approach to asynchronous programming. An async function returns a Future that represents a computation that will complete later, and await is used to wait for that completion without blocking the entire thread.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike JavaScript where async is built into the runtime, Rust makes async opt-in for zero-cost abstractions
- Prevents blocking expensive threads while waiting for I/O operations
- Avoids callback hell and deeply nested code that plagues older async patterns
- Enables writing thousands of concurrent operations on a single thread
- Solves the "colored function" problem more elegantly than languages with separate sync/async type systems

🔍 IMPORTANT DETAILS & INTRICACIES
- Futures are lazy: calling an async function does nothing until you .await it
- async fn desugars to fn() -> impl Future<Output = T>
- #[tokio::main] macro sets up the runtime scheduler and thread pool
- Each .await is a suspension point where the task can yield to other tasks
- Async functions are zero-cost: no heap allocations for state machines
- The compiler transforms async blocks into state machines at compile time

💼 WHERE IT'S MOST USED
- Web servers handling thousands of concurrent connections (actix-web, axum)
- Database connection pools and query execution
- Network clients making many parallel API requests
- File I/O operations that would otherwise block threads
- Embedded systems with limited threads but many I/O operations

✅ TAKEAWAY
Rust's async/await provides efficient concurrency without the overhead of multiple threads. Futures are lazy and do nothing until awaited, allowing the runtime to efficiently multiplex many tasks onto a small thread pool. This enables building scalable I/O-bound applications with minimal resource usage."#,
        },
        Example {
            name: "async_return",
            description: "Async functions with return values",
            difficulty: Difficulty::Beginner,
            code: r#"use tokio;

async fn get_number() -> i32 {
    42
}

async fn double_number(n: i32) -> i32 {
    n * 2
}

#[tokio::main]
async fn main() {
    let num = get_number().await;
    println!("Got: {}", num);

    let doubled = double_number(num).await;
    println!("Doubled: {}", doubled);
}"#,
            commentary: r#"📚 INTRODUCTION
Async functions can return values just like regular functions. The async keyword transforms the return type T into Future<Output = T>, and .await extracts the value when the future completes.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike Go's goroutines which use channels for all return values, Rust lets async functions return directly
- Prevents the need to manually wrap return values in channels or callbacks
- Maintains type safety and compiler checks for return values across async boundaries
- Allows composition of async functions without manual future chaining
- Avoids error-prone manual state management that C++ coroutines require

🔍 IMPORTANT DETAILS & INTRICACIES
- async fn foo() -> T desugars to fn foo() -> impl Future<Output = T>
- The return type annotation comes after async: async fn name() -> T
- Works seamlessly with Result<T, E> for error handling with the ? operator
- Each .await suspends execution but preserves local variables in the generated state machine
- Return values are only produced when the future is polled to completion
- No implicit boxing or heap allocation for simple return types

💼 WHERE IT'S MOST USED
- Database queries that return rows or result sets
- HTTP clients returning response bodies
- File I/O operations returning file contents
- Authentication functions returning user tokens or session data
- RPC calls returning computation results

✅ TAKEAWAY
Async functions with return values work intuitively like synchronous functions, but the compiler transforms them into state machines that return futures. Using .await extracts the value and enables natural composition of async operations while maintaining Rust's type safety and zero-cost abstraction guarantees."#,
        },
        Example {
            name: "concurrent_tasks",
            description: "Running multiple async tasks concurrently",
            difficulty: Difficulty::Intermediate,
            code: r#"use tokio;
use tokio::time::{sleep, Duration};

async fn task1() {
    println!("Task 1 starting");
    sleep(Duration::from_millis(100)).await;
    println!("Task 1 done");
}

async fn task2() {
    println!("Task 2 starting");
    sleep(Duration::from_millis(50)).await;
    println!("Task 2 done");
}

#[tokio::main]
async fn main() {
    // Run concurrently
    tokio::join!(task1(), task2());
    println!("All tasks complete");
}"#,
            commentary: r#"📚 INTRODUCTION
The join! macro runs multiple futures concurrently and waits for all of them to complete. It's more efficient than sequential awaits because futures can make progress while others are waiting on I/O.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike Python's asyncio.gather() which requires dynamic allocation, join! is zero-cost at runtime
- Prevents inefficient sequential execution when operations are independent
- Avoids the complexity of manually spawning and joining tasks for simple concurrent operations
- Provides compile-time safety for concurrent execution compared to JavaScript's Promise.all()
- Eliminates race conditions from improper manual task coordination

🔍 IMPORTANT DETAILS & INTRICACIES
- join! is a macro that expands to efficient polling code at compile time
- Returns a tuple of results in the same order as the input futures
- If any future panics, all other futures are immediately cancelled
- All futures must be known at compile time (use join_all for dynamic collections)
- Futures are polled in a round-robin fashion, giving each a chance to progress
- No heap allocation or dynamic dispatch involved

💼 WHERE IT'S MOST USED
- Fetching multiple independent API resources in parallel
- Loading configuration from multiple files simultaneously
- Executing multiple database queries that don't depend on each other
- Performing parallel validation checks on user input
- Initializing multiple system components during application startup

✅ TAKEAWAY
The join! macro provides efficient, compile-time concurrent execution of multiple futures with guaranteed completion of all tasks. It's the preferred way to run independent operations concurrently without the overhead of spawning tasks, and it maintains Rust's zero-cost abstraction principle while providing clean, readable syntax for parallel operations."#,
        },
        Example {
            name: "select_macro",
            description: "Racing futures with select!",
            difficulty: Difficulty::Intermediate,
            code: r#"use tokio;
use tokio::time::{sleep, Duration};

async fn fast_task() -> &'static str {
    sleep(Duration::from_millis(50)).await;
    "fast"
}

async fn slow_task() -> &'static str {
    sleep(Duration::from_millis(200)).await;
    "slow"
}

#[tokio::main]
async fn main() {
    tokio::select! {
        result = fast_task() => {
            println!("Fast task won: {}", result);
        }
        result = slow_task() => {
            println!("Slow task won: {}", result);
        }
    }
    println!("Select complete");
}"#,
            commentary: r#"📚 INTRODUCTION
The select! macro races multiple futures against each other and executes the branch corresponding to whichever completes first. Remaining futures are immediately cancelled when one completes.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike JavaScript's Promise.race() which can leak unhandled rejections, select! properly cancels losing branches
- Prevents waiting indefinitely when you only need the first result
- Avoids resource waste from continuing unnecessary operations
- Provides compile-time checked pattern matching unlike runtime-only solutions in other languages
- Solves timeout and cancellation scenarios more elegantly than manual future management

🔍 IMPORTANT DETAILS & INTRICACIES
- The macro compiles to efficient polling code with no runtime overhead
- When one branch completes, all other futures are immediately dropped (running their Drop implementations)
- Behavior is non-deterministic if multiple futures complete in the same poll
- Each branch can have different types and execute different code
- Can combine with pattern matching for complex conditional logic
- Biased variant (select_biased!) checks branches in order for deterministic behavior

💼 WHERE IT'S MOST USED
- Implementing timeouts for network requests or user operations
- Waiting for user input or automated timeout in interactive applications
- Racing multiple data sources (cache vs network, primary vs backup server)
- Implementing cancellation tokens for long-running operations
- First-response-wins scenarios in distributed systems

✅ TAKEAWAY
The select! macro enables efficient first-wins concurrency by racing multiple futures and automatically cancelling losers. It provides zero-cost, type-safe racing semantics with proper cleanup, making it ideal for timeouts, alternative data sources, and cancellation scenarios where you only need the first result from competing operations."#,
        },
        Example {
            name: "spawn_tasks",
            description: "Spawning independent async tasks",
            difficulty: Difficulty::Intermediate,
            code: r#"use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("Task 1 complete");
        42
    });

    let handle2 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        println!("Task 2 complete");
        100
    });

    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();

    println!("Results: {}, {}", result1, result2);
}"#,
            commentary: r#"📚 INTRODUCTION
tokio::spawn creates an independent async task that runs on the runtime's thread pool. It returns a JoinHandle that can be awaited to retrieve the task's result.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike OS threads which are heavyweight (typically 2MB stack), spawned tasks use minimal memory
- Prevents blocking the current task when you need true parallelism or independent execution
- Avoids the need for manual thread management and coordination
- Provides structured concurrency compared to fire-and-forget approaches in other languages
- Enables running thousands of concurrent tasks on a handful of threads

🔍 IMPORTANT DETAILS & INTRICACIES
- Spawned tasks must be 'static - they cannot borrow local data (use move closures)
- Tasks run on the runtime's thread pool and may execute on different threads
- JoinHandle::await returns Result<T, JoinError> where the error indicates panic or cancellation
- If a JoinHandle is dropped without awaiting, the task continues running in the background
- Tasks can outlive the scope that spawned them unlike structured concurrency in some languages
- Each spawned task has a small overhead (around 64 bytes) for bookkeeping

💼 WHERE IT'S MOST USED
- Background job processing in web servers
- Handling individual client connections in network services
- Parallel processing of independent work items
- Fire-and-forget logging or metrics collection
- CPU-bound work that should run on separate threads without blocking I/O

✅ TAKEAWAY
tokio::spawn enables lightweight task parallelism by creating independent tasks that run on the runtime's thread pool. Tasks must be 'static but consume minimal resources, allowing thousands of concurrent operations. The returned JoinHandle provides a way to await results or cancel tasks, making it the foundation for scalable concurrent systems in Rust."#,
        },
        Example {
            name: "async_channels",
            description: "Message passing with async channels",
            difficulty: Difficulty::Intermediate,
            code: r#"use tokio;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}"#,
            commentary: r#"📚 INTRODUCTION
Async channels enable message passing between tasks without blocking. tokio::sync::mpsc provides multi-producer, single-consumer channels where send and receive operations are async.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike std::sync::mpsc which blocks threads, async channels yield to other tasks when waiting
- Prevents thread exhaustion in high-concurrency scenarios
- Avoids deadlocks from blocking operations in async contexts
- Provides backpressure through bounded capacity unlike unbounded channels in some languages
- Enables clean task communication without shared mutable state

🔍 IMPORTANT DETAILS & INTRICACIES
- Channel capacity creates backpressure: send().await blocks when buffer is full
- recv().await returns None when all senders are dropped, signaling clean shutdown
- Sender can be cloned for multiple producers, receiver cannot be cloned
- Unbounded channels (unbounded_channel) exist but can cause memory issues
- send().await only suspends the task, not the underlying thread
- More expensive than std::sync::mpsc for non-async code due to future machinery

💼 WHERE IT'S MOST USED
- Work distribution in worker pool patterns
- Event streaming from network handlers to processors
- Collecting results from spawned tasks
- Implementing actor model patterns
- Coordinating shutdown signals across multiple tasks

✅ TAKEAWAY
Async channels provide non-blocking message passing between tasks, enabling clean communication without shared state. The bounded capacity provides automatic backpressure, preventing fast producers from overwhelming slow consumers. They're essential for task coordination in async systems and are more efficient than blocking channels when used in async contexts."#,
        },
        Example {
            name: "async_mutex",
            description: "Shared state with async Mutex",
            difficulty: Difficulty::Advanced,
            code: r#"use tokio;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut num = counter.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().await);
}"#,
            commentary: r#"📚 INTRODUCTION
Async Mutex (tokio::sync::Mutex) provides mutual exclusion for shared state in async contexts. Unlike std::sync::Mutex, its lock() method is async and can be held across await points safely.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike std::sync::Mutex which blocks threads, async Mutex yields when lock is contended
- Prevents holding a thread-blocking lock across await points which could deadlock the runtime
- Avoids the "await while holding std::sync::Mutex" footgun that can freeze async executors
- Provides safe shared mutable state without the complexity of message passing
- Enables holding locks across async operations unlike std::sync::Mutex

🔍 IMPORTANT DETAILS & INTRICACIES
- lock().await suspends the task until the lock is available, not the thread
- Can safely hold the guard across .await points without blocking the runtime
- More expensive than std::sync::Mutex for short critical sections (use std::sync for CPU-only work)
- Still use Arc<Mutex<T>> pattern for sharing between tasks
- MutexGuard implements Drop for automatic unlock, preventing lock leaks
- Not fair: starvation is possible under heavy contention

💼 WHERE IT'S MOST USED
- Shared connection pools in web servers
- Concurrent access to configuration that changes rarely
- Protecting shared caches that need async operations during updates
- Coordinating state between multiple async tasks
- Wrapping non-Send types to share across tasks

✅ TAKEAWAY
Async Mutex enables safe shared mutable state in async code by providing non-blocking lock acquisition. It's essential when you need to hold locks across await points, but comes with performance costs compared to std::sync::Mutex. Choose async Mutex when crossing await boundaries, use std::sync::Mutex for purely synchronous critical sections, and prefer message passing when possible."#,
        },
        Example {
            name: "timeout",
            description: "Adding timeouts to async operations",
            difficulty: Difficulty::Intermediate,
            code: r#"use tokio;
use tokio::time::{timeout, sleep, Duration};

async fn slow_operation() -> &'static str {
    sleep(Duration::from_secs(5)).await;
    "completed"
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_secs(1), slow_operation()).await {
        Ok(result) => println!("Success: {}", result),
        Err(_) => println!("Operation timed out!"),
    }
}"#,
            commentary: r#"📚 INTRODUCTION
The timeout function wraps any future with a time limit. If the future doesn't complete within the specified duration, it's cancelled and an Elapsed error is returned.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike languages without built-in timeout support, Rust provides zero-cost timeout wrapping
- Prevents operations from hanging indefinitely due to network issues or bugs
- Avoids manual timer management and cancellation token coordination
- Provides automatic cleanup of timed-out operations unlike manual timeout tracking
- Enables defensive programming against unresponsive external services

🔍 IMPORTANT DETAILS & INTRICACIES
- Returns Result<T, Elapsed> where T is the future's output type
- The wrapped future is immediately dropped when timeout expires, running Drop implementations
- Uses Tokio's timer wheel for efficient timeout tracking
- No overhead if the future completes before timeout
- Can be combined with select! for more complex timeout scenarios
- Timeout duration starts when the future is first polled, not when created

💼 WHERE IT'S MOST USED
- HTTP requests to external APIs with maximum wait times
- Database queries that should fail fast rather than hang
- User input operations with maximum waiting periods
- Health checks for distributed systems
- Circuit breaker patterns for resilient services

✅ TAKEAWAY
Timeout wrapping provides automatic time-bound execution of any async operation with proper cleanup. It's a defensive programming tool that prevents hanging operations from degrading system responsiveness, and it integrates seamlessly with Rust's error handling through Result types, making timeout scenarios first-class concerns in your API design."#,
        },
        Example {
            name: "async_stream",
            description: "Processing streams of values asynchronously",
            difficulty: Difficulty::Advanced,
            code: r#"use tokio;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    let mut stream = ReceiverStream::new(rx);

    while let Some(value) = stream.next().await {
        let doubled = value * 2;
        println!("{} -> {}", value, doubled);
    }
}"#,
            commentary: r#"📚 INTRODUCTION
The Stream trait is the async equivalent of Iterator. Instead of next() returning Option<T>, it returns a future that resolves to Option<T>, enabling asynchronous iteration over sequences of values.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike synchronous iterators that block on each item, streams yield control while waiting
- Prevents blocking threads when processing sequences from I/O sources
- Avoids the complexity of manually polling channels for sequential processing
- Provides composability through familiar iterator-like methods (map, filter, fold)
- Enables backpressure-aware processing unlike fire-and-forget message queues

🔍 IMPORTANT DETAILS & INTRICACIES
- Stream::next() is the fundamental method, returning Future<Output = Option<T>>
- StreamExt provides combinator methods (map, filter, collect) similar to Iterator
- More composable than raw channel receivers for transformation pipelines
- ReceiverStream wraps mpsc::Receiver to provide Stream implementation
- Streams are pull-based: consumers control the rate of item processing
- Requires tokio-stream crate for adapter types and extension traits

💼 WHERE IT'S MOST USED
- Processing WebSocket message streams in real-time applications
- Handling chunked HTTP response bodies
- Reading and transforming file contents line-by-line
- Processing database query results row-by-row
- Event stream processing with filtering and transformation

✅ TAKEAWAY
Streams bring iterator-like composability to async programming, enabling elegant processing of sequential async values. They provide backpressure naturally through their pull-based model and integrate seamlessly with channels and I/O sources. Streams are the foundation for building transformation pipelines over asynchronous data sources while maintaining non-blocking execution."#,
        },
        Example {
            name: "parallel_requests",
            description: "Making multiple concurrent async requests",
            difficulty: Difficulty::Advanced,
            code: r#"use tokio;
use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32) -> String {
    println!("Fetching data {}...", id);
    sleep(Duration::from_millis(100)).await;
    format!("Data {}", id)
}

#[tokio::main]
async fn main() {
    let futures = (0..5).map(|i| fetch_data(i));

    let results = futures::future::join_all(futures).await;

    for result in results {
        println!("Got: {}", result);
    }
}"#,
            commentary: r#"📚 INTRODUCTION
join_all executes a dynamic collection of futures concurrently, waiting for all to complete. It's the runtime equivalent of the compile-time join! macro but works with collections of unknown size.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
- Unlike join! which requires compile-time known futures, join_all handles dynamic collections
- Prevents inefficient sequential processing of independent operations
- Avoids manual spawning and collecting of task handles for simple parallel operations
- Provides ordered results unlike unordered future combinators in some libraries
- Enables fan-out patterns where one request triggers many parallel sub-requests

🔍 IMPORTANT DETAILS & INTRICACIES
- Accepts any iterator of futures and returns Vec<T> in the same order
- All futures execute concurrently on the same task (no spawning involved)
- Requires heap allocation for the result vector unlike compile-time join!
- If any future panics, join_all panics (use try_join_all for Result handling)
- FuturesUnordered provides as-completed ordering if you don't need result order
- More efficient than spawning separate tasks for each future

💼 WHERE IT'S MOST USED
- Fetching data from multiple API endpoints in web applications
- Parallel database queries for aggregating related data
- Batch processing of independent work items
- Loading multiple resources (images, files) simultaneously
- Fan-out operations in microservices architecture

✅ TAKEAWAY
join_all provides efficient concurrent execution of dynamic collections of futures with guaranteed completion and ordered results. It's the go-to pattern for parallel request processing when you need all results before proceeding. While it requires heap allocation, it avoids the overhead of spawning tasks and provides clean, predictable semantics for batch async operations."#,
        },
    ]
}
