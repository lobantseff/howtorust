// Closure examples organized in a separate file

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_closure",
            description: "Creating and calling a simple closure",
            code: r#"let add = |a, b| a + b;
let result = add(5, 3);
println!("5 + 3 = {}", result);

let greet = |name| format!("Hello, {}!", name);
println!("{}", greet("Alice"));"#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "closure_type_inference",
            description: "Type inference in closures",
            code: r#"// Types are inferred from usage
let multiply = |x, y| x * y;
println!("10 * 5 = {}", multiply(10, 5));

// Can also specify types explicitly
let divide = |x: f64, y: f64| -> f64 { x / y };
println!("10.0 / 3.0 = {}", divide(10.0, 3.0));"#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "capturing_environment",
            description: "Closures capturing variables from their environment",
            code: r#"let x = 10;
let y = 20;

let add_to_x = |a| a + x;
let add_both = |a| a + x + y;

println!("5 + x = {}", add_to_x(5));
println!("5 + x + y = {}", add_both(5));"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "fn_traits",
            description: "Three closure traits: Fn, FnMut, FnOnce",
            code: r#"// Fn: borrows immutably
let x = vec![1, 2, 3];
let print_vec = || println!("Vec: {:?}", x);
print_vec();
print_vec();  // Can call multiple times

// FnMut: borrows mutably
let mut count = 0;
let mut increment = || {
    count += 1;
    println!("Count: {}", count);
};
increment();
increment();

// FnOnce: takes ownership
let s = String::from("hello");
let consume = || {
    let _owned = s;
    println!("Consumed string");
};
consume();
// consume();  // Error: can only call once!"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "move_keyword",
            description: "Using move to force ownership transfer",
            code: r#"let x = vec![1, 2, 3];

// move forces closure to take ownership
let consume = move || {
    println!("Vec: {:?}", x);
};

consume();
// println!("{:?}", x);  // Error: x was moved

// Useful for threads
let y = String::from("Hello");
let closure = move || {
    println!("{}", y);
};
closure();"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "closures_as_parameters",
            description: "Passing closures as function parameters",
            code: r#"fn apply_operation<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let double = |n| n * 2;
let square = |n| n * n;

println!("Double 5: {}", apply_operation(5, double));
println!("Square 5: {}", apply_operation(5, square));

// Can also use inline closures
println!("Add 10: {}", apply_operation(5, |n| n + 10));"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "returning_closures",
            description: "Returning closures from functions",
            code: r#"fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let add_5 = make_adder(5);
let add_10 = make_adder(10);

println!("7 + 5 = {}", add_5(7));
println!("7 + 10 = {}", add_10(7));"#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "closure_caching",
            description: "Caching expensive computations with closures",
            code: r#"struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

let mut expensive = Cacher::new(|x| {
    println!("Calculating...");
    x * 2
});

println!("First call: {}", expensive.value(10));
println!("Second call: {}", expensive.value(10));  // Uses cached"#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "closure_composition",
            description: "Composing multiple closures",
            code: r#"fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

let add_one = |x: i32| x + 1;
let double = |x: i32| x * 2;

let add_then_double = compose(add_one, double);
let double_then_add = compose(double, add_one);

println!("(5 + 1) * 2 = {}", add_then_double(5));
println!("(5 * 2) + 1 = {}", double_then_add(5));"#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
