pub fn run_example(name: &str) {
    match name {
        "basic_iteration" => basic_iteration(),
        "iterator_methods" => iterator_methods(),
        "consuming_adapters" => consuming_adapters(),
        "chaining_iterators" => chaining_iterators(),
        "fold_reduce" => fold_reduce(),
        "enumerate_zip" => enumerate_zip(),
        "find_any_all" => find_any_all(),
        "custom_iterator" => custom_iterator(),
        "lazy_evaluation" => lazy_evaluation(),
        "flat_map_flatten" => flat_map_flatten(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_iteration() {
    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    for i in 0..5 {
        println!("Index: {}", i);
    }
}

fn iterator_methods() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let evens: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .copied()
        .collect();
    println!("Evens: {:?}", evens);
}

fn consuming_adapters() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let product: i32 = numbers.iter().product();
    println!("Product: {}", product);

    let sum_doubled: i32 = numbers.iter().map(|x| x * 2).sum();
    println!("Sum of doubled: {}", sum_doubled);
}

fn chaining_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .filter(|x| *x > 10)
        .collect();

    println!("Result: {:?}", result);
}

fn fold_reduce() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum with fold: {}", sum);

    let factorial = (1..=5).fold(1, |acc, x| acc * x);
    println!("5! = {}", factorial);

    let max = numbers
        .iter()
        .reduce(|a, b| if a > b { a } else { b });
    println!("Max: {:?}", max);
}

fn enumerate_zip() {
    let names = vec!["Alice", "Bob", "Charlie"];

    for (i, name) in names.iter().enumerate() {
        println!("{}: {}", i, name);
    }

    let ages = vec![25, 30, 35];
    let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("Combined: {:?}", combined);
}

fn find_any_all() {
    let numbers = vec![1, 2, 3, 4, 5];

    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    let has_even = numbers.iter().any(|x| *x % 2 == 0);
    println!("Has even: {}", has_even);

    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);
}

fn custom_iterator() {
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new(5);
    for num in counter {
        println!("Count: {}", num);
    }
}

fn lazy_evaluation() {
    let numbers = vec![1, 2, 3, 4, 5];

    let iterator = numbers
        .iter()
        .map(|x| {
            println!("Mapping {}", x);
            x * 2
        })
        .filter(|x| {
            println!("Filtering {}", x);
            *x > 5
        });

    println!("Iterator created, now consuming:");
    let result: Vec<i32> = iterator.collect();
    println!("Result: {:?}", result);
}

fn flat_map_flatten() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];

    let flattened: Vec<i32> = nested.iter().flatten().copied().collect();
    println!("Flattened: {:?}", flattened);

    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
    println!("All chars: {:?}", chars);
}
