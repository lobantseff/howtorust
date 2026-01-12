// Lifetimes examples organized in a separate file

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_lifetime",
            description: "Function with lifetime annotations",
            code: r#"fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

let string1 = String::from("long string");
let string2 = String::from("short");
let result = longest(&string1, &string2);
println!("Longest: {}", result);"#,
            commentary: r#"📚 INTRODUCTION
Lifetime annotations describe relationships between reference lifetimes using the 'a syntax. The signature fn longest<'a>(x: &'a str, y: &'a str) -> &'a str means "x and y live for some lifetime 'a, and the return value also lives for 'a." This tells the borrow checker that the returned reference is valid as long as both inputs are valid.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Languages without lifetime tracking (C, C++) allow functions to return pointers to freed memory, causing undefined behavior and security vulnerabilities. Languages with garbage collection (Java, Python) prevent this but add runtime overhead. Rust's lifetime annotations solve dangling references at compile time with zero runtime cost. They prevent: returning references to local variables, using references after their data is freed, data races from shared mutable state, and iterator invalidation bugs.

🔍 IMPORTANT DETAILS & INTRICACIES
The 'a lifetime represents the overlap of input lifetimes - the shorter of the two. If string1 lives for the entire function but string2 lives only part of it, 'a becomes string2's lifetime. The returned reference can't outlive 'a because it might be string2. Lifetime annotations are constraints the borrow checker enforces, not instructions that change actual lifetimes. They don't affect memory layout or runtime behavior - they're purely compile-time checks that get erased after verification.

💼 WHERE IT'S MOST USED
Function signatures returning references derived from input references (parsing functions returning slices, comparison functions like longest, data structure methods returning internal references). Common in zero-copy parsers, string manipulation libraries, iterator implementations, and any API that borrows rather than owns data for performance.

✅ TAKEAWAY
Lifetime annotations are constraints that make borrowing relationships explicit, enabling the compiler to verify reference safety across function boundaries. They represent the overlap of input lifetimes and prevent dangling references at compile time with zero runtime cost. Think of them as documentation that the compiler verifies - they don't change behavior, they just ensure references are used safely. Start with one lifetime for related references, adding more only when inputs have truly independent lifetimes."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "lifetime_elision",
            description: "Functions where lifetimes are inferred",
            code: r#"// These lifetimes are inferred by the compiler
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

let sentence = String::from("hello world");
let word = first_word(&sentence);
println!("First word: {}", word);"#,
            commentary: r#"📚 INTRODUCTION
Lifetime elision rules allow the compiler to infer lifetime annotations in common patterns, reducing boilerplate. The function first_word returns &str without explicit annotations, yet the compiler knows the returned reference has the same lifetime as the input. Three rules handle most cases: each input reference gets its own lifetime, single-input functions assign that lifetime to outputs, and methods assign &self's lifetime to outputs.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Early Rust required explicit lifetimes everywhere, making simple functions verbose (fn first_word<'a>(s: &'a str) -> &'a str). This was safer than C/C++ (no dangling pointers) but more tedious than garbage-collected languages. Elision rules emerged from analyzing real code patterns, providing both safety and ergonomics. They prevent: annotation boilerplate in 87% of functions, the friction of writing trivial lifetime relationships, and the learning curve barrier for beginners.

🔍 IMPORTANT DETAILS & INTRICACIES
Rule 1: Each parameter reference gets a unique lifetime ('a, 'b, etc.). Rule 2: If there's exactly one input lifetime, assign it to all outputs. Rule 3: If there's &self or &mut self, assign its lifetime to all outputs. The compiler applies these deterministically, stopping when all lifetimes are determined or ambiguity remains. Elided lifetimes are just as checked and safe as explicit ones - elision is purely syntactic sugar. The compiler mentally writes the annotations for you.

💼 WHERE IT'S MOST USED
Simple functions with one reference parameter (string processing, slice operations), all struct methods returning references to self's data (getters, iterators), and functions that clearly transform one borrowed input to one borrowed output. Approximately 87% of functions with lifetimes can use elision rules.

