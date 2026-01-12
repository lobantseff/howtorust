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
            commentary: r#"Fundamentals of async/await:
- async fn returns a Future, not the actual value
- .await executes the future and waits for completion
- #[tokio::main] sets up the async runtime
- Futures are lazy - nothing happens until .await is called
- async/await makes asynchronous code look synchronous
- The runtime handles scheduling and execution of futures"#,
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
            commentary: r#"Async functions with return values:
- async fn returns Future<Output = T> for return type T
- Use .await to extract the value from the Future
- Can chain async calls like synchronous code
- The return type annotation goes after the async keyword
- Error handling works naturally with Result types
- Each .await is a suspension point where the task can yield"#,
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
            commentary: r#"Concurrent execution with join!:
- tokio::join! runs multiple futures concurrently
- Waits for ALL futures to complete before continuing
- More efficient than sequential .await calls
- Tasks can make progress while others are waiting
- If one task panics, others are cancelled
- Returns a tuple of all results in the same order"#,
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
            commentary: r#"Racing futures with select!:
- Waits for the FIRST future to complete
- Cancels remaining futures when one completes
- Each branch can have different code and return types
- Useful for timeouts, user input, or competing operations
- Can include multiple futures and patterns
- Non-deterministic when multiple futures complete simultaneously"#,
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
            commentary: r#"Spawning tasks:
- tokio::spawn creates an independent task
- Returns JoinHandle for awaiting the result
- Tasks run on the runtime's thread pool
- Can outlive the scope that spawned them
- Must use move closure if capturing variables
- await on handle returns Result<T, JoinError>
- Spawned tasks are 'static - no borrowed data"#,
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
            commentary: r#"Async channels:
- tokio::sync::mpsc is the async equivalent of std::sync::mpsc
- send() and recv() are async operations
- Channel has a bounded capacity (32 in this example)
- send() blocks when channel is full
- recv() returns None when all senders are dropped
- More efficient than blocking channels in async context"#,
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
            commentary: r#"Async Mutex:
- tokio::sync::Mutex instead of std::sync::Mutex
- lock() is async and returns a Future
- Can hold lock across .await points safely
- More expensive than std::sync::Mutex for uncontended locks
- Use std::sync::Mutex if not holding across await
- Guards work the same - auto-release on drop
- Arc is still std::sync::Arc (not async)"#,
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
            commentary: r#"Timeouts:
- tokio::time::timeout wraps any future
- Returns Result<T, Elapsed>
- Cancels the future if timeout expires
- Useful for network operations and user interactions
- Can combine with select! for more complex timeout logic
- The timed-out future is dropped and cleaned up"#,
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
            commentary: r#"Async streams:
- Stream trait is the async equivalent of Iterator
- next() returns Future<Output = Option<T>>
- Useful for processing sequences of async values
- Can chain operations like map, filter, fold
- More composable than raw channels
- Used for handling network streams, file I/O, etc.
- Requires tokio-stream crate for utilities"#,
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
            commentary: r#"Parallel requests:
- Create multiple futures without awaiting them
- join_all() executes all futures concurrently
- Waits for all to complete and returns Vec of results
- More efficient than sequential requests
- Order of results matches order of input futures
- Consider using FuturesUnordered for as-completed processing
- Real HTTP requests would use reqwest or similar"#,
        },
    ]
}
