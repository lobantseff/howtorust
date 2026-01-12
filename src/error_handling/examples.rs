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
            commentary: r#"Result<T, E> is Rust's foundational type for handling recoverable errors, representing either successful computation with Ok(value) or failure with Err(error). Unlike exception-based error handling in languages like Java or Python where errors can be thrown from anywhere and propagate invisibly, Result makes error handling explicit in the function signature itself. When you see a function returning Result<String, ParseError>, you immediately know it can fail and must be handled. This design philosophy eliminates entire classes of bugs where errors are silently ignored or forgotten.

The technical implementation is remarkably simple - Result is just an enum with two variants: Ok(T) for success containing the result value, and Err(E) for failure containing error information. The compiler enforces that you cannot access the Ok value without first checking whether the Result is Ok or Err, typically through pattern matching with match expressions. This compile-time guarantee means you cannot accidentally use an error result as if it were successful, preventing runtime crashes from unchecked errors.

Rust chose this design over exceptions because exceptions violate the principle of explicit control flow - they can unwind the stack invisibly, making it hard to reason about program state. Result keeps error handling in the normal flow of code, making it visible and trackable. This contrasts with Go's (value, error) tuple returns which can be ignored, and with exception-based systems where try-catch blocks can be forgotten. The tradeoff is more verbose code, but Rust provides ergonomic tools like the ? operator to reduce boilerplate while maintaining explicitness.

In practice, you'll use Result extensively for any operation that can fail: file I/O, network requests, parsing, database queries, and validation. A common pattern is to define domain-specific error types and return Result<SuccessType, CustomError>. The key insight is that errors are values, not exceptional control flow, allowing you to transform, combine, and compose them using standard functional patterns. Be careful not to unwrap() carelessly - it defeats the safety guarantees by potentially panicking."#,
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
            commentary: r#"Option<T> is Rust's solution to the billion-dollar mistake of null pointers, encoding the possibility of absence directly in the type system. Instead of allowing any reference to be null (as in Java, C++, or JavaScript), Rust requires you to explicitly mark values that might not exist using Option<T>, which is either Some(value) containing the actual value, or None representing absence. This fundamental difference means null pointer exceptions - one of the most common sources of crashes in software - simply cannot happen in safe Rust code. The compiler enforces exhaustive handling of both cases before you can use the value.

The implementation is elegant in its simplicity: Option is an enum with two variants, Some(T) wrapping the actual value and None representing nothing. Because Option is a distinct type from T, you cannot accidentally use an Option<String> where a String is expected - the type system catches these errors at compile time. This forces developers to consciously acknowledge and handle the possibility of absence, typically through pattern matching, combinators like map/and_then, or the ? operator for early returns. The safety comes from making the "maybe absent" state explicit and checkable.

Rust's design choice to eliminate null and replace it with Option reflects a philosophy of making invalid states unrepresentable. In languages with null, every reference carries an implicit "might be null" possibility that's invisible in the type signature and must be documented separately. This leads to defensive null checks scattered throughout code or, worse, forgotten checks that cause crashes. Option moves this possibility into the type system where it's visible, searchable, and compiler-verified. The tradeoff is slightly more verbose code, but Rust's ergonomic features like if let, pattern matching, and combinators make working with Option natural and expressive.

Practically speaking, Option appears everywhere in Rust: accessing hash map entries, getting the first element of a vector, parsing strings, finding items in collections, and representing optional configuration. The idiomatic approach is to use combinators for transformation (map, filter, and_then) and pattern matching for consumption. A common gotcha is calling unwrap() on Option::None, which panics - instead, use unwrap_or, unwrap_or_else, or proper pattern matching. Option is so fundamental that Some and None are imported in the prelude, allowing you to use them without qualification."#,
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
            commentary: r#"The unwrap() and expect() methods provide a quick way to extract values from Result or Option, but they come with a significant caveat: they panic if the value is Err or None, crashing the current thread. unwrap() simply panics with a generic message, while expect(msg) lets you provide a custom panic message that explains what went wrong. These methods are essentially assertions - they express the programmer's confidence that the value will be Ok/Some, and if that confidence is misplaced, the program crashes. This makes them powerful but dangerous tools that should be used judiciously.

