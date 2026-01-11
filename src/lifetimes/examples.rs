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
            commentary: r#"Lifetime annotations describe the relationships between the lifetimes of references. The 'a syntax
is a lifetime parameter. This signature means: the returned reference will live as long as the
shorter of the two input references. Lifetimes don't change how long references live.

From The Rust Book (Chapter 10.3):
"Lifetime annotations don't change how long any of the references live. They describe the relationships
of the lifetimes of multiple references to each other."

The borrow checker uses these annotations to ensure references remain valid."#,
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
            commentary: r#"Lifetime elision rules allow the compiler to infer lifetimes in many cases, so you don't need to
write them explicitly. The function returns a reference with the same lifetime as the input. The
compiler applies three elision rules to determine if explicit annotations are needed.

From The Rust Book (Chapter 10.3):
"The patterns programmed into Rust's analysis of references are called the lifetime elision rules.
These aren't rules for programmers to follow; they're a set of particular cases that the compiler
will consider."

If elision rules don't apply, you must annotate lifetimes explicitly."#,
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
            commentary: r#"Structs that hold references need lifetime annotations to tell the compiler how long those references
should remain valid. Excerpt<'a> means an instance cannot outlive the reference it holds in part.
This prevents dangling references in struct fields.

From The Rust Book (Chapter 10.3):
"This annotation means an instance of Excerpt can't outlive the reference it holds in its part field."

When implementing methods, you need impl<'a> to declare the lifetime parameter for the impl block."#,
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
            commentary: r#"Functions can have multiple lifetime parameters when references have independent lifetimes. Here,
'a applies to x, y, and the return value (they're connected), while 'b applies to ann (independent).
This tells the compiler that ann's lifetime doesn't affect the returned reference.

From The Rust Book (Chapter 10.3):
"When annotating lifetimes in functions, the annotations go in the function signature, not in the
function body."

Use multiple lifetimes when inputs have unrelated scopes or the return value relates to only some inputs."#,
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
            commentary: r#"You can combine generic type parameters with lifetimes. Generic types can also have lifetime bounds
(like T: 'a) meaning T must live at least as long as 'a. This example combines generics, trait bounds,
and lifetimes in one signature.

From The Rust Book (Chapter 10.3):
"We can use generic type parameters, trait bounds, and lifetimes all in the same function."

Lifetimes are a form of generic - they parameterize how long references are valid."#,
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
            commentary: r#"Method lifetimes often benefit from elision. In announce_and_return_part, the return type's lifetime
is inferred to match &self (the struct's 'a lifetime), not the announcement parameter. The third
elision rule states that if there's a &self parameter, its lifetime is assigned to all output references.

From The Rust Book (Chapter 10.3):
"If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime
of self is assigned to all output lifetime parameters."

This makes method signatures much cleaner than explicit annotations would."#,
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
            commentary: r#"The 'static lifetime means the reference is valid for the entire duration of the program. String
literals are stored in the program binary and have 'static lifetime. Be careful not to overuse 'static
- often you want a more specific lifetime.

From The Rust Book (Chapter 10.3):
"The 'static lifetime denotes that the affected reference can live for the entire duration of the
program."

Common mistake: Using 'static when you actually just need any valid lifetime. Let the compiler infer when possible."#,
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
            commentary: r#"Advanced lifetime patterns can involve nested lifetimes and lifetime subtyping. Here, Parser has
two independent lifetimes: 'a for the parser's reference to Context, and 's for the string data
within Context. This models complex borrowing relationships.

From The Rust Book (Chapter 19.2 - Advanced Lifetimes):
"Lifetime subtyping ensures that one lifetime outlives another lifetime."

This pattern is common in parsers and complex data structures with multiple borrowing levels."#,
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
            commentary: r#"Higher-ranked trait bounds (HRTB) use for<'a> syntax to express that a closure or function must
work for ANY lifetime. When you write Fn(&str), it's actually shorthand for for<'a> Fn(&'a str).
The compiler automatically applies HRTB for function traits.

From The Rust Book (Chapter 19.2):
"When a closure has a parameter that's a reference, the closure needs to be generic over lifetimes."

HRTBs are usually inferred, but you may need to write them explicitly for complex trait bounds."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