✅ TAKEAWAY
Lifetime elision rules let you omit obvious lifetime annotations without sacrificing safety. The compiler uses three deterministic rules to infer lifetimes in common patterns, making code cleaner while maintaining the same compile-time guarantees. When the compiler asks for explicit annotations, it's because the elision rules can't determine which inputs the output borrows from - you need to resolve the ambiguity. Most functions benefit from elision; only complex multi-input functions need explicit lifetimes."#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "struct_lifetimes",
            description: "Structs that hold references",
            code: r#"struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self) -> &str {
        println!("Attention please!");
        self.part
    }
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().unwrap();
let excerpt = Excerpt { part: first_sentence };
println!("Excerpt: {}", excerpt.announce());"#,
            commentary: r#"📚 INTRODUCTION
Structs holding references need lifetime parameters to ensure instances don't outlive borrowed data. Excerpt<'a> declares the struct is generic over lifetime 'a, and the part field borrows data for that lifetime. This constrains struct instances - an Excerpt<'a> cannot outlive the string it references. The impl<'a> Excerpt<'a> syntax declares the lifetime for method implementations.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
C/C++ structs can store pointers to freed memory, causing use-after-free bugs and security vulnerabilities. Languages like Java avoid this with garbage collection but prevent zero-copy patterns (every struct field must be owned). Rust's struct lifetimes enable zero-copy data structures while preventing dangling references. They solve: use-after-free bugs in struct fields, iterator invalidation when containers are modified, parser performance (borrow input instead of copying), and self-referential struct issues.

🔍 IMPORTANT DETAILS & INTRICACIES
The lifetime parameter becomes part of the struct's type - Excerpt<'a> and Excerpt<'b> are distinct types, like Vec<i32> and Vec<f64>. When you create an instance, the compiler infers the lifetime from the borrowed data and tracks it through the type system. If you pass an Excerpt to a function, the lifetime goes with it. Methods can return references tied to the struct's lifetime (announce returns &str with lifetime 'a) or introduce new lifetimes for parameters. All relationships are tracked at compile time with zero runtime overhead.

💼 WHERE IT'S MOST USED
Zero-copy parsers holding slices of input data (serde, nom parser combinators), iterator implementations borrowing from collections, views into data structures (string slices, array windows), configuration holders borrowing static data, and any high-performance code that needs to avoid cloning. Common in embedded systems and systems programming where allocations are expensive.

✅ TAKEAWAY
Struct lifetimes enable zero-copy data structures by ensuring instances can't outlive their borrowed data. The lifetime parameter becomes part of the type, flowing through the type system to track borrowing relationships. This prevents use-after-free bugs at compile time while enabling high-performance patterns impossible in garbage-collected languages. Choose borrowed references when you don't need ownership and the data lives long enough; use owned data (String, Vec) when instances must outlive the source or be sent between threads."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "multiple_lifetimes",
            description: "Functions with multiple lifetime parameters",
            code: r#"fn longest_with_announcement<'a, 'b>(
    x: &'a str,
    y: &'a str,
    ann: &'b str,
) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

let s1 = "abc";
let s2 = "defgh";
let announcement = "Comparing strings";
let result = longest_with_announcement(s1, s2, announcement);
println!("Result: {}", result);"#,
            commentary: r#"📚 INTRODUCTION
Functions need multiple lifetime parameters when inputs have independent lifetimes and different relationships to outputs. This example uses 'a for x, y, and the return value (connected - return borrows from x or y), while 'b applies to ann (independent - just used for printing). The separate lifetimes tell the compiler that ann's lifetime doesn't constrain the returned reference.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Single-lifetime functions would force all parameters to live equally long, which is overly restrictive. C/C++ doesn't track these relationships at all, allowing dangling pointers. Java's garbage collection sidesteps the issue but adds runtime cost. Rust's multiple lifetimes enable precise borrowing control with compile-time checking. They prevent: unnecessarily restrictive APIs that reject valid code, forcing temporary data to live longer than needed, conflating independent borrowing relationships, and the performance overhead of cloning data just to satisfy overly strict lifetime requirements.

🔍 IMPORTANT DETAILS & INTRICACIES
Lifetime parameters are determined at call sites by actual argument lifetimes. When you call the function, 'a becomes the shorter of s1 and s2's lifetimes, while 'b becomes announcement's lifetime - these might be completely different scopes. The type system tracks each through the call, ensuring the return value isn't used beyond 'a's scope. If all parameters used one lifetime, the compiler would require ann to live as long as the result, unnecessarily restricting valid code. The borrow checker knows which borrows must outlive which others.

