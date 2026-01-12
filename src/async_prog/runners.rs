use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, timeout, Duration};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

pub fn run_example(name: &str) {
    match name {
        "basic_async" => basic_async(),
        "async_return" => async_return(),
        "concurrent_tasks" => concurrent_tasks(),
        "select_macro" => select_macro(),
        "spawn_tasks" => spawn_tasks(),
        "async_channels" => async_channels(),
        "async_mutex" => async_mutex(),
        "timeout" => run_timeout(),
        "async_stream" => async_stream(),
        "parallel_requests" => parallel_requests(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_async() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        say_hello().await;
        println!("After await");
    });
}

async fn say_hello() {
    println!("Hello from async function!");
}

fn async_return() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let num = get_number().await;
        println!("Got: {}", num);

        let doubled = double_number(num).await;
        println!("Doubled: {}", doubled);
    });
}

async fn get_number() -> i32 {
    42
}

async fn double_number(n: i32) -> i32 {
    n * 2
}

fn concurrent_tasks() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        tokio::join!(task1(), task2());
        println!("All tasks complete");
    });
}

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

fn select_macro() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        tokio::select! {
            result = fast_task() => {
                println!("Fast task won: {}", result);
            }
            result = slow_task() => {
                println!("Slow task won: {}", result);
            }
        }
        println!("Select complete");
    });
}

async fn fast_task() -> &'static str {
    sleep(Duration::from_millis(50)).await;
    "fast"
}

async fn slow_task() -> &'static str {
    sleep(Duration::from_millis(200)).await;
    "slow"
}

fn spawn_tasks() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
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
    });
}

fn async_channels() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let (tx, mut rx) = mpsc::channel(32);

        tokio::spawn(async move {
            for i in 0..5 {
                tx.send(i).await.unwrap();
            }
        });

        while let Some(value) = rx.recv().await {
            println!("Received: {}", value);
        }
    });
}

fn async_mutex() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
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
            handle.await.unwrap();
        }

        println!("Result: {}", *counter.lock().await);
    });
}

fn run_timeout() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match timeout(Duration::from_secs(1), slow_operation()).await {
            Ok(result) => println!("Success: {}", result),
            Err(_) => println!("Operation timed out!"),
        }
    });
}

async fn slow_operation() -> &'static str {
    sleep(Duration::from_secs(5)).await;
    "completed"
}

fn async_stream() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
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
    });
}

fn parallel_requests() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let futures = (0..5).map(|i| fetch_data(i));

        let results = futures::future::join_all(futures).await;

        for result in results {
            println!("Got: {}", result);
        }
    });
}

async fn fetch_data(id: u32) -> String {
    println!("Fetching data {}...", id);
    sleep(Duration::from_millis(100)).await;
    format!("Data {}", id)
}
