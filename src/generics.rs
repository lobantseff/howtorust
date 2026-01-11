// Generic Types: Writing flexible, reusable code

use crate::{Difficulty, Example};

// Example generic structures for demonstration
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "generic_function",
            description: "Basic generic function",
            code: r#"fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

let numbers = vec![34, 50, 25, 100, 65];
println!("Largest number: {}", largest(&numbers));

let chars = vec!['y', 'm', 'a', 'q'];
println!("Largest char: {}", largest(&chars));"#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "generic_struct",
            description: "Generic struct with single type parameter",
            code: r#"#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

let integer_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };

println!("Integer point: {:?}", integer_point);
println!("Float point: {:?}", float_point);"#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "multiple_generic_params",
            description: "Generic struct with multiple type parameters",
            code: r#"#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair1 = Pair { first: 5, second: "hello" };
let pair2 = Pair { first: 1.5, second: 'c' };

println!("Pair 1: {:?}", pair1);
println!("Pair 2: {:?}", pair2);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "generic_methods",
            description: "Implementing methods on generic types",
            code: r#"struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

let p = Point { x: 3.0_f32, y: 4.0_f32 };
println!("x coordinate: {}", p.x());
println!("Distance: {}", p.distance_from_origin());"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "generic_enums",
            description: "Generic enums like Option and Result",
            code: r#"enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

let some_number = MyOption::Some(5);
let some_string = MyOption::Some("hello");
let no_value: MyOption<i32> = MyOption::None;

let success: MyResult<i32, String> = MyResult::Ok(200);
let failure: MyResult<i32, String> = MyResult::Err("Error!".to_string());

println!("Created generic enum instances");"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "trait_bounds",
            description: "Using trait bounds to constrain generics",
            code: r#"fn print_debug<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

fn compare<T: PartialOrd>(a: T, b: T) -> bool {
    a > b
}

print_debug(42);
print_debug("hello");
println!("5 > 3: {}", compare(5, 3));
println!("'a' > 'z': {}", compare('a', 'z'));"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "where_clause",
            description: "Using where clauses for complex bounds",
            code: r#"fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    println!("t: {}", t);
    println!("u: {:?}", u);
    42
}

let result = some_function(&"hello", &vec![1, 2, 3]);
println!("Function returned: {}", result);"#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "generic_associated_types",
            description: "Using generic methods with different type parameters",
            code: r#"struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn mixup<U>(self, other: Point<U>) -> Point<(T, U)> {
        Point {
            x: (self.x, other.x),
            y: (self.y, other.y),
        }
    }
}

let p1 = Point { x: 5, y: 10 };
let p2 = Point { x: "Hello", y: "World" };
let p3 = p1.mixup(p2);

println!("p3.x: ({}, {})", (p3.x).0, (p3.x).1);
println!("p3.y: ({}, {})", (p3.y).0, (p3.y).1);"#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "const_generics",
            description: "Const generics for compile-time values",
            code: r#"struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Default + Copy, const N: usize> ArrayPair<T, N> {
    fn new() -> Self {
        ArrayPair {
            left: [T::default(); N],
            right: [T::default(); N],
        }
    }
}

let pair: ArrayPair<i32, 3> = ArrayPair::new();
println!("Created array pair with size 3");

let pair2: ArrayPair<f64, 5> = ArrayPair::new();
println!("Created array pair with size 5");"#,
            difficulty: Difficulty::Advanced,
        },
    ]
}

pub fn run_example(name: &str) {
    match name {
        "generic_function" => generic_function(),
        "generic_struct" => generic_struct(),
        "multiple_generic_params" => multiple_generic_params(),
        "generic_methods" => generic_methods(),
        "generic_enums" => generic_enums(),
        "trait_bounds" => trait_bounds(),
        "where_clause" => where_clause(),
        "generic_associated_types" => generic_associated_types(),
        "const_generics" => const_generics(),
        _ => println!("Example '{}' not found", name),
    }
}

fn generic_function() {
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));
}

fn generic_struct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);
}

fn multiple_generic_params() {
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    let pair1 = Pair {
        first: 5,
        second: "hello",
    };
    let pair2 = Pair {
        first: 1.5,
        second: 'c',
    };

    println!("Pair 1: {:?}", pair1);
    println!("Pair 2: {:?}", pair2);
}

fn generic_methods() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point {
        x: 3.0_f32,
        y: 4.0_f32,
    };
    println!("x coordinate: {}", p.x());
    println!("Distance: {}", p.distance_from_origin());
}

fn generic_enums() {
    enum MyOption<T> {
        Some(T),
        None,
    }

    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    let _some_number = MyOption::Some(5);
    let _some_string = MyOption::Some("hello");
    let _no_value: MyOption<i32> = MyOption::None;

    let _success: MyResult<i32, String> = MyResult::Ok(200);
    let _failure: MyResult<i32, String> = MyResult::Err("Error!".to_string());

    println!("Created generic enum instances");
}

fn trait_bounds() {
    fn print_debug<T: std::fmt::Debug>(item: T) {
        println!("{:?}", item);
    }

    fn compare<T: PartialOrd>(a: T, b: T) -> bool {
        a > b
    }

    print_debug(42);
    print_debug("hello");
    println!("5 > 3: {}", compare(5, 3));
    println!("'a' > 'z': {}", compare('a', 'z'));
}

fn where_clause() {
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: std::fmt::Display + Clone,
        U: Clone + std::fmt::Debug,
    {
        println!("t: {}", t);
        println!("u: {:?}", u);
        42
    }

    let result = some_function(&"hello", &vec![1, 2, 3]);
    println!("Function returned: {}", result);
}

fn generic_associated_types() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn mixup<U>(self, other: Point<U>) -> Point<(T, U)> {
            Point {
                x: (self.x, other.x),
                y: (self.y, other.y),
            }
        }
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: "World",
    };
    let p3 = p1.mixup(p2);

    println!("p3.x: ({}, {})", (p3.x).0, (p3.x).1);
    println!("p3.y: ({}, {})", (p3.y).0, (p3.y).1);
}

fn const_generics() {
    struct ArrayPair<T, const N: usize> {
        left: [T; N],
        right: [T; N],
    }

    impl<T: Default + Copy, const N: usize> ArrayPair<T, N> {
        fn new() -> Self {
            ArrayPair {
                left: [T::default(); N],
                right: [T::default(); N],
            }
        }
    }

    let _pair: ArrayPair<i32, 3> = ArrayPair::new();
    println!("Created array pair with size 3");

    let _pair2: ArrayPair<f64, 5> = ArrayPair::new();
    println!("Created array pair with size 5");
}