💼 WHERE IT'S MOST USED
Complex APIs where some inputs affect output validity and others don't (logging/debugging parameters, configuration vs data parameters), functions processing multiple independent data sources, parsers with separate input and context lifetimes, and callbacks taking both long-lived and short-lived references. Common in large library APIs with optional parameters.

✅ TAKEAWAY
Multiple lifetime parameters give fine-grained control over borrowing relationships, enabling flexible APIs that accept references with independent lifetimes. They express "the output depends on these parameters but not those," allowing the borrow checker to accept more valid code. Start with one lifetime and add more only when parameters have truly independent relationships or when the borrow checker indicates conflicts. Clear naming ('input, 'output) makes complex signatures more readable than 'a, 'b, 'c."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "lifetime_bounds",
            description: "Generic types with lifetime bounds",
            code: r#"use std::fmt::Display;

fn longest_with_display<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

let result = longest_with_display("long", "short", 42);
println!("Result: {}", result);"#,
            commentary: r#"📚 INTRODUCTION
Generic functions can combine type parameters, trait bounds, and lifetime parameters. This example uses lifetime 'a for references, type parameter T for the announcement, and trait bound T: Display to constrain T's capabilities. All three systems work together: lifetimes ensure reference validity, generics enable type flexibility, and trait bounds guarantee required operations exist.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
C++ templates check constraints at instantiation, giving cryptic errors deep in template code. Java's generics are erased, preventing some optimizations. Go lacks generics entirely (before 1.18), forcing interface{} and runtime casts. Rust combines compile-time generics with lifetime safety and trait bounds. This prevents: cryptic template errors far from the actual problem, runtime type errors from erased generics, the performance cost of dynamic dispatch, and unsafe code from unconstrained generic parameters.

🔍 IMPORTANT DETAILS & INTRICACIES
Lifetimes are actually a form of generic - they parameterize code over how long references live, just as type parameters parameterize over what types are used. The <'a, T> syntax treats both uniformly. Lifetime bounds on type parameters (T: 'a) express that T must live at least as long as 'a, useful when T might contain references. Generic types can capture lifetimes, and bounds ensure those captured lifetimes are valid long enough. All checks happen at compile time and are erased before runtime - zero overhead abstraction.

💼 WHERE IT'S MOST USED
Library APIs that need maximum flexibility and reusability (collections, iterators, parsers), trait implementations that work with borrowed data, builder patterns combining owned and borrowed data, serialization/deserialization frameworks (serde), and any code requiring both zero-cost abstraction and memory safety. Common in the standard library and foundational crates.

✅ TAKEAWAY
Combining lifetimes, generics, and trait bounds creates flexible, type-safe, zero-cost abstractions. Each piece serves a purpose: lifetimes for safety, generics for reusability, trait bounds for capabilities. The <'a, T> syntax treats lifetimes and types uniformly as generic parameters, enabling powerful reasoning about both. Use where clauses to keep signatures readable when combining many constraints. This is the backbone of Rust's ability to match C++ performance while preventing entire classes of bugs at compile time."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "method_lifetimes",
            description: "Lifetime annotations in method definitions",
            code: r#"struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

let novel = String::from("Once upon a time...");
let excerpt = ImportantExcerpt { part: &novel };
println!("Level: {}", excerpt.level());
println!("Part: {}", excerpt.announce_and_return_part("Starting"));"#,
            commentary: r#"📚 INTRODUCTION
Method lifetimes benefit from elision, particularly the third rule that assigns &self's lifetime to all output references. The announce_and_return_part method returns &str without explicit annotations, yet the compiler correctly infers it returns a reference tied to &self's lifetime (the struct's 'a), not to announcement. This makes method signatures clean while maintaining safety.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Without elision, methods would need explicit lifetime annotations for every returned reference (fn announce_and_return_part(&self, announcement: &str) -> &'a str), cluttering code. C++ has no such tracking, allowing methods to return dangling pointers. Languages with garbage collection avoid the issue but sacrifice performance. Rust's method elision rules provide both safety and clean syntax. They prevent: annotation clutter in the 90% case where methods return self's data, mistakes from forgetting lifetime relationships, and the ergonomic friction that would make Rust painful for OOP patterns.

🔍 IMPORTANT DETAILS & INTRICACIES
The third elision rule creates asymmetry between self and other parameters - methods default to returning self's data, not parameter data. This reflects real-world usage where methods primarily expose internal state. When a method needs to return parameter data instead, you must use explicit annotations to override the default (fn use_param<'a, 'b>(&'a self, param: &'b str) -> &'b str). The level method shows another pattern: methods returning non-reference types never need lifetime annotations since they don't borrow. The compiler tracks the struct's lifetime parameter through all methods automatically.

