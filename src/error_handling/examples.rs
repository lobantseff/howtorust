// Error handling examples organized in a separate file

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_result",
            description: "Using Result for recoverable errors",
            code: r#"fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}

match divide(10.0, 0.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}"#,
            commentary: r#"Result<T, E> is Rust's primary error handling type for recoverable errors. Ok(value) represents
success, Err(error) represents failure. This forces you to handle errors explicitly through pattern
matching, making error handling visible and compiler-checked.

From The Rust Book (Chapter 9.2):
"Result enum is defined as having two variants, Ok and Err. The Ok variant indicates the operation
was successful, and inside Ok is the successfully generated value."

Unlike exceptions, Result makes error handling explicit in the type system."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "basic_option",
            description: "Using Option for values that might be absent",
            code: r#"fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

match find_user(1) {
    Some(name) => println!("Found user: {}", name),
    None => println!("User not found"),
}

match find_user(999) {
    Some(name) => println!("Found user: {}", name),
    None => println!("User not found"),
}"#,
            commentary: r#"Option<T> represents a value that might be absent - Some(value) or None. This replaces null
from other languages, making it explicit when a value might not exist. The compiler forces you to
handle the None case, eliminating null pointer errors.

From The Rust Book (Chapter 6.1):
"Rust doesn't have the null feature that many other languages have. The Option<T> enum encodes the
very common scenario in which a value could be something or it could be nothing."

Option is so common that Some and None are in the prelude - you can use them without Option:: prefix."#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "unwrap_and_expect",
            description: "Using unwrap and expect",
            code: r#"let some_value: Option<i32> = Some(42);
let value = some_value.unwrap();
println!("Value: {}", value);

let result: Result<i32, &str> = Ok(100);
let num = result.expect("Failed to get number");
println!("Number: {}", num);

// Be careful: these panic on None/Err!
// let none_value: Option<i32> = None;
// none_value.unwrap();  // This would panic!"#,
            commentary: r#"unwrap() extracts the value or panics if None/Err. expect() is similar but lets you provide a
custom panic message. Use these when you're absolutely certain the value will be Some/Ok, or in
prototypes. In production code, prefer proper error handling.

From The Rust Book (Chapter 9.2):
"If the Result value is the Err variant, unwrap will call the panic! macro for us."

Common gotcha: Overusing unwrap can make your code fragile. Use it when failure is truly unrecoverable or in tests."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "question_mark_operator",
            description: "Using ? operator for error propagation",
            code: r#"fn read_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = text.parse()?;
    Ok(num * 2)
}

match read_number("42") {
    Ok(n) => println!("Result: {}", n),
    Err(e) => println!("Error: {}", e),
}

match read_number("not a number") {
    Ok(n) => println!("Result: {}", n),
    Err(e) => println!("Error: {}", e),
}"#,
            commentary: r#"The ? operator is shorthand for error propagation. If the value is Ok, it unwraps it. If Err,
it returns early from the function with that error. This makes error handling concise while remaining
explicit. The ? operator can only be used in functions that return Result or Option.

From The Rust Book (Chapter 9.2):
"The ? operator can only be used in functions that have a return type compatible with the value the
? is used on."

The ? operator also converts error types automatically using From trait implementation."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "option_combinators",
            description: "Using Option combinators (map, and_then, etc.)",
            code: r#"let some_string = Some("42");

let doubled = some_string
    .map(|s| s.parse::<i32>().ok())
    .flatten()
    .map(|n| n * 2);

println!("Doubled: {:?}", doubled);

let result = Some(5)
    .and_then(|n| if n > 0 { Some(n * 2) } else { None })
    .or(Some(0));

println!("Result: {:?}", result);"#,
            commentary: r#"Option provides combinators for functional-style error handling without explicit pattern matching.
map transforms the inner value if Some. and_then chains operations that return Option. or provides
a fallback value. These compose nicely for complex transformations.

From The Rust Book (Chapter 9.3):
"These methods can make your code more concise than using match expressions everywhere."

Combinators enable a "railway-oriented programming" style where None short-circuits the chain."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "result_combinators",
            description: "Using Result combinators",
            code: r#"let result: Result<i32, &str> = Ok(10);

