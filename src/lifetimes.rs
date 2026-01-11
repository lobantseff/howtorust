// Lifetimes: Validating references

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
            difficulty: Difficulty::Advanced,
        },
    ]
}

pub fn run_example(name: &str) {
    match name {
        "basic_lifetime" => basic_lifetime(),
        "lifetime_elision" => lifetime_elision(),
        "struct_lifetimes" => struct_lifetimes(),
        "multiple_lifetimes" => multiple_lifetimes(),
        "lifetime_bounds" => lifetime_bounds(),
        "method_lifetimes" => method_lifetimes(),
        "static_lifetime" => static_lifetime(),
        "lifetime_subtyping" => lifetime_subtyping(),
        "higher_ranked_trait_bounds" => higher_ranked_trait_bounds(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_lifetime() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string");
    let string2 = String::from("short");
    let result = longest(&string1, &string2);
    println!("Longest: {}", result);
}

fn lifetime_elision() {
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
    println!("First word: {}", word);
}

fn struct_lifetimes() {
    struct Excerpt<'a> {
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
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("Excerpt: {}", excerpt.announce());
}

fn multiple_lifetimes() {
    fn longest_with_announcement<'a, 'b>(x: &'a str, y: &'a str, ann: &'b str) -> &'a str {
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
    println!("Result: {}", result);
}

fn lifetime_bounds() {
    use std::fmt::Display;

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
    println!("Result: {}", result);
}

fn method_lifetimes() {
    struct ImportantExcerpt<'a> {
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
    println!(
        "Part: {}",
        excerpt.announce_and_return_part("Starting")
    );
}

fn static_lifetime() {
    let s: &'static str = "I live for the entire program";
    println!("{}", s);

    static GLOBAL: &str = "Global string";
    println!("{}", GLOBAL);
}

fn lifetime_subtyping() {
    struct Context<'a>(&'a str);

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
    }
}

fn higher_ranked_trait_bounds() {
    fn apply_processor<F>(data: &str, processor: F) -> String
    where
        F: Fn(&str) -> String,
    {
        processor(data)
    }

    let result = apply_processor("hello", |s| s.to_uppercase());
    println!("Result: {}", result);
}