The technical mechanism is straightforward: unwrap() pattern matches on the Result/Option and either returns the wrapped value or calls panic!, which unwinds the stack and terminates the thread. expect() does the same but includes your custom message in the panic, making debugging easier. The key insight is that panicking is a form of error handling - it's appropriate when continuing execution would be incorrect or dangerous. For example, if your web server configuration is invalid, crashing during startup is better than running with wrong settings. However, panicking is not recoverable within the normal control flow (without catch_unwind), so it's unsuitable for expected errors.

This design reflects Rust's philosophy of making error handling explicit while providing escape hatches for prototyping and genuinely unrecoverable situations. unwrap() is intentionally short and easy to type because it's meant for quick prototyping and exploration - you can scatter unwrap() calls while developing, then systematically replace them with proper error handling before production. expect() is preferred over unwrap() because the custom message documents why you believe the operation cannot fail, serving as both documentation and debugging aid when assumptions are violated.

In practice, use unwrap() and expect() primarily in four contexts: writing tests where panics indicate test failure, examples and prototypes where error handling obscures the main point, during development for rapid iteration, and when you've verified that failure is impossible (e.g., parsing a hardcoded constant string). In production code, prefer pattern matching, the ? operator, unwrap_or for defaults, or ok_or to convert to Result. A red flag is seeing many unwrap() calls in production - it suggests insufficient error handling. The compiler won't warn about unwrap(), so vigilance is required during code review."#,
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
            commentary: r#"The ? operator is Rust's elegant solution to error propagation, providing syntactic sugar that dramatically reduces boilerplate while maintaining type safety and explicit error handling. When applied to a Result<T, E>, the ? operator unwraps Ok values to extract the inner value, allowing computation to continue, or immediately returns Err from the enclosing function, propagating the error upward. This transforms what would be verbose match expressions or if-let chains into concise, readable code. The key constraint is that ? can only be used in functions whose return type is compatible - Result for Results, Option for Options, or types implementing the Try trait.

The technical implementation leverages Rust's type system: ? expands to approximately "match expr { Ok(val) => val, Err(err) => return Err(err.into()) }". The crucial detail is the automatic .into() call, which uses the From trait to convert error types. This means if your function returns Result<T, CustomError> and you use ? on something returning Result<T, ParseError>, Rust will automatically convert ParseError to CustomError if you've implemented From<ParseError> for CustomError. This automatic conversion enables ergonomic error handling across different error types without manual wrapping at each call site.

Rust introduced ? to address the verbosity of error handling in earlier versions where every fallible operation required explicit pattern matching. Prior to ?, developers used try! macro which was less readable. The design balances explicitness with ergonomics - errors are still values in the type system, not invisible exceptions, but propagating them is as concise as throwing exceptions in other languages. Unlike exceptions which can unwind from anywhere, ? makes error propagation visible in the source code, and the compiler ensures the return type is correct. This explicit visibility helps reasoning about control flow while maintaining conciseness.

In practice, ? becomes the default way to handle errors in Rust codebases. You'll see long chains of fallible operations connected with ? operators, creating clean linear code that reads like the happy path while handling all errors. A common pattern is to use ? throughout a function and only handle errors at the top level or boundaries between system components. Be aware that ? performs error conversion, which can sometimes hide type mismatches or require explicit From implementations. Also, mixing Option and Result requires conversion with ok_or or similar methods since ? doesn't automatically convert between them."#,
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
            commentary: r#"Option combinators provide a functional programming style for working with potentially absent values, enabling elegant transformation pipelines without explicit pattern matching. Methods like map, and_then, or, filter, and flatten form a rich algebra for composing optional computations. The map combinator applies a function to the wrapped value if present, transforming Option<T> to Option<U> while automatically propagating None. The and_then combinator (also called flat_map) chains operations that themselves return Option, flattening the result to avoid Option<Option<T>>. The or combinator provides fallback values, returning the first Some encountered. Together, these combinators enable concise, expressive code that handles both success and absence cases implicitly.

