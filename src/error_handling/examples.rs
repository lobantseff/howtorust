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
            commentary: r#"📚 INTRODUCTION
Result<T, E> is Rust's foundational type for handling recoverable errors, an enum with two variants: Ok(T) for successful computation and Err(E) for failure. It makes error handling explicit in function signatures, forcing you to acknowledge and handle potential failures at compile time.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Unlike exception-based languages (Java, Python, JavaScript) where errors can be thrown from anywhere and propagate invisibly through the call stack, Result makes errors explicit in the type signature. When you see Result<String, ParseError>, you know immediately this function can fail and must be handled. This eliminates bugs from:
- Silently ignored errors (unlike Go's error returns which can be ignored)
- Forgotten error handling (unlike try-catch blocks that can be omitted)
- Unexpected exceptions crashing production code
- Hidden control flow from invisible stack unwinding

🔍 IMPORTANT DETAILS & INTRICACIES
Result is simply an enum: enum Result<T, E> { Ok(T), Err(E) }. The compiler enforces that you cannot access the Ok value without first checking whether the Result is Ok or Err, typically through pattern matching. This compile-time guarantee prevents using error results as if they were successful. The ? operator provides ergonomic error propagation, automatically returning early on Err while unwrapping Ok values. Result integrates with the type system to enable error composition through From trait implementations and combinators like map, and_then, and map_err.

💼 WHERE IT'S MOST USED
Result appears everywhere fallible operations occur: file I/O (fs::read_to_string), network requests (HTTP clients), parsing (str::parse), database queries, validation logic, and configuration loading. Common pattern: define domain-specific error types and return Result<SuccessType, CustomError>. Libraries use Result in APIs to force error handling, while applications use it to track success/failure through business logic layers.

✅ TAKEAWAY
Result makes errors visible values in the type system rather than exceptional control flow, giving you compile-time guarantees that errors are handled while allowing transformation, composition, and propagation using standard patterns. Unlike exceptions which invisibly unwind the stack or error codes which can be ignored, Result keeps error handling explicit and in normal control flow, eliminating entire classes of runtime failures at the cost of slightly more verbose code that's offset by ergonomic tools like the ? operator."#,
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
            commentary: r#"📚 INTRODUCTION
Option<T> is Rust's solution to the billion-dollar mistake of null pointers, an enum encoding presence (Some(T)) or absence (None) directly in the type system. Instead of allowing any reference to be null, Rust requires explicit marking of values that might not exist, eliminating null pointer exceptions entirely.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
In languages with null (Java, C++, JavaScript, Python), any reference might be null, an invisible possibility that causes crashes when forgotten. Option makes the "maybe absent" state explicit and compiler-verified. This prevents:
- Null pointer exceptions (eliminated entirely in safe Rust)
- Defensive null checks scattered throughout code
- Documentation-only indication of nullable values
- Runtime crashes from forgotten null checks
Unlike languages where null is implicit and unchecked, Option forces acknowledgment of absence at compile time through the type system.

🔍 IMPORTANT DETAILS & INTRICACIES
Option is an enum: enum Option<T> { Some(T), None }. Because Option<T> is a distinct type from T, you cannot use Option<String> where String is expected - the compiler catches this. The type system enforces handling both cases before accessing the value. Option is so fundamental that Some and None are in the prelude (no import needed). The ? operator works with Option for early returns. Internally, Option<&T> and Option<Box<T>> have the same size as the pointer itself through null pointer optimization - the compiler uses the null bit pattern to represent None, making it zero-cost.

💼 WHERE IT'S MOST USED
Option appears everywhere values might be absent: HashMap::get returns Option<&V>, vector.first() returns Option<&T>, string.find() returns Option<usize>, Iterator::next returns Option<Item>, and optional configuration fields. Common pattern: use combinators (map, and_then, filter) for transformations and pattern matching for consumption. Used extensively in APIs to represent optional parameters or results where "not found" is expected.

✅ TAKEAWAY
Option makes the possibility of absence explicit in the type system, moving what would be runtime null checks in other languages to compile-time verification, eliminating null pointer exceptions entirely. Unlike languages where null is implicit and every reference might crash your program, Option forces conscious handling through pattern matching, combinators, or the ? operator, trading slight verbosity for complete safety while remaining zero-cost through compiler optimizations that use the null bit pattern to represent None."#,
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
            commentary: r#"📚 INTRODUCTION
unwrap() and expect(msg) are methods that extract values from Result or Option by panicking if the value is Err or None. unwrap() panics with a generic message, while expect() includes a custom panic message. They're essentially assertions expressing programmer confidence that the value will be Ok/Some.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
unwrap() provides an escape hatch from Rust's explicit error handling for situations where failure is genuinely unrecoverable or during prototyping. Unlike languages with exceptions where errors can be silently ignored, Rust forces you to handle errors, but sometimes you need to say "this cannot fail, crash if I'm wrong." This prevents:
- Verbose error handling during prototyping/exploration
- Complex error propagation for truly unrecoverable errors
- Silent failures where crashing is safer (invalid startup config)
unwrap() is intentionally short to type for quick iteration, with the expectation you'll replace it with proper handling later. expect() adds documentation value by explaining why failure is impossible.

🔍 IMPORTANT DETAILS & INTRICACIES
Technically, unwrap() pattern matches: match result { Ok(v) => v, Err(_) => panic!() }. Panicking unwinds the stack and terminates the current thread (or aborts the process if panic=abort is set). The panic is not recoverable in normal control flow (except with catch_unwind, which is rarely used). expect() works identically but includes your message in the panic output. Key: the compiler doesn't warn about unwrap(), so it's easy to accidentally leave them in production code. Panicking in destructors during unwinding causes abort, so be careful with unwrap() in Drop implementations.

💼 WHERE IT'S MOST USED
Four appropriate contexts: (1) Tests where panics indicate test failure, (2) Examples/prototypes where error handling obscures the main point, (3) Development for rapid iteration before adding proper error handling, (4) When failure is truly impossible (parsing hardcoded constants, accessing known-valid indices). In production, prefer pattern matching, the ? operator, unwrap_or for defaults, or ok_or to convert to Result.

✅ TAKEAWAY
unwrap() and expect() are deliberate escape hatches from Rust's explicit error handling, trading safety for convenience by panicking on Err/None rather than forcing you to handle errors. Use them primarily for prototyping, tests, and truly unrecoverable failures where crashing is safer than continuing, but systematically replace them with proper error handling in production code since the compiler won't warn you about their presence and each unwrap() is a potential crash waiting to happen."#,
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
            commentary: r#"📚 INTRODUCTION
The ? operator is Rust's syntactic sugar for error propagation. Applied to Result<T, E>, it unwraps Ok values to continue computation or immediately returns Err from the enclosing function, transforming verbose match expressions into concise, readable code while maintaining type safety.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Before ?, every fallible operation required explicit pattern matching or try! macro, creating deeply nested or verbose code. The ? operator makes error propagation as concise as throwing exceptions in other languages while keeping errors explicit. It solves:
- Verbose boilerplate from manual error handling at every call site
- Nested match expressions obscuring the happy path
- Error conversion at function boundaries (automatic via From trait)
Unlike exceptions that invisibly unwind the stack, ? makes propagation visible in source code while the compiler ensures return type compatibility, balancing explicitness with ergonomics.

🔍 IMPORTANT DETAILS & INTRICACIES
The ? operator expands to: match expr { Ok(val) => val, Err(err) => return Err(err.into()) }. The crucial detail is the automatic .into() call using the From trait. If your function returns Result<T, AppError> and you use ? on Result<T, ParseError>, Rust automatically converts ParseError to AppError if From<ParseError> is implemented for AppError. The ? operator only works in functions returning Result, Option, or types implementing Try trait. Mixing Result and Option requires conversion with ok_or/ok_or_else. The compiler checks type compatibility at compile time.

💼 WHERE IT'S MOST USED
The ? operator becomes the default error handling mechanism in Rust codebases. You'll see chains of fallible operations: fs::read_to_string(path)?.parse()?.validate()?. Common pattern: use ? throughout a function for error propagation and only handle errors at top level or system boundaries. Used extensively in I/O operations, parsing, network code, database queries, and anywhere errors need to bubble up through multiple layers.

✅ TAKEAWAY
The ? operator transforms error propagation from verbose match expressions into concise, readable code while maintaining Rust's explicit error handling philosophy through visible syntax and compile-time type checking. It automatically unwraps Ok values or returns Err early, using the From trait to convert between error types transparently, making error handling as convenient as exceptions but with the safety and explicitness of values, allowing you to write clean linear code that reads like the happy path while properly propagating all errors."#,
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
            commentary: r#"📚 INTRODUCTION
Option combinators provide a functional programming style for transforming potentially absent values without explicit pattern matching. Methods like map, and_then, or, filter, and flatten form a composable algebra for handling optional values, enabling transformation pipelines that automatically propagate None while transforming Some values.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Combinators eliminate verbose pattern matching when transforming optional values, making data transformation pipelines explicit and readable. They solve:
- Nested match expressions when chaining optional operations
- Repetitive boilerplate from manual Some/None handling
- Visual noise obscuring the transformation logic
- Option<Option<T>> nesting (and_then flattens automatically)
Unlike exceptions that invisibly propagate or imperative null checks scattered throughout code, combinators keep the None possibility explicit while making transformations concise and composable, inspired by functional languages like Haskell's Maybe monad but with accessible method syntax.

🔍 IMPORTANT DETAILS & INTRICACIES
Combinators maintain Option context during transformations. map(f) on Some(x) produces Some(f(x)); on None, it produces None without calling f (automatic short-circuiting). and_then prevents nesting by flattening Option<Option<T>> to Option<T>, essential for chaining fallible operations. or provides fallbacks, returning the first Some. filter conditionally keeps values. This "railway-oriented programming" treats Some as success track and None as failure track. Combinators consume the Option, so use as_ref() or clone() to preserve ownership. Chaining: value.map(f).and_then(g).or(default).

💼 WHERE IT'S MOST USED
Use combinators for transformation pipelines: parsing configuration (file_contents.as_ref().map(parse).flatten()), data processing, working with collections (first().map(transform)), chaining optional lookups. Common patterns: map for transformations, and_then for chaining operations returning Option, or/or_else for defaults, filter for conditional inclusion, flatten for collapsing nested Options. Prefer combinators when operations are simple; use pattern matching when logic is complex or you need multiple bindings.

✅ TAKEAWAY
Option combinators provide a functional, composable approach to transforming optional values, eliminating verbose pattern matching while maintaining explicit handling of absence through automatic None propagation. They enable elegant transformation pipelines where Some values flow through transformations and None short-circuits the chain, making optional value handling as concise as working with regular values but with compile-time safety, though they consume ownership so you may need as_ref() or clone() to preserve the original Option."#,
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
            commentary: r#"📚 INTRODUCTION
Result combinators mirror Option combinators but operate on success/failure rather than presence/absence. map transforms Ok values while preserving Err, map_err transforms Err values while preserving Ok, and_then chains Result-returning operations with flattening, and or provides fallback Results.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Result combinators eliminate verbose pattern matching when transforming success or error values, enabling clean transformation pipelines that handle both tracks independently. They solve:
- Nested match expressions when chaining fallible operations
- Error type conversion across abstraction boundaries (map_err)
- Result<Result<T, E>, E> nesting (and_then flattens)
- Repetitive boilerplate for error propagation and transformation
Unlike exception-based languages where error transformation happens in catch blocks separate from main logic, Result combinators keep success and error transformations inline and composable, following the "railway-oriented programming" pattern.

🔍 IMPORTANT DETAILS & INTRICACIES
Combinators leverage pattern matching internally: map(f) on Ok(x) produces Ok(f(x)), on Err(e) produces Err(e) unchanged. map_err is Result-specific (no Option equivalent), enabling error type conversion essential for abstraction boundaries. and_then flattens Result<Result<T, E>, E> to Result<T, E>, crucial for chaining fallible operations. or provides fallback attempts. Combinators consume the Result, so use as_ref() to preserve ownership. Chaining: result.map(transform).map_err(add_context).and_then(validate).or(fallback).

💼 WHERE IT'S MOST USED
Use combinators for transformation pipelines: parsing and validation (input.parse().map(validate).and_then(process)), error type conversion across layers (db_call().map_err(|e| AppError::Database(e))), adding error context, trying alternatives with or/or_else. Common patterns: map for success transformations, map_err for error type conversion, and_then for chaining Result-returning operations, or for fallbacks. Prefer combinators for simple transformations; use pattern matching for complex logic.

✅ TAKEAWAY
Result combinators provide functional composition for transforming both success and error values, eliminating verbose pattern matching while maintaining type safety through a railway-oriented approach where Ok and Err flow through separate transformation tracks. Unlike exception-based error handling where transformations happen in catch blocks, combinators keep all transformations inline and composable, with map_err uniquely enabling error type conversion essential for propagating errors across abstraction boundaries while and_then flattens nested Results to enable clean chaining of fallible operations."#,
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
            commentary: r#"📚 INTRODUCTION
Custom error types encode domain-specific error conditions in the type system using enums with variants representing different failure modes. They provide type-safe, exhaustive error handling by making each error case explicit and matchable, replacing generic strings or standard library errors with precise, meaningful types.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Custom error types make error handling precise and verifiable at compile time. They prevent:
- Generic Result<T, String> errors that lose type information
- Forgotten error cases (compiler enforces exhaustive matching)
- Runtime string parsing to determine error types
- Accidental suppression of unexpected errors (unlike catch-all exception handlers)
Unlike Go's string errors or Java's generic exceptions, Rust's custom error enums provide compile-time guarantees about which errors can occur from which functions, making error handling explicit and self-documenting while preventing impossible states through the type system.

🔍 IMPORTANT DETAILS & INTRICACIES
Define enums with variants for failure modes: enum MathError { DivisionByZero, NegativeSquareRoot }. Derive Debug for developer output, implement Display for user-facing messages using pattern matching in fmt(). Use tuple variants for context: FileError::NotFound(PathBuf). For libraries, implement std::error::Error trait (requires Display + Debug) to enable error chaining with source() and ecosystem compatibility. Implement From<SourceError> for automatic conversion with ?. Each variant can carry associated data, making errors informative while remaining type-safe.

💼 WHERE IT'S MOST USED
Design error types per domain: parsers use ParseError with UnexpectedToken/UnexpectedEof/InvalidSyntax variants, network code uses NetworkError with ConnectionRefused/Timeout/InvalidResponse, file I/O uses FileError with NotFound/PermissionDenied/AlreadyExists. Start with one error type per module, split if complexity grows. Libraries expose error types in public APIs, applications use them internally and at layer boundaries. Pattern: return Result<SuccessType, DomainError> from all fallible functions.

✅ TAKEAWAY
Custom error types move error handling from runtime string parsing to compile-time type checking by encoding domain-specific failures as enum variants, giving you exhaustive pattern matching and precise error information instead of generic catch-all types. Unlike exception hierarchies that allow catch-all handlers to suppress unexpected errors, Rust's error enums force you to handle every variant explicitly, making impossible states unrepresentable and creating self-documenting, maintainable code where the type signature tells you exactly what can go wrong, though this requires more upfront design than generic errors."#,
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
            commentary: r#"📚 INTRODUCTION
Implementing From trait for error types enables automatic error conversion when using the ? operator. When From<SourceError> is implemented for TargetError, the ? operator transparently converts between error types during propagation, eliminating manual conversion at every call site while maintaining type safety.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
From implementations create clean error boundaries between system layers, allowing unified error types while calling functions with different error types. They solve:
- Manual error wrapping boilerplate at every call site
- Noise that obscures business logic with conversion code
- Type mismatches when propagating errors across layers
- Loss of type information from generic error handling
Unlike exception hierarchies that use subtyping (losing type safety with catch-all handlers), Rust's From trait provides explicit conversions in type definitions while keeping call sites clean, maintaining full type information through directed conversion graphs.

🔍 IMPORTANT DETAILS & INTRICACIES
The ? operator desugars to: match expr { Ok(v) => v, Err(e) => return Err(e.into()) }. The .into() calls your From implementation. When fallible_call() returns Result<T, ParseError> in a function returning Result<T, AppError>, the compiler uses From<ParseError> for AppError to convert automatically. Pattern: wrap source errors in enum variants: impl From<IoError> for AppError { fn from(err: IoError) -> Self { AppError::Io(err) } }. This preserves the original error for specific handling while unifying the error type.

💼 WHERE IT'S MOST USED
Implement From for each external error type your layer encounters. If business logic calls database and parsing code, implement From<DbError> and From<ParseError> for AppError, creating unified layer-specific error types. Common in multi-layer applications: presentation layer has PresentationError with From<BusinessError>, business layer has BusinessError with From<DbError> and From<ValidationError>. Pattern: preserve source errors in tuple variants for debugging and specific error handling downstream.

✅ TAKEAWAY
From implementations enable the ? operator to automatically convert between error types, creating clean abstraction boundaries where each layer has a unified error type but can call functions with different error types without manual conversion at call sites. Unlike exception hierarchies that use runtime polymorphism and lose type safety, From provides compile-time verified conversions defined explicitly in type definitions, maintaining full type information while keeping usage code clean, though you should implement From judiciously since it creates implicit conversions that can make error flow less obvious."#,
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
            commentary: r#"📚 INTRODUCTION
Box<dyn Error> is Rust's dynamic error type that can hold any error implementing std::error::Error trait. The Box provides heap allocation, while dyn Error uses trait objects for runtime polymorphism, allowing heterogeneous error types to be returned from the same function at the cost of type precision.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Box<dyn Error> provides maximum flexibility when specific error types matter less than knowing failure occurred, or when aggregating many error sources would create unwieldy custom enums. It solves:
- Custom error enum explosion across many error sources
- Verbosity of maintaining precise error types in application code
- Complexity when errors are logged rather than programmatically handled
- Boilerplate when prototyping or writing scripts
Unlike languages where exceptions are dynamically typed by default (Java) or errors are just interfaces over strings (Go), Rust's Box<dyn Error> is opt-in dynamic typing that preserves error information through the Error trait's source() method.

🔍 IMPORTANT DETAILS & INTRICACIES
Box<dyn Error> is a fat pointer (two pointers): one to heap-allocated error data, one to vtable for Error trait methods. This enables runtime polymorphism - different error types return through the same Result<T, Box<dyn Error>> type with dynamic dispatch. Boxing is necessary because error types have different sizes; the compiler needs fixed size. Tradeoffs: heap allocation cost, dynamic dispatch overhead, loss of compile-time type information (can't pattern match without downcasting via downcast_ref()). The ? operator works seamlessly since most stdlib errors implement Error with automatic conversion.

💼 WHERE IT'S MOST USED
Use Box<dyn Error> in applications, scripts, prototypes, main functions, CLI tools where errors are logged/displayed rather than programmatically handled. Perfect when you want quick error propagation without custom error ceremony. Libraries should use specific error types so consumers can handle errors differently. Pattern: use Box<dyn Error> internally, convert to specific types at API boundaries. Pair with anyhow crate for applications (adds context) or thiserror for libraries (maintains type safety).

✅ TAKEAWAY
Box<dyn Error> trades Rust's static typing for pragmatic flexibility through runtime polymorphism, allowing any Error-implementing type to be returned from the same function without custom error enum maintenance, making it perfect for applications and scripts where errors are logged rather than programmatically handled. However, this flexibility costs heap allocation, dynamic dispatch, and loss of compile-time type information that prevents pattern matching on specific error types, so libraries should use specific error types while applications can embrace Box<dyn Error>'s convenience for simpler error propagation across many error sources."#,
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
            commentary: r#"📚 INTRODUCTION
The early return pattern uses guard clauses that return Err immediately when validation fails, keeping the happy path at base indentation level. Each validation check stands alone: if something is wrong, return the error immediately, otherwise continue, creating linear, top-to-bottom code with validation concentrated at the beginning.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Early returns prevent nested if-else blocks that obscure the happy path, making validation explicit and linear. They solve:
- Deeply nested validation creating indentation pyramids
- Hidden happy path buried in nested blocks
- Complex control flow from accumulated validation flags
- Mixed validation and processing logic
Unlike exception-based validation where throws can bypass cleanup code (requiring try-finally), Rust's early returns stay in normal control flow, returning error values without invisible stack unwinding. This is more explicit than C's error codes (which can be ignored) and safer than exception-based approaches.

🔍 IMPORTANT DETAILS & INTRICACIES
Implementation: functions return Result<T, E>, each validation uses if condition returning Err(...) on failure. The return keyword exits immediately from anywhere, making guard clauses natural. Unlike exceptions, this remains in normal control flow - no stack unwinding, no catch blocks, just explicit error values. The pattern combines beautifully with ? operator - validation functions can call other validators and propagate errors concisely. Structure: validation guards at top, happy path at base indentation, making control flow visually obvious.

💼 WHERE IT'S MOST USED
Use for validation-heavy functions: HTTP request handlers (validate headers, auth, body), command parsers (validate syntax, arguments), data validators (validate fields, ranges, formats), API entry points. Structure: place all validation guards at function start with early returns, then unindented happy path processes valid data. Each guard becomes self-documenting: if value < 0, return error. Common in any function with multiple preconditions before processing.

✅ TAKEAWAY
Early return pattern keeps validation linear and explicit by returning Err immediately on failure rather than nesting conditions, placing the happy path at base indentation where it's visually prominent while validation guards at the top clearly state all preconditions. Unlike exception-based approaches with invisible stack unwinding or nested if-else pyramids that obscure logic, early returns create top-to-bottom readable code where each guard says "if this is wrong, we're done" in normal control flow, making validation self-documenting and maintainable while combining naturally with the ? operator for calling other validators."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
