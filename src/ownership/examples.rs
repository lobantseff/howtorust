// Ownership examples organized in a separate file

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
            commentary: r#"Ownership is Rust's most unique feature. Each value has a single owner, and when
the owner goes out of scope, the value is dropped. When we assign s1 to s2, the
ownership is MOVED - s1 is no longer valid. This prevents double-free errors and
ensures memory safety without a garbage collector.

From The Rust Book (Chapter 4.1):
"When a variable goes out of scope, Rust automatically calls the drop function and
cleans up the heap memory for that variable."

This is different from shallow copying in other languages - Rust actually invalidates
the first variable to prevent use-after-move bugs."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "clone_vs_move",
            description: "Cloning vs moving data",
            code: r#"let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy
println!("s1: {}, s2: {}", s1, s2);  // Both valid!"#,
            commentary: r#"The clone method creates a deep copy of heap data, allowing both variables to remain valid.
This is more expensive than moving since it copies the actual heap data. Use clone when you need
multiple owners of the same data, but be aware of the performance cost.

From The Rust Book (Chapter 4.1):
"If we do want to deeply copy the heap data of the String, not just the stack data, we can use
a common method called clone."

Common gotcha: Don't clone unnecessarily - only use it when you truly need independent copies."#,
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
            commentary: r#"Passing a value to a function moves ownership just like assignment does. After calling
takes_ownership(s), the variable s is no longer valid because ownership was transferred to the
function parameter. When the function ends, the parameter goes out of scope and drop is called.

From The Rust Book (Chapter 4.1):
"Passing a variable to a function will move or copy, just as assignment does."

To use a value after passing it to a function, either return it, pass a reference, or clone it."#,
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
            commentary: r#"References allow you to refer to a value without taking ownership. The & creates a reference,
and this is called borrowing. Immutable references allow reading but not modifying the data.
You can have multiple immutable references to the same value simultaneously.

From The Rust Book (Chapter 4.2):
"We call having references as function parameters borrowing. The action of creating a reference
is called borrowing."

This prevents the original owner from being invalidated while still providing access to the data."#,
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
            commentary: r#"Mutable references (&mut) allow modifying borrowed data. You can only have ONE mutable reference
to a value in a scope, and no immutable references at the same time. This prevents data races at
compile time - a key feature of Rust's memory safety.

From The Rust Book (Chapter 4.2):
"If you have a mutable reference to a value, you can have no other references to that value."

The restriction prevents simultaneous mutable access that could cause race conditions."#,
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
            commentary: r#"Multiple immutable references are allowed because reading data doesn't cause conflicts - many
readers can safely access the same data simultaneously. This is safe because the data cannot be
modified while these references exist.

From The Rust Book (Chapter 4.2):
"We can have multiple immutable references because no one who is just reading the data has the
ability to affect anyone else's reading of the data."

This is similar to reader-writer locks but enforced at compile time with zero runtime cost."#,
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
            commentary: r#"The scope of a reference lasts from where it's introduced until its last use. This example shows
Non-Lexical Lifetimes (NLL) - r1 and r2 become invalid after their last use, allowing r3 to
borrow mutably. You cannot have mutable and immutable references active simultaneously.

From The Rust Book (Chapter 4.2):
"A reference's scope starts from where it is introduced and continues through the last time that
reference is used."

This is smarter than old scope-based rules and allows more flexible borrowing patterns."#,
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
            commentary: r#"Slices are references to a contiguous sequence of elements in a collection. String slices (&str)
are immutable references to part of a String. The slice stores a pointer to the starting position
and a length. Slices prevent the original data from being modified while the slice exists.

From The Rust Book (Chapter 4.3):
"String slices are a reference to part of a String. A string slice is written using the range
syntax within brackets."

Common gotcha: Indexing must be on valid UTF-8 character boundaries or it will panic."#,
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
            commentary: r#"Rust's compiler prevents dangling references - references that point to deallocated memory. In
the commented code, s would be dropped at the end of dangle(), making the returned reference invalid.
The borrow checker catches this at compile time. The solution is to return ownership of the value.

From The Rust Book (Chapter 4.2):
"The compiler guarantees that references will never be dangling references: if you have a reference
to some data, the compiler will ensure that the data will not go out of scope before the reference."

This eliminates an entire class of bugs common in languages like C and C++."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
