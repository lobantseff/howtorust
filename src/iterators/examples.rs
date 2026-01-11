// Iterators: Processing sequences of values

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_iteration",
            description: "Basic iteration with for loops",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

for num in numbers.iter() {
    println!("Number: {}", num);
}

// Can also iterate over ranges
for i in 0..5 {
    println!("Index: {}", i);
}"#,
            commentary: r#"Iterators provide a way to process sequences of values. The iter() method creates an iterator that
borrows each element. For loops use iterators under the hood - they're syntax sugar for calling next()
repeatedly. Ranges (0..5) also implement Iterator.

From The Rust Book (Chapter 13.2):
"The iterator pattern allows you to perform some task on a sequence of items in turn."

All iterators implement the Iterator trait, which requires defining a next() method."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "iterator_methods",
            description: "Common iterator methods: map, filter, collect",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
println!("Doubled: {:?}", doubled);

let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();
println!("Evens: {:?}", evens);"#,
            commentary: r#"Iterator adapters like map and filter transform or filter elements without executing immediately
(lazy evaluation). collect() is a consumer that triggers evaluation and gathers results into a
collection. This functional style is idiomatic in Rust and often compiles to very efficient code.

From The Rust Book (Chapter 13.2):
"Iterator adapters are methods defined on the Iterator trait that don't consume the iterator.
Instead, they produce different iterators by changing some aspect of the original iterator."

The compiler can often optimize iterator chains to be as fast as handwritten loops."#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "consuming_adapters",
            description: "Consuming adapters: sum, collect, fold",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

let sum: i32 = numbers.iter().sum();
println!("Sum: {}", sum);

let product: i32 = numbers.iter().product();
println!("Product: {}", product);

let sum_doubled: i32 = numbers.iter().map(|x| x * 2).sum();
println!("Sum of doubled: {}", sum_doubled);"#,
            commentary: r#"Consuming adapters like sum() and product() consume the iterator and produce a final value.
Unlike iterator adapters, they execute immediately. You can chain adapters before consuming - the
entire chain executes efficiently when the consumer is called.

From The Rust Book (Chapter 13.2):
"Methods that call next are called consuming adapters, because calling them uses up the iterator."

These methods take ownership of the iterator, so you can't use it after calling them."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "chaining_iterators",
            description: "Chaining multiple iterator adapters",
            code: r#"let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = numbers
    .iter()
    .filter(|x| *x % 2 == 0)  // Keep evens
    .map(|x| x * x)            // Square them
    .filter(|x| *x > 10)       // Keep those > 10
    .collect();

println!("Result: {:?}", result);"#,
            commentary: r#"Iterator adapters can be chained to build complex data transformations. Each adapter returns a
new iterator, allowing fluent method chaining. The entire pipeline is lazy - nothing executes until
a consumer like collect() is called.

From The Rust Book (Chapter 13.2):
"You can chain multiple calls to iterator adapters to perform complex actions in a readable way."

This style is often clearer and more maintainable than nested loops or imperative code."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "fold_reduce",
            description: "Using fold and reduce for accumulation",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

// fold: provide initial value
let sum = numbers.iter().fold(0, |acc, x| acc + x);
println!("Sum with fold: {}", sum);

let factorial = (1..=5).fold(1, |acc, x| acc * x);
println!("5! = {}", factorial);

// reduce: no initial value, returns Option
let max = numbers.iter().reduce(|a, b| if a > b { a } else { b });
println!("Max: {:?}", max);"#,
            commentary: r#"fold() accumulates values using an initial value and a closure. It's extremely versatile - you can
implement most other iterator methods using fold. reduce() is similar but uses the first element as
the initial value, returning Option in case the iterator is empty.

From The Rust Book (Chapter 13.2):
"The fold method takes an initial value and a closure with two arguments: an accumulator and an element."

fold is powerful but can be less readable than specialized methods like sum() or collect()."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "enumerate_zip",
            description: "Using enumerate and zip",
            code: r#"let names = vec!["Alice", "Bob", "Charlie"];

// enumerate: add indices
for (i, name) in names.iter().enumerate() {
    println!("{}: {}", i, name);
}

// zip: combine two iterators
let ages = vec![25, 30, 35];
let combined: Vec<_> = names.iter().zip(ages.iter()).collect();
println!("Combined: {:?}", combined);"#,
            commentary: r#"enumerate() adds indices to elements, yielding (index, element) tuples. zip() combines two iterators
pairwise, stopping when the shorter iterator ends. Both are essential for working with multiple
sequences simultaneously.

From The Rust Book (Chapter 13.2):
"The enumerate method returns a new iterator that yields tuples of (index, value)."

Common pattern: Use enumerate when you need both position and value, zip when processing parallel sequences."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "find_any_all",
            description: "Finding elements: find, any, all",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

let first_even = numbers.iter().find(|x| *x % 2 == 0);
println!("First even: {:?}", first_even);

let has_even = numbers.iter().any(|x| *x % 2 == 0);
println!("Has even: {}", has_even);

let all_positive = numbers.iter().all(|x| *x > 0);
println!("All positive: {}", all_positive);"#,
            commentary: r#"find() returns the first element matching a predicate (as Option). any() returns true if ANY element
matches. all() returns true if ALL elements match. These short-circuit - they stop iterating as soon
as the answer is known.

From The Rust Book (Chapter 13.2):
"The find method returns the first item in the iterator that matches the predicate."

These methods are more expressive than manual loops and clearly communicate intent."#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "custom_iterator",
            description: "Creating a custom iterator",
            code: r#"struct Counter {
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
}"#,
            commentary: r#"Creating custom iterators involves implementing the Iterator trait with its associated type Item
and next() method. Once you implement next(), you automatically get all other iterator methods for
free. This makes custom iteration patterns first-class citizens.

From The Rust Book (Chapter 13.2):
"You can create iterators that do anything you want by implementing the Iterator trait on your own types."

Only next() is required - all other iterator methods have default implementations built on next()."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "lazy_evaluation",
            description: "Iterators are lazy and zero-cost",
            code: r#"let numbers = vec![1, 2, 3, 4, 5];

// This doesn't execute until consumed!
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
println!("Result: {:?}", result);"#,
            commentary: r#"Iterators use lazy evaluation - adapters don't execute until a consumer is called. This allows the
compiler to optimize the entire chain together, often producing code as fast as hand-written loops.
The "zero-cost abstraction" means you pay no runtime cost for the abstraction.

From The Rust Book (Chapter 13.4):
"Iterators are one of Rust's zero-cost abstractions. The abstraction imposes no additional runtime
overhead."

Lazy evaluation also means you can create infinite iterators that only compute values as needed."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "flat_map_flatten",
            description: "Flattening nested structures",
            code: r#"let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];

let flattened: Vec<i32> = nested.iter().flatten().copied().collect();
println!("Flattened: {:?}", flattened);

// flat_map: map then flatten
let words = vec!["hello", "world"];
let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
println!("All chars: {:?}", chars);"#,
            commentary: r#"flatten() takes an iterator of iterables and produces a single flat iterator. flat_map() is map
followed by flatten - it's useful when each element maps to multiple output values. Both are common
when working with nested or hierarchical data.

From The Rust Book (Chapter 13.2):
"The flatten method is useful when you have an iterator of iterators and want to produce a single
iterator."

flat_map is equivalent to .map(f).flatten() but more concise and potentially more efficient."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
