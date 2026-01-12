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
            commentary: r#"Lifetime annotations describe relationships between reference lifetimes, not the lifetimes themselves. The 'a syntax declares a lifetime parameter - a generic over how long references live. The signature fn longest<'a>(x: &'a str, y: &'a str) -> &'a str means "x and y live for some lifetime 'a, and the return value also lives for 'a." This tells the borrow checker that the returned reference is valid as long as both inputs are valid. Lifetime annotations are constraints the borrow checker enforces, not instructions that change actual lifetimes.

The key insight is that 'a represents the overlap of the input lifetimes - the shorter of the two. If string1 lives for the entire function and string2 lives for only part of it, 'a represents string2's lifetime. The returned reference can't outlive 'a because it might be string2 (which dies sooner). The borrow checker uses this information to reject code where the returned reference would outlive one of its possible sources. This catches dangling reference bugs at compile time with no runtime overhead.

Lifetime annotations don't change program behavior - they only help the compiler verify references are used safely. The actual lifetimes are determined by scope and control flow. Think of lifetime annotations as documentation that the compiler verifies, similar to type annotations. They make implicit lifetime relationships explicit, enabling the borrow checker to reason about reference validity across function boundaries. Without these annotations, the compiler can't determine how returned references relate to input references.

In practice, lifetime annotations appear in function signatures that return references derived from input references. The annotation connects inputs to outputs, telling the compiler "this returned reference borrows from these inputs." Most simple functions don't need lifetime annotations thanks to lifetime elision rules. When they're needed, start by declaring one lifetime for all related references, then add more lifetimes only if inputs have truly independent lifetimes. Clear lifetime annotations make APIs self-documenting about borrowing relationships."#,
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
            commentary: r#"Lifetime elision rules allow the compiler to infer lifetime annotations in common patterns, reducing boilerplate without sacrificing safety. The function first_word returns &str without explicit lifetime annotations, yet the compiler knows the returned reference has the same lifetime as the input reference. Three elision rules handle most common cases: each input reference gets its own lifetime, functions with one input lifetime assign it to all outputs, and methods assign &self's lifetime to all outputs. If these rules can fully determine lifetimes, no explicit annotations are needed.

These rules emerged from analyzing real Rust code patterns. Early Rust required explicit lifetimes everywhere, creating verbose signatures for simple functions. The Rust team identified common patterns where lifetimes were predictable and encoded them as elision rules. The compiler tries these rules first, falling back to requiring explicit annotations only when ambiguity remains. This gives you the safety of lifetime checking with the ergonomics of simpler code. Elided lifetimes are just as checked and safe as explicit ones.

The elision rules are deterministic and well-defined. Rule 1: Each parameter reference gets a unique lifetime ('a, 'b, etc.). Rule 2: If there's exactly one input lifetime, it's assigned to all output lifetimes. Rule 3: If there's a &self or &mut self parameter, its lifetime is assigned to all outputs. The compiler applies these rules in order, stopping when all lifetimes are determined or when ambiguity remains (requiring explicit annotations). Understanding these rules helps you write cleaner APIs and know when annotations are needed.

In practice, most functions benefit from elision. Simple cases like functions taking one reference and returning a related reference work automatically. Methods returning references to self's data work automatically. Only when you have multiple input references and ambiguity about which the output relates to do you need explicit annotations. When the compiler asks for lifetime annotations, it's because the elision rules can't determine the relationships - you need to specify which inputs the output borrows from."#,
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
            commentary: r#"Structs holding references need lifetime parameters to ensure instances don't outlive the data they borrow. Excerpt<'a> declares that the struct is generic over a lifetime 'a, and the part field borrows data for that lifetime. This constrains struct instances - an Excerpt<'a> cannot outlive the borrowed string it references. The borrow checker enforces this, preventing dangling references in struct fields. Without the lifetime parameter, the compiler couldn't reason about how long the struct can safely exist.

The lifetime parameter becomes part of the struct's type - Excerpt<'a> and Excerpt<'b> are distinct types from the compiler's perspective, just like Vec<i32> and Vec<f64>. When you create an instance, the compiler infers the lifetime from the borrowed data. The lifetime tracks through the type system: if you pass an Excerpt to a function, the lifetime goes with it. This enables precise reasoning about borrowing even when references are stored in structs rather than just passed through function parameters.