💼 WHERE IT'S MOST USED
Getter methods exposing struct fields, iterator implementations returning references to collection elements, builder pattern methods returning references to constructed objects, parser methods returning slices of input, and any struct API that provides views into internal data. About 90% of struct methods that return references use elision.

✅ TAKEAWAY
Method lifetime elision makes the common case automatic: methods returning self's data need no explicit lifetime annotations. The third elision rule assigns &self's lifetime to outputs, making struct APIs clean and readable while maintaining compile-time safety. Only when returning parameter data or combining multiple sources do you need explicit annotations. This asymmetry reflects real usage patterns and makes Rust ergonomic for object-oriented designs where methods primarily expose internal state."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "static_lifetime",
            description: "The 'static lifetime for references",
            code: r#"// String literals have 'static lifetime
let s: &'static str = "I live for the entire program";
println!("{}", s);

// Static variables also have 'static lifetime
static GLOBAL: &str = "Global string";
println!("{}", GLOBAL);"#,
            commentary: r#"📚 INTRODUCTION
The 'static lifetime represents data that lives for the entire program execution. String literals have 'static lifetime because they're embedded in the binary - the data exists before main runs and persists until termination. The reference &'static str points to immutable data in the program's data segment, never freed or moved. This is the longest possible lifetime in Rust.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
C/C++ string literals can be modified (causing crashes) or confused with heap-allocated strings. Languages like Java guarantee string immutability but all strings are heap-allocated with GC overhead. Rust's 'static lifetime ensures program-lifetime data is immutable and embedded in the binary. It prevents: modifying string literals (undefined behavior in C), deallocating static data, confusion about where data is stored, and the performance cost of heap-allocating constant data.

🔍 IMPORTANT DETAILS & INTRICACIES
'static is often misunderstood. T: 'static doesn't mean T must be a 'static reference - it means T must not contain any non-'static references. Owned types like String, Vec<T>, and integers all satisfy T: 'static because they don't borrow anything. The bound appears in threading APIs where types sent to threads can't borrow short-lived data. A function parameter &'static str is overly restrictive - usually fn process(s: &str) with inferred lifetime is better. Reserve 'static for data that genuinely must persist for the program's lifetime.

💼 WHERE IT'S MOST USED
String literals and compile-time constants, global configuration and lookup tables, thread spawning (T: 'static + Send bounds), error messages and format strings, and embedded systems where all data must be known at compile time. Common in libraries that need constant data tables (regex patterns, Unicode tables, compression dictionaries).

✅ TAKEAWAY
'static represents program-lifetime data embedded in the binary, not heap-allocated. String literals and static variables are automatically 'static. The trait bound T: 'static means "no borrowed references," not "must be a literal" - owned types like String satisfy it. Don't overuse 'static in function parameters; use inferred lifetimes for flexibility. In threading, T: 'static allows owned data and 'static references but not temporary borrows, enabling safe cross-thread sharing without restricting everything to literals."#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "lifetime_subtyping",
            description: "Lifetime variance and subtyping",
            code: r#"struct Context<'a>(&'a str);

struct Parser<'a, 's> {
    context: &'a Context<'s>,
}

impl<'a, 's> Parser<'a, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Ok(())
    }
}

let ctx = Context("some context");
let parser = Parser { context: &ctx };
match parser.parse() {
    Ok(_) => println!("Parsing succeeded"),
    Err(e) => println!("Error: {}", e),
}"#,
            commentary: r#"📚 INTRODUCTION
Complex data structures can have multiple independent lifetimes modeling nested borrowing relationships. Parser has two lifetimes: 'a for how long it borrows Context, and 's for how long Context borrows its internal string. These lifetimes are independent - Parser's lifetime doesn't constrain Context's internal data lifetime. Each borrowing relationship gets its own lifetime parameter.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Single-lifetime structs would force all borrowed data to live equally long, preventing valid nested borrowing patterns. C/C++ allows arbitrary pointer nesting but with no safety. Java requires ownership of nested data, preventing zero-copy structures. Rust's multiple struct lifetimes enable precise tracking of multi-level borrowing. They prevent: conflating independent borrowing relationships, forcing data to live longer than necessary, the performance cost of copying data at each nesting level, and invalid patterns where inner data outlives outer containers.