The technical beauty lies in how combinators maintain the Option context while transforming values. When you call map(f) on Some(x), it produces Some(f(x)); on None, it produces None without calling f. This automatic short-circuiting means None propagates through the entire chain, similar to how exceptions bubble up but without leaving the normal control flow. The and_then combinator is particularly powerful because it prevents nesting - instead of Option<Option<T>>, it flattens to Option<T>, enabling seamless chaining of multiple fallible operations. This pattern, sometimes called "railway-oriented programming," treats Some as the success track and None as the failure track, with operations staying on track or derailing to None.

This design reflects Rust's embrace of functional programming patterns alongside its systems programming heritage. Combinators originated in languages like Haskell where the Maybe monad provides similar functionality, but Rust makes them more accessible through method syntax rather than specialized operators. Compared to always using match, combinators reduce visual noise and make the data transformation pipeline explicit. The tradeoff is a learning curve - developers from imperative backgrounds may find combinators less intuitive than explicit pattern matching initially, but they typically become preferred once familiar. Unlike exceptions which are invisible until they occur, combinators make the possibility of None explicit and composable.

In practice, use combinators when building transformation pipelines or when the operation on Some is simple and self-contained. Common patterns include map for transformations, and_then for chaining fallible operations, or/or_else for defaults, filter for conditional inclusion, and flatten to collapse nested Options. For example, parsing configuration often chains: file_contents.as_ref().map(parse).flatten(). Be cautious with long chains as they can become hard to read - sometimes explicit pattern matching is clearer. Also, remember that combinators consume the Option, so you may need to clone values or use as_ref() to preserve ownership when needed."#,
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
            commentary: r#"Result combinators mirror Option's combinator pattern but operate on the success/failure dichotomy rather than presence/absence. The map combinator transforms Ok values while preserving Err unchanged, allowing value transformations without affecting error propagation. The map_err combinator does the inverse - it transforms Err values while leaving Ok untouched, enabling error type conversion and enrichment. The and_then combinator chains Result-returning operations, flattening Result<Result<T, E>, E> to Result<T, E>. The or combinator provides fallback Results, trying alternatives until one succeeds. These combinators compose into powerful error-handling pipelines that remain readable and type-safe.

The technical implementation leverages Rust's match expressions under the hood - map(f) on Ok(x) produces Ok(f(x)), while on Err(e) it passes through Err(e) unchanged. What makes Result combinators particularly powerful is map_err, which has no equivalent in Option. This allows transforming error types at any point in a chain, essential when propagating errors across abstraction boundaries. For instance, a database layer might return Result<T, DbError>, which you map_err to Result<T, AppError> before returning from a business logic layer. The and_then combinator is crucial for avoiding nested Results when chaining fallible operations - instead of Result<Result<User, ParseError>, DbError>, you get Result<User, DbError>.

Rust's Result combinator design balances functional programming elegance with systems programming practicality. While inspired by functional languages like Haskell's Either monad, Rust makes combinators more approachable through method syntax and familiar names. Compared to exception-based languages where error transformation happens in catch blocks separate from the main logic, Result combinators keep success and error transformations inline and composable. The design encourages the "railway-oriented programming" pattern where Ok is the success track and Err is the error track, with operations transforming values on each track independently.

In practice, Result combinators shine when building transformation pipelines across multiple fallible operations. Use map for transforming successful results, map_err for converting error types or adding context, and_then for chaining operations that return Results, or/or_else for trying alternatives or providing fallbacks. A common pattern is parsing and validation: input.parse().map(validate).and_then(process). When working across layers, map_err adds context: db_call().map_err(|e| AppError::Database(e)). Be mindful that combinators consume the Result, so use as_ref() when you need to preserve ownership. Also, overly long combinator chains can become unreadable - balance conciseness with clarity."#,
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
            commentary: r#"Custom error types are essential for creating robust, maintainable Rust applications by encoding domain-specific error conditions in the type system rather than using generic strings or standard library errors. Enums are the idiomatic choice because they can represent multiple distinct error variants with associated data, making impossible states unrepresentable and enabling exhaustive pattern matching. For example, a MathError enum with DivisionByZero and NegativeSquareRoot variants is far more precise and type-safe than returning Result<T, String>. Implementing Display allows errors to be formatted for user-facing messages, while Debug provides detailed information for developers during debugging.

