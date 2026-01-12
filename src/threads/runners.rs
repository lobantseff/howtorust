use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_example(name: &str) {
    match name {
        "basic_thread" => basic_thread(),
        "thread_move" => thread_move(),
        "multiple_threads" => multiple_threads(),
        "shared_state" => shared_state(),
        "message_passing" => message_passing(),
        "multiple_producers" => multiple_producers(),
        "thread_pool" => thread_pool(),
        "scoped_threads" => scoped_threads(),
        "thread_parking" => thread_parking(),
        "thread_builder" => thread_builder(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_thread() {
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
    });

    println!("Hello from main thread!");

    handle.join().unwrap();
}

fn thread_move() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Data from thread: {:?}", data);
    });

    handle.join().unwrap();
}

fn multiple_threads() {
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
}

fn shared_state() {
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
}

fn message_passing() {
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
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let msg = format!("Message from thread {}", i);
            tx_clone.send(msg).unwrap();
        });
    }

    drop(tx);

    for received in rx {
        println!("Received: {}", received);
    }
}

fn thread_pool() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];

    for id in 0..4 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || loop {
            let job = rx.lock().unwrap().recv();
            match job {
                Ok(num) => {
                    println!("Worker {} processing {}", id, num);
                    let result = num * 2;
                    println!("Worker {} result: {}", id, result);
                }
                Err(_) => break,
            }
        });
        handles.push(handle);
    }

    for i in 0..10 {
        tx.send(i).unwrap();
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn scoped_threads() {
    let mut data = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        s.spawn(|| {
            println!("First thread reading: {:?}", data);
        });

        s.spawn(|| {
            println!("Second thread reading: {:?}", data);
        });
    });

    data.push(6);
    println!("Final data: {:?}", data);
}

fn thread_parking() {
    let handle = thread::spawn(|| {
        println!("Thread parking...");
        thread::park();
        println!("Thread unparked!");
    });

    thread::sleep(Duration::from_millis(100));

    println!("Unparking thread...");
    handle.thread().unpark();

    handle.join().unwrap();
}

fn thread_builder() {
    let builder = thread::Builder::new()
        .name("worker-thread".to_string())
        .stack_size(4 * 1024 * 1024);

    let handle = builder
        .spawn(|| {
            let current = thread::current();
            let name = current.name().unwrap();
            println!("Running in thread: {}", name);

            let large_array = vec![0; 100000];
            println!("Array length: {}", large_array.len());
        })
        .unwrap();

    handle.join().unwrap();
}