When implementing methods, the impl<'a> Excerpt<'a> syntax declares the lifetime parameter for the impl block. Without <'a> after impl, you couldn't reference the lifetime used in Excerpt<'a>. The lifetime parameter from the struct definition flows into method signatures. Methods can return references tied to the struct's lifetime (like announce returning &str tied to 'a) or introduce new lifetimes for method parameters. The type system tracks all these relationships, ensuring safety.

In practice, structs with references appear when you need to hold borrowed data alongside other state. Common uses include parsers holding slices of input, views into collections, or wrappers around borrowed data. The lifetime parameter documents borrowing relationships in your type signatures. Sometimes you can avoid lifetimes by using owned data (String instead of &str) or smart pointers (Rc, Arc), but borrows are more efficient when you don't need ownership. Choose based on whether data needs to outlive the original owner."#,
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
            commentary: r#"Functions need multiple lifetime parameters when inputs have independent lifetimes and different relationships to outputs. This example uses 'a for x, y, and the return value (they're connected - the return borrows from x or y), while 'b applies to ann (independent - just used for printing). The separate lifetimes tell the compiler that ann's lifetime doesn't constrain the returned reference. This precision enables more flexible borrowing - ann can die before the result is used without issues.

Multiple lifetime parameters give you fine-grained control over borrowing relationships. If all parameters used one lifetime 'a, the compiler would require ann to live as long as the returned reference, which is unnecessarily restrictive. With separate lifetimes, you express "the output depends on these parameters but not those." The borrow checker uses this information to allow more code - it knows which borrows must outlive which others. This is crucial for complex functions that take multiple borrows with different roles.

The lifetime parameters are determined at call sites by the actual lifetimes of the arguments. When you call the function, 'a becomes the shorter of s1 and s2's lifetimes, while 'b becomes announcement's lifetime. These might be completely different scopes. The type system tracks these through the call, ensuring the return value isn't used beyond 'a's scope. Multiple lifetimes enable expressing complex but valid borrowing patterns that a single lifetime parameter would reject as potentially unsafe.

In practice, start with one lifetime for all references and add more only when the borrow checker complains or when you know logically that parameters have independent relationships. Most functions need only one lifetime. Multiple lifetimes appear in complex APIs where some inputs affect the output's validity and others don't, or when outputs borrow from specific inputs but not others. Clear naming ('input, 'output, etc.) can make complex signatures more readable than 'a, 'b, 'c."#,
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
            commentary: r#"Generic functions can combine type parameters, trait bounds, and lifetime parameters, creating highly flexible and type-safe APIs. This example uses lifetime parameter 'a for references, type parameter T for the announcement type, and trait bound T: Display to constrain T's capabilities. All three systems work together: lifetimes ensure reference validity, generics enable type flexibility, and trait bounds guarantee required operations exist. The signature shows the full power of Rust's type system integrated into one declaration.

Lifetime bounds on type parameters (T: 'a) express that the type T must live at least as long as 'a. This is useful when T might contain references that need to outlive a certain scope. Generic types can capture lifetimes, and lifetime bounds ensure those captured lifetimes are valid long enough. Though not shown in this simple example, lifetime bounds become critical in complex generic code where types might internally borrow data and those borrows need to coordinate with other lifetimes in the system.

Lifetimes are actually a form of generic - they parameterize code over how long references live, just as type parameters parameterize over what types are used. The <'a, T> syntax treats both uniformly as generic parameters. This conceptual unity makes Rust's generics more powerful than in languages where lifetimes are a separate, special mechanism. You can apply the same reasoning to lifetimes as to types: when do you need multiple, how do they constrain each other, how do they flow through your API.

In practice, combining lifetimes, generics, and trait bounds is common in library code that needs to be maximally flexible and reusable. The complexity of such signatures can be intimidating, but each piece serves a purpose: lifetimes for safety, generics for reusability, trait bounds for capabilities. Use where clauses to keep signatures readable when combining many constraints. Well-designed generic APIs with clear lifetime annotations form the backbone of zero-cost abstractions in Rust."#,
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
            commentary: r#"Method lifetimes benefit heavily from elision, particularly the third elision rule that assigns &self's lifetime to all output references. The announce_and_return_part method returns &str without explicit lifetime annotations, yet the compiler correctly infers it returns a reference tied to &self's lifetime (which is 'a from the struct), not to announcement. This makes sense semantically - methods typically return data from self, not from temporary parameters. The elision rule captures this common pattern, making method signatures clean and readable.

The elision rule for methods prevents a common mistake: forgetting that returned references might outlive temporary parameters. Without the rule, you'd need to write fn announce_and_return_part(&self, announcement: &str) -> &'a str explicitly, cluttering every method that returns self's data. The rule makes the common case (returning self's data) automatic while still being explicit when needed (returning parameter data requires explicit annotations to override the default).

This asymmetry between self and other parameters reflects real-world usage patterns. Methods primarily operate on and return data from self. When a method needs to return references to parameter data, the unusual pattern warrants explicit lifetime annotations to make the borrowing relationship clear. The automatic handling of self lifetimes makes Rust feel more ergonomic for object-oriented patterns where methods expose internal state.

In practice, most struct methods with references work with elision. Only when returning references to parameters (not self) or combining references from multiple sources do you need explicit lifetimes in method signatures. The level method shows another pattern: methods returning non-reference types never need lifetime annotations since they don't borrow. This combination of rules means most method signatures are lifetime-free, improving readability without sacrificing safety."#,
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
            commentary: r#"The 'static lifetime represents data that lives for the entire program execution. String literals have 'static lifetime because they're embedded in the binary - the data exists before main runs and persists until the program terminates. The reference &'static str points to immutable data in the program's data segment, never freed or moved. This is the longest possible lifetime in Rust, and 'static references can be freely used anywhere since they never become invalid.

Static lifetimes are often misunderstood. Requiring T: 'static doesn't mean T must be a 'static reference - it means T must not contain any non-'static references. Owned types like String, Vec<T>, and integers all satisfy T: 'static because they don't borrow anything. The bound T: 'static appears frequently in threading APIs where types sent to threads can't borrow short-lived data. Understanding this distinction prevents overusing 'static annotations on references when 'static bounds on types are what you actually need.

Beware of unnecessary 'static lifetime requirements. A function parameter &'static str is more restrictive than necessary unless you truly need program-lifetime data. Usually you want fn process(s: &str) with an inferred lifetime, allowing callers to pass any valid reference. Overusing 'static makes APIs rigid and limits reusability. Reserve 'static for data that genuinely must persist for the program's lifetime, like configuration loaded at startup or constant tables.

In practice, 'static appears in three main contexts: string literals and constants (automatically 'static), global variables (declared static), and trait bounds for sending to threads (T: 'static + Send). Don't add 'static to function parameters unless you have a specific reason. When threading, understand that T: 'static allows owned data and 'static references, but not temporary borrows. This enables safe cross-thread data sharing without restricting everything to literal constants."#,
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
            commentary: r#"Complex data structures can have multiple independent lifetimes modeling nested borrowing relationships. Parser has two lifetimes: 'a for how long it borrows Context, and 's for how long Context borrows its internal string. These lifetimes are independent - Parser's lifetime doesn't constrain Context's internal data lifetime. This pattern enables precise tracking of multi-level borrowing where one borrowed struct borrows from another, which itself contains borrowed data. Each borrowing relationship gets its own lifetime parameter.

Lifetime subtyping and variance enable this flexibility. The Rust compiler understands that longer lifetimes can be treated as shorter lifetimes (covariance), enabling 'static to work where 'a is expected. For references, longer-lived data can substitute for shorter requirements. The lifetime system tracks these relationships transitively: if 's outlives 'a, and Context<'s> is borrowed for 'a, the borrow checker ensures data validity throughout. This mathematical rigor ensures safety while allowing complex patterns that initially seem impossibly restrictive.

These nested lifetime patterns are essential for zero-copy parsers and complex data structures. A parser might borrow input data ('s), while its own state is borrowed for a shorter scope ('a). Without separate lifetime parameters, the compiler would conflate these independent concerns, requiring everything to live as long as everything else. Separate lifetimes communicate that some borrowing relationships are truly independent, enabling more flexible code without sacrificing safety.

In practice, start with single lifetimes and add more only when the borrow checker indicates conflicts or when you know logically that borrowing relationships are independent. Most structs need only one lifetime. Multiple lifetimes appear in parsers (input data vs parser state), hierarchical data structures (trees with multiple borrowing levels), and complex iterators (borrowing both the collection and iterator state separately). Clear lifetime naming ('input, 'parser, etc.) makes complex signatures understandable."#,
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
            commentary: r#"Higher-ranked trait bounds use for<'a> syntax to express that a function or closure must work for any lifetime, not just one specific lifetime. When you write F: Fn(&str) -> String, it's shorthand for F: for<'a> Fn(&'a str) -> String. This says "F must work for any possible lifetime 'a," enabling the closure to be called with references of any validity. The for<'a> quantifies over all possible lifetimes, making the closure more flexible than if it captured a specific lifetime at definition.

Higher-ranked lifetimes are necessary because closures and function traits are called multiple times, potentially with different lifetimes each time. Without for<'a>, the closure would be tied to the lifetime where it was created, limiting when it could be called. The higher-ranked bound ensures the closure works regardless of the caller's context. The compiler infers higher-ranked bounds for most function traits automatically, but you may need explicit for<'a> syntax in complex generic constraints or when working with custom traits.

This feature enables powerful abstractions where functions accept closures that work with arbitrary lifetimes. Iterator methods like map and filter rely on this - the closure is called for each element, and each element might have a different lifetime. Higher-ranked bounds ensure the closure can handle all of them. This is more general than capturing a specific lifetime, making closures more reusable across contexts.

In practice, you rarely write for<'a> explicitly - the compiler infers it for standard function traits. When you see it in compiler errors or need to write it manually in complex bounds, understand it means "this must work for any lifetime the caller provides." This universality is key to Rust's zero-cost abstractions - generic code that works with borrowed data of any lifetime without runtime overhead or restrictions. Higher-ranked bounds are part of what makes Rust's closures and iterators so flexible and composable."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