🔍 IMPORTANT DETAILS & INTRICACIES
Lifetime subtyping and variance enable this flexibility. The compiler understands that longer lifetimes can substitute for shorter ones (covariance) - 'static works where 'a is expected. The lifetime system tracks relationships transitively: if 's outlives 'a, and Context<'s> is borrowed for 'a, the borrow checker ensures validity throughout. This mathematical rigor allows complex patterns while maintaining safety. Without separate parameters, the compiler would require everything to live as long as everything else, rejecting valid code.

💼 WHERE IT'S MOST USED
Zero-copy parsers (input data lifetime vs parser state lifetime), hierarchical data structures with multiple borrowing levels, complex iterators borrowing both collection and iterator state separately, nested views into data structures, and any multi-level borrowing where lifetimes are logically independent. Common in nom parser combinators, tree data structures with borrowed nodes, and streaming APIs.

✅ TAKEAWAY
Multiple independent lifetimes enable precise tracking of nested borrowing relationships in complex data structures. Each borrowing level gets its own lifetime parameter, preventing the compiler from conflating independent concerns. Start with single lifetimes and add more only when relationships are truly independent or when the borrow checker indicates conflicts. Use clear naming ('input, 'parser) instead of 'a, 'b for complex signatures. This enables zero-copy patterns with multi-level borrowing that would be impossible in languages with either no tracking (unsafe) or single-level tracking (overly restrictive)."#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "higher_ranked_trait_bounds",
            description: "Higher-ranked lifetime bounds (for<'a>)",
            code: r#"trait Processor {
    fn process(&self, input: &str) -> String;
}

fn apply_processor<F>(data: &str, processor: F) -> String
where
    F: Fn(&str) -> String,
{
    processor(data)
}

let result = apply_processor("hello", |s| s.to_uppercase());
println!("Result: {}", result);"#,
            commentary: r#"📚 INTRODUCTION
Higher-ranked trait bounds use for<'a> syntax to express that a function or closure must work for any lifetime, not just one specific lifetime. When you write F: Fn(&str) -> String, it's shorthand for F: for<'a> Fn(&'a str) -> String. This says "F must work for any possible lifetime 'a," enabling the closure to be called with references of any validity.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Without higher-ranked bounds, closures would be tied to the lifetime where they were created, severely limiting reusability. C++ lambdas can capture pointers unsafely. Java's lambdas work on objects, not references, forcing heap allocation. Rust's higher-ranked bounds enable closures that work with borrowed data of any lifetime safely. They prevent: closures that can't be called with different lifetimes, forcing heap allocation to work around lifetime restrictions, the inflexibility of closures tied to specific scopes, and unsafe code from unchecked lifetime assumptions.

🔍 IMPORTANT DETAILS & INTRICACIES
The for<'a> quantifies over all possible lifetimes universally - the closure must work for any 'a the caller provides. This is more general than capturing a specific lifetime. Iterator methods like map and filter rely on this - each element might have a different lifetime, and higher-ranked bounds ensure the closure handles all of them. The compiler infers for<'a> for standard function traits (Fn, FnMut, FnOnce) automatically. You only write it explicitly in complex generic constraints, custom traits, or when compiler errors mention it.

💼 WHERE IT'S MOST USED
Iterator combinators (map, filter, fold) where closures process elements of varying lifetimes, higher-order functions accepting callbacks, trait objects with methods taking references, generic code working with borrowed data of unknown lifetimes, and any API that needs closures to work across different calling contexts. Essential to the zero-cost abstraction foundation of Rust's iterator chains.

✅ TAKEAWAY
Higher-ranked trait bounds enable closures and functions that work with any lifetime, not just one specific lifetime. The for<'a> syntax means "must work for all possible lifetimes," making closures reusable across contexts. This is inferred automatically for standard function traits - you rarely write it explicitly. It's key to Rust's zero-cost abstractions: generic code that works with borrowed data of any lifetime without runtime overhead. This universality makes closures and iterators composable and flexible while maintaining compile-time safety guarantees."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
