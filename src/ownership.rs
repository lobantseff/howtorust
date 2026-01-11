// Ownership Module: Understanding Rust's ownership system
// Examples progress from beginner to advanced

use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_ownership",
            description: "Basic ownership transfer (move semantics)",
            code: r#"let s1 = String::from("hello");
let s2 = s1;  // s1 is moved to s2
// println!("{}", s1);  // This would error!
println!("s2: {}", s2);"#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "clone_vs_move",
            description: "Cloning vs moving data",
            code: r#"let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy
println!("s1: {}, s2: {}", s1, s2);  // Both valid!"#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "function_ownership",
            description: "Ownership with functions",
            code: r#"fn takes_ownership(s: String) {
    println!("Inside function: {}", s);
}  // s goes out of scope and is dropped

let s = String::from("hello");
takes_ownership(s);
// println!("{}", s);  // Error: value moved"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "borrowing_immutable",
            description: "Immutable borrowing (references)",
            code: r#"fn calculate_length(s: &String) -> usize {
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("'{}' has length {}", s1, len);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "borrowing_mutable",
            description: "Mutable borrowing",
            code: r#"fn append_world(s: &mut String) {
    s.push_str(", world!");
}

let mut s = String::from("hello");
append_world(&mut s);
println!("{}", s);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "multiple_references",
            description: "Multiple immutable references allowed",
            code: r#"let s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &s;
println!("{}, {}, {}", r1, r2, r3);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "reference_rules",
            description: "Borrowing rules: can't mix mutable and immutable",
            code: r#"let mut s = String::from("hello");
let r1 = &s;     // Immutable borrow
let r2 = &s;     // Immutable borrow
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // Mutable borrow OK now
r3.push_str(" world");
println!("{}", r3);"#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "slice_internals",
            description: "String slices and memory representation",
            code: r#"let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
println!("First word: {}, second word: {}", hello, world);

// Slices don't take ownership
println!("Original string still valid: {}", s);"#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "dangling_reference_prevention",
            description: "How Rust prevents dangling references",
            code: r#"// This function returns a reference to data it owns
// Rust won't compile this!
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // Error: s will be dropped
}
*/

// Correct way: return owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Ownership moves out
}

let s = no_dangle();
println!("{}", s);"#,
            difficulty: Difficulty::Advanced,
        },
    ]
}

// Example execution functions
pub fn run_example(name: &str) {
    match name {
        "basic_ownership" => basic_ownership(),
        "clone_vs_move" => clone_vs_move(),
        "function_ownership" => function_ownership(),
        "borrowing_immutable" => borrowing_immutable(),
        "borrowing_mutable" => borrowing_mutable(),
        "multiple_references" => multiple_references(),
        "reference_rules" => reference_rules(),
        "slice_internals" => slice_internals(),
        "dangling_reference_prevention" => dangling_reference_prevention(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
}

fn clone_vs_move() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn function_ownership() {
    fn takes_ownership(s: String) {
        println!("Inside function: {}", s);
    }

    let s = String::from("hello");
    takes_ownership(s);
}

fn borrowing_immutable() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' has length {}", s1, len);
}

fn borrowing_mutable() {
    fn append_world(s: &mut String) {
        s.push_str(", world!");
    }

    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);
}

fn multiple_references() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}, {}, {}", r1, r2, r3);
}

fn reference_rules() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}

fn slice_internals() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("First word: {}, second word: {}", hello, world);
    println!("Original string still valid: {}", s);
}

fn dangling_reference_prevention() {
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }

    let s = no_dangle();
    println!("{}", s);
}