let doubled = result.map(|n| n * 2);
println!("Doubled: {:?}", doubled);

let error_result: Result<i32, &str> = Err("oops");
let with_default = error_result.or(Ok(0));
println!("With default: {:?}", with_default);

let mapped_err = error_result.map_err(|e| format!("Error: {}", e));
println!("Mapped error: {:?}", mapped_err);"#,
            commentary: r#"Result has similar combinators to Option. map transforms Ok values. map_err transforms Err values.
or provides a fallback Result. and_then chains Result-returning operations. These avoid nested
match statements and make error handling pipelines readable.

From The Rust Book (Chapter 9.2):
"Methods like map are defined on many types in Rust, not just Result."

Use map_err to convert between error types when propagating errors through different layers."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "custom_error_types",
            description: "Creating custom error types",
            code: r#"use std::fmt;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative"),
        }
    }
}

fn divide_checked(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

match divide_checked(10.0, 0.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}"#,
            commentary: r#"Custom error types provide domain-specific error information. Enums are perfect for errors with
distinct variants. Implementing Display and Debug makes them user-friendly. This is more type-safe
and informative than using strings for errors.

From The Rust Book (Chapter 9.2):
"Creating your own types for use with Result is encouraged."

For library code, also implement std::error::Error trait to integrate with the error ecosystem."#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "error_conversion",
            description: "Converting between error types with From",
            code: r#"use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    Custom(String),
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ParseError(err)
    }
}

fn process_number(s: &str) -> Result<i32, AppError> {
    let num: i32 = s.parse()?;  // Automatically converts ParseIntError to AppError
    if num < 0 {
        Err(AppError::Custom("Negative numbers not allowed".into()))
    } else {
        Ok(num)
    }
}

match process_number("42") {
    Ok(n) => println!("Number: {}", n),
    Err(e) => println!("Error: {:?}", e),
}"#,
            commentary: r#"Implementing From trait enables automatic error type conversion when using ?. The ? operator calls
From::from to convert the error type. This allows propagating errors from different sources through
a unified error type without manual conversion.

From The Rust Book (Chapter 9.2):
"The ? operator automatically converts the error type using the From trait."

This pattern is essential for libraries that need to wrap multiple underlying error types."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "multiple_error_types",
            description: "Handling multiple error types with Box<dyn Error>",
            code: r#"use std::error::Error;
use std::fs;

fn read_and_parse(filename: &str) -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

// Simulating without file I/O for demo
fn parse_demo(text: &str) -> Result<i32, Box<dyn Error>> {
    let num: i32 = text.trim().parse()?;
    Ok(num)
}

match parse_demo("123") {
    Ok(n) => println!("Parsed: {}", n),
    Err(e) => println!("Error: {}", e),
}"#,
            commentary: r#"Box<dyn Error> is a catch-all error type that can hold any error implementing the Error trait.
This is convenient for application code that doesn't need precise error types. The tradeoff is losing
type information about specific errors.

From The Rust Book (Chapter 9.2):
"Using Box<dyn Error> means we can return errors of different types from the same function."

Use this for quick prototypes or applications. Libraries should use specific error types."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "early_return_pattern",
            description: "Early returns for cleaner error handling",
            code: r#"fn validate_and_process(value: i32) -> Result<i32, String> {
    if value < 0 {
        return Err("Value must be non-negative".into());
    }

    if value > 100 {
        return Err("Value must be <= 100".into());
    }

    if value % 2 != 0 {
        return Err("Value must be even".into());
    }

    Ok(value * 2)
}

for val in [10, -5, 150, 7, 20] {
    match validate_and_process(val) {
        Ok(result) => println!("{} -> {}", val, result),
        Err(e) => println!("{} -> Error: {}", val, e),
    }
}"#,
            commentary: r#"Early returns with Err make validation logic clear and avoid deep nesting. Each check returns
immediately on failure, and the happy path remains at the base indentation level. This pattern,
combined with Result, makes control flow explicit.

From The Rust Book (Chapter 9.2):
"Returning errors early is a common pattern in Rust."

This is more readable than deeply nested if-else blocks and makes each validation condition standalone."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