The technical foundation of custom errors involves defining an enum with variants representing different failure modes, deriving Debug for automatic debugging support, and manually implementing Display to control how errors appear to users. The Display trait's fmt method uses pattern matching to provide appropriate messages for each variant. Some variants carry data (e.g., InvalidInput(String)), while others are unit variants when no additional context is needed. For library code, implementing the std::error::Error trait (which requires Display and Debug) allows your errors to interoperate with the broader Rust error ecosystem, enabling features like error chaining with source() and compatibility with error handling crates.

Rust's design philosophy encourages fine-grained, domain-specific error types over generic catch-all errors. This contrasts with languages like Go where errors are often just strings, or Java where developers sometimes overuse generic exceptions. Custom error types provide compile-time guarantees about which errors can occur from which functions, making error handling explicit and verifiable. The type system ensures you handle all error variants, preventing forgotten error cases. This is more rigorous than exception hierarchies in object-oriented languages, where catch-all handlers can inadvertently suppress unexpected exceptions. The tradeoff is more upfront design work, but the result is more maintainable and self-documenting code.

In practice, design error types around your domain's failure modes. For a parser, you might have ParseError with variants like UnexpectedToken, UnexpectedEof, InvalidSyntax. For network code, NetworkError with variants like ConnectionRefused, Timeout, InvalidResponse. Start simple with a single error type per module, then split into multiple types if complexity grows. Use tuple variants for errors needing context: FileError::NotFound(PathBuf). Implement From to enable automatic conversion with the ? operator. Consider using libraries like thiserror to reduce boilerplate. A common pitfall is making error types too generic or too granular - find the right balance for your domain."#,
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
            commentary: r#"Implementing the From trait for error types is the key to ergonomic error propagation in Rust, enabling automatic error conversion when using the ? operator. When you implement From<SourceError> for TargetError, the ? operator can automatically convert SourceError into TargetError during error propagation, eliminating manual conversion at every call site. This allows functions to return a unified error type while calling functions that return different error types, creating clean error boundaries between system layers. The ? operator desugars to code that calls .into() on errors, which in turn uses your From implementations to perform the conversion transparently.

The technical mechanism is elegant: when you write "let x = fallible_call()?;" in a function returning Result<T, AppError>, and fallible_call returns Result<T, ParseError>, the compiler checks if From<ParseError> is implemented for AppError. If so, it automatically inserts the conversion: "let x = match fallible_call() { Ok(v) => v, Err(e) => return Err(AppError::from(e)) };". This eliminates the boilerplate of manually wrapping every error. The From implementation typically wraps the source error in an enum variant, either as a tuple variant (AppError::ParseError(ParseIntError)) or with a conversion that extracts relevant information.

This design solves a critical problem in error handling: propagating errors across abstraction boundaries. Without automatic conversion, every call to a function with a different error type requires manual wrapping, creating noise that obscures logic. Languages with exceptions avoid this through exception hierarchies and polymorphism, but lose type safety - catch blocks can intercept unexpected exceptions. Rust's approach maintains type safety while providing convenience through explicit but automatic conversions. The From trait creates a directed graph of error conversions, making error transformation visible in type definitions while invisible at call sites. This is more powerful than exception hierarchies because conversions can include arbitrary logic, not just subtyping.

In practice, implement From for each external error type that your functions might encounter. For example, if your business logic calls database and parsing code, implement From<DbError> and From<ParseError> for your AppError type. This creates a unified error type for your layer while preserving information about the underlying error source. Pattern: wrap errors in enum variants to maintain error type information and enable specific handling. Use tuple variants to preserve the original error: From<IoError> for AppError { fn from(err: IoError) -> Self { AppError::Io(err) } }. Be cautious about implementing From too liberally - it creates implicit conversions that can make error flow less obvious. Consider the thiserror crate which automates this pattern with derive macros."#,
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
            commentary: r#"Box<dyn Error> is Rust's dynamic error type that can hold any error implementing the std::error::Error trait, providing maximum flexibility at the cost of type precision. The Box provides heap allocation for the error (necessary because different error types have different sizes), while dyn Error uses trait objects for dynamic dispatch, allowing heterogeneous error types to be returned from the same function. This approach is invaluable for application code where the specific error type matters less than knowing something failed, or when aggregating errors from multiple sources would create unwieldy custom error enums. The ? operator works seamlessly with Box<dyn Error> because most standard library errors implement Error and can be automatically converted.

