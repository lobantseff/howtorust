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
            commentary: r#"Closures are anonymous functions that can capture their environment. They use the |param| syntax
instead of fn. Unlike regular functions, closures can infer parameter and return types from usage,
making them concise for short operations.

From The Rust Book (Chapter 13.1):
"Closures are anonymous functions you can save in a variable or pass as arguments to other functions."

Closures are commonly used with iterator methods like map, filter, and for_each."#,
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
            commentary: r#"Closures have powerful type inference - the compiler determines types from how you use them.
Once inferred, the types are locked in for that closure. You can explicitly annotate types if
needed, especially when the closure is stored or passed around.

From The Rust Book (Chapter 13.1):
"Closure definitions will have one concrete type inferred for each of their parameters and for
their return value."

Common gotcha: Each closure has a unique type, even if they have identical signatures."#,
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
            commentary: r#"Unlike functions, closures can capture variables from their enclosing scope. The closure stores
a reference to these variables and can use them. This is what makes closures "close over" their
environment, hence the name.

From The Rust Book (Chapter 13.1):
"Closures can capture values from their environment in three ways: borrowing immutably, borrowing
mutably, and taking ownership."

The compiler automatically chooses the least restrictive capture method based on usage."#,
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
            commentary: r#"Closures implement one or more of three traits based on what they capture. Fn captures immutably
and can be called multiple times. FnMut captures mutably and needs &mut self. FnOnce takes ownership
and can only be called once. All closures implement FnOnce, and Fn closures also implement FnMut.

From The Rust Book (Chapter 13.1):
"The way a closure captures and handles values from its environment affects which traits the closure
implements, and traits are how functions and structs can specify what kinds of closures they can use."

When accepting closures as parameters, prefer Fn when possible for maximum flexibility."#,
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
            commentary: r#"The move keyword forces a closure to take ownership of captured variables instead of borrowing.
This is essential for closures that outlive their environment, such as those passed to threads.
Without move, the closure would borrow, which isn't safe across thread boundaries.

From The Rust Book (Chapter 13.1):
"The move keyword forces the closure to take ownership of the values it uses in the environment."

Common use case: Spawning threads that need to own their data since the parent thread may end first."#,
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
            commentary: r#"Functions can accept closures as parameters using generic type parameters with trait bounds.
The F: Fn(i32) -> i32 bound means "F is any type that implements Fn with one i32 parameter and
returns i32." This enables powerful functional programming patterns.

From The Rust Book (Chapter 13.1):
"We can create functions that accept closures by specifying a generic type parameter with a trait
bound for one of the Fn traits."

This is how iterator methods like map and filter work - they accept closures as arguments."#,
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
            commentary: r#"Functions can return closures using impl Trait syntax. The move keyword is necessary because
the returned closure must own its captured data - it outlives the function that created it.
This pattern creates function factories or partial application.

From The Rust Book (Chapter 13.1):
"Because closures are represented using traits, you can't return closures directly. In most cases
where you might want to return a trait, you can instead use impl Trait."

This enables powerful patterns like currying and creating configurable behavior."#,
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
            commentary: r#"Storing closures in structs enables patterns like memoization and lazy evaluation. The struct
holds the closure and cached results. This is useful for expensive computations that might not
be needed or should only run once.

From The Rust Book (Chapter 13.1):
"We can create a struct that will hold the closure and the resulting value. The struct will execute
the closure only if we need the resulting value."

Common gotcha: This simple cacher only works for one input - real implementations use HashMap."#,
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
            commentary: r#"Function composition creates a new function by combining two functions - the output of one becomes
the input of the next. This demonstrates advanced generic programming with closures, using three
type parameters to connect the input, intermediate, and output types.

This pattern is common in functional programming and enables building complex transformations from
simple building blocks. The compose function returns a closure that captures both input closures.

Note: Order matters - compose(f, g) means "do f first, then g" which reads as g(f(x))."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