The technical implementation relies on Rust's trait object system. Box<dyn Error> is a fat pointer containing both a pointer to the error data on the heap and a pointer to a vtable for the Error trait's methods. This enables runtime polymorphism - different error types can be returned through the same Result<T, Box<dyn Error>> type, with method dispatch determined at runtime. The size is always two pointers regardless of the underlying error type, which is why boxing is necessary - without it, the compiler couldn't know how much stack space to allocate. The tradeoff is performance (heap allocation and dynamic dispatch) and loss of type information - you can't pattern match on specific error types without downcasting.

This design choice reflects a pragmatic balance between Rust's static typing and practical ergonomics. While Rust encourages precise types, sometimes the verbosity of maintaining custom error enums across many error sources outweighs the benefits, especially in application code where errors are logged rather than handled programmatically. This contrasts with languages like Java where exceptions are dynamically typed by default, or Go where error is an interface but errors are typically strings. Rust's Box<dyn Error> provides dynamic typing opt-in, preserving type information through the Error trait's source() method for error chaining while avoiding the explosion of variants in custom error enums.

In practice, use Box<dyn Error> in applications, scripts, or when prototyping where you want quick error propagation without custom error type ceremony. It's perfect for main functions, CLI tools, and situations where errors are simply logged or displayed to users. However, libraries should almost always use specific error types because library consumers need to handle different errors differently - they can't match on Box<dyn Error> effectively. You can add context with error wrapping crates like anyhow (application) or maintain type safety with thiserror (library). A common pattern is using Box<dyn Error> internally but converting to specific types at API boundaries. Be aware that losing type information means you can't handle specific errors programmatically without runtime type inspection using downcast_ref()."#,
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
            commentary: r#"The early return pattern for error handling uses guard clauses that return Err immediately when validation fails, keeping the happy path at the base indentation level and making validation logic linear and explicit. Instead of nesting if-else blocks or accumulating validation in a deeply indented tree, each validation check stands alone at the top level: if something is wrong, return the error immediately, otherwise continue. This creates code that reads naturally from top to bottom, with all validation concentrated at the beginning and the successful processing path remaining unindented and visually prominent. The pattern emerges naturally from Rust's Result type and makes control flow obvious - errors cause immediate function exit rather than setting flags or breaking from loops.

The technical implementation is straightforward: validation functions return Result<T, E>, and each validation check uses an if condition that returns Err(...) on failure. The key insight is that return is not just for the function's end - it can exit early from anywhere, making guard clauses natural. Unlike languages with exceptions where early validation might throw, Rust's early returns remain in normal control flow, returning error values rather than unwinding the stack. This keeps error handling explicit and predictable - there's no invisible stack unwinding, no catch blocks far from the validation logic, just clear, linear code that says "if this is wrong, we're done, otherwise carry on."

Rust's design encourages this pattern because Result makes errors explicit and first-class values. Early returns with Err are essentially guard clauses, a well-established pattern from languages like Swift and Kotlin, but more powerful in Rust because the type system guarantees you handle the error at the call site. This contrasts with exception-based validation where early throws can bypass cleanup code unless carefully wrapped in try-finally, and with languages like C where early returns with error codes can be ignored. The early return pattern also works beautifully with the ? operator - validation functions can call other validation functions and propagate errors concisely.

In practice, use early returns for validation-heavy functions where multiple preconditions must be checked before processing. Common in HTTP request handlers, command parsers, data validators, and API entry points. Structure your function as: validate inputs with early returns for each error case, then the happy path processes valid data without indentation. Each validation becomes self-documenting: "if value < 0, that's an error, move on otherwise." This is more maintainable than deeply nested if-else or accumulating errors in a list. Combine with the ? operator for validation functions that call other fallible functions. A pitfall is making error messages too generic - each early return should provide specific, actionable error information about which validation failed and why."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
