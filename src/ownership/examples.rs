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
            commentary: r#"Ownership is Rust's most unique and powerful feature, solving memory safety without garbage collection. Each value in Rust has exactly one owner at any time - the variable that holds it. When we write 'let s2 = s1;', we're not copying the data but transferring ownership. The original variable s1 becomes invalid, and attempting to use it results in a compile-time error. This prevents double-free bugs where two variables might try to free the same memory.

Under the hood, when String data is on the heap, both the pointer and the ownership are transferred. The stack data (pointer, length, capacity) is copied, but Rust marks s1 as moved, preventing its use. This is different from a shallow copy because the original variable is invalidated. When s2 goes out of scope, only it calls drop to free the memory - s1 can't, because it's no longer valid. This mechanism ensures exactly one cleanup per allocation.

This design eliminates entire classes of bugs common in C++ like use-after-free, double-free, and dangling pointers - all caught at compile time with zero runtime cost. Unlike garbage collected languages, there's no performance penalty, no stop-the-world pauses, and memory is freed deterministically the moment a variable goes out of scope. This is what Rust calls a "zero-cost abstraction" - safety without runtime overhead. The ownership system also forms the foundation for Rust's concurrency safety guarantees.

In practice, use moves when transferring ownership permanently, like passing data to another function or thread. If you need the original variable afterward, either use references (borrowing) or explicitly clone. The move semantics make ownership clear and prevent accidental sharing of mutable state, which is crucial for Rust's thread safety guarantees. This explicit transfer of ownership makes data flow through your program transparent and predictable."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "clone_vs_move",
            description: "Cloning vs moving data",
            code: r#"let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy
println!("s1: {}, s2: {}", s1, s2);  // Both valid!"#,
            commentary: r#"The clone method creates a deep copy of heap data, allocating new memory and copying all the data, allowing both variables to remain valid and independent. When you call s1.clone(), Rust creates a complete duplicate of the String's heap-allocated buffer, not just the stack metadata. This means both s1 and s2 own separate copies of the data and can be used freely without interfering with each other. Each variable will independently free its own memory when it goes out of scope.

This operation is more expensive than moving since it actually copies the heap data - for a large String, this could mean copying megabytes of memory. The cost is proportional to the size of the data being cloned. In contrast, a move just copies the stack metadata (24 bytes for a String on 64-bit systems) and invalidates the source. The compiler doesn't perform clones automatically - you must explicitly call clone(), making the performance cost visible in your code.

Clone is appropriate when you genuinely need multiple independent owners of the same data - for example, when sending data to multiple threads, caching values, or implementing undo functionality. However, it's often overused by beginners who aren't yet comfortable with borrowing. In many cases, using references (&T or &mut T) is more efficient and idiomatic. The Clone trait is implemented for most standard library types, but custom types must explicitly derive or implement it.

Best practice: Don't clone unnecessarily. Consider whether you actually need independent copies or if borrowing would suffice. Profile before optimizing, but be aware that excessive cloning can significantly impact performance, especially in hot loops. If you find yourself cloning frequently in performance-critical code, it's often a sign that you should restructure your ownership patterns or use reference-counted pointers like Rc or Arc."#,
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
            commentary: r#"Passing a value to a function moves ownership using the same semantics as variable assignment. After calling takes_ownership(s), the variable s in the outer scope is no longer valid because ownership was permanently transferred to the function parameter. When the function ends, the parameter goes out of scope and drop is automatically called, freeing the memory. This makes it impossible to accidentally use freed memory - the compiler won't let you use s after the move.

This behavior applies to heap-allocated types like String, Vec, Box, and custom types without the Copy trait. Stack-only types like integers, floats, and tuples of Copy types are copied instead of moved, so they remain valid after being passed to functions. The distinction between Copy and move semantics is determined by whether the type implements the Copy trait, which is only allowed for types that can be safely copied bit-for-bit without allocating new resources.

The move-by-default semantics prevent common bugs found in other languages. In C++, passing by value copies the object, which can be expensive and surprising. In Java, everything is a reference, hiding ownership. Rust makes ownership transfer explicit and compile-time checked. This is particularly important for expensive resources like file handles, network connections, or large data structures where implicit copies would be prohibitively expensive.

To maintain access after passing to a function, you have three options: return ownership back from the function, pass a reference (&T or &mut T) to borrow temporarily, or explicitly clone before passing. Each option communicates different intent - returning ownership suggests transformation, borrowing suggests temporary access, and cloning suggests independent copies are needed. Choosing the right pattern makes your code's ownership semantics clear and efficient."#,
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
            commentary: r#"References allow you to refer to a value without taking ownership, solving the problem of passing data to functions while keeping it usable afterward. The & operator creates a reference, and using references as function parameters is called borrowing. The function borrows the data temporarily but doesn't own it, so when the function ends, the data isn't dropped. Immutable references (&T) allow reading the data but not modifying it, making them safe to have multiple simultaneous borrows.

Under the hood, a reference is essentially a pointer, but Rust's borrow checker enforces strict rules at compile time. The referenced value cannot be moved or dropped while any references to it exist. The compiler tracks the lifetime of references to ensure they never outlive the data they point to, preventing dangling pointer bugs. This compile-time checking means references have zero runtime cost compared to raw pointers - they're just memory addresses with compiler-enforced safety.

You can have unlimited immutable references to the same value simultaneously because reading is inherently safe - multiple readers can't cause data races or inconsistencies. This is similar to reader-writer locks in concurrent programming, but enforced at compile time with zero runtime overhead. The guarantee that the data won't change while immutable references exist enables powerful optimizations and makes reasoning about code behavior straightforward.

Borrowing is the default way to pass data in Rust when you don't need to transfer ownership. Use &T for functions that need to read data, &mut T for functions that need to modify it, and owned parameters (T) only when the function needs to consume or store the value. This pattern keeps your code efficient by avoiding unnecessary moves and clones while maintaining clear ownership semantics. Most Rust APIs are designed around borrowing, making it feel natural once you internalize the pattern."#,
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
            commentary: r#"Mutable references (&mut T) allow modifying borrowed data, providing a way to change values without taking ownership. However, Rust enforces a critical restriction: you can have ONLY ONE mutable reference to a value in a given scope, and no immutable references can exist at the same time. This "no aliasing with mutability" rule prevents data races at compile time, making Rust's memory safety guarantees extend to concurrent code. The compiler will reject any code that could potentially create multiple mutable references or mix mutable and immutable references.

This restriction prevents several classes of bugs. Data races occur when multiple threads access the same memory simultaneously with at least one performing a write, leading to undefined behavior. Rust's borrow checker makes data races impossible by enforcing that either you have multiple readers OR one writer, never both. This also prevents iterator invalidation bugs (modifying a collection while iterating), use-after-free bugs (one reference frees while another still points to it), and other aliasing problems that plague C and C++ code.

The mutable reference rules mirror reader-writer locks from concurrent programming, but they're enforced at compile time with zero runtime cost. There's no lock acquisition, no runtime checking, no performance penalty - just compile-time verification that your code is safe. The borrow checker's analysis is conservative: it may reject safe code it can't prove is correct, but it never accepts unsafe code. This is Rust's "fearless concurrency" - you can write parallel code with confidence that if it compiles, it won't have data races.

In practice, the single mutable reference restriction encourages better code design. Instead of spreading mutations throughout your code, you tend to localize them in specific scopes. If you need shared mutable state, Rust provides interior mutability patterns (Cell, RefCell, Mutex, RwLock) that make the synchronization explicit and type-checked. The key insight: Rust makes you think about mutation and sharing upfront, preventing bugs that would only appear at runtime in other languages."#,
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
            commentary: r#"Multiple immutable references to the same value are allowed simultaneously because reading data doesn't cause conflicts - many readers can safely access the same data without interfering with each other. This is fundamentally safe because the data is guaranteed not to change while these immutable references exist. You could have hundreds of immutable references active at once, and the code would be perfectly safe. The compiler guarantees that no mutable reference can exist while any immutable references are active, ensuring the data remains stable.

This multiple-readers pattern mirrors the semantics of reader-writer locks in concurrent programming, where multiple threads can hold read locks simultaneously. However, Rust enforces this at compile time rather than runtime, eliminating the overhead of actual lock acquisition and release. There's no runtime checking, no atomic operations, no waiting - just compile-time verification that produces zero-cost machine code. This makes immutable borrowing extremely efficient for passing data around your program.

The guarantee of immutability while references exist enables powerful compiler optimizations. The compiler can cache values, reorder operations, and make assumptions about data stability that would be unsafe with mutable access. This is one reason why Rust code can be as fast as C - the compiler has strong guarantees about aliasing and mutability that enable aggressive optimization. Languages without these guarantees must be conservative in their optimizations or use expensive runtime checks.

In practice, immutable references are the default way to share data in Rust. When designing APIs, prefer taking &T parameters over owned values whenever possible - it's more flexible and efficient. Functions that take references can be called with owned values, other references, or dereferenced smart pointers. This makes immutable borrowing the most composable and ergonomic choice for most function parameters, especially in generic contexts where the caller's ownership needs may vary."#,
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
            commentary: r#"The scope of a reference lasts from where it's introduced until its last use, not until the end of the enclosing block. This example demonstrates Non-Lexical Lifetimes (NLL), a feature added in Rust 2018 that made the borrow checker significantly smarter. The immutable references r1 and r2 end their lifetimes after the println! that uses them, even though they're still in scope lexically. This allows r3 to borrow mutably afterward. Before NLL, this code wouldn't compile because r1 and r2 would live until the block's end.

NLL uses control flow analysis to determine the actual lifetime of references based on how they're used. It tracks where each reference is last accessed and ends its lifetime there, rather than at the end of the lexical scope. This makes the borrow checker much more flexible and ergonomic without sacrificing safety. The compiler analyzes all possible control flow paths to ensure references don't outlive their data, handling complex scenarios like early returns, loops, and conditionals correctly.

This smarter analysis eliminates many false positives that the old borrow checker would reject. Code that's clearly safe to humans but couldn't be proven safe under simple lexical scoping rules now compiles. The key restriction remains: you cannot have mutable and immutable references active simultaneously. However, "active" is now precisely defined by actual usage rather than lexical scope, making the rules feel more intuitive and less restrictive.

In practice, NLL makes Rust much more ergonomic to write. You no longer need artificial scope blocks ({ }) to manually control reference lifetimes in most cases. The compiler figures it out based on your actual usage patterns. This is especially helpful in functions with multiple borrowing phases or complex control flow. The general pattern: use immutable references first, then after they're done, you can take a mutable reference. This ordering matches how most algorithms naturally work."#,
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
            commentary: r#"Slices are references to a contiguous sequence of elements in a collection, providing a view into part of the data without taking ownership. String slices (&str) are fat pointers containing two words: a pointer to the starting byte and a length in bytes. When you create &s[0..5], you're not copying the data - you're creating a reference to a range within the original String. The slice borrows the data, so the original String remains valid and can be used after the slice is created (as long as the borrow rules are followed).

Under the hood, a slice is implemented as a pointer and length stored on the stack, making it very cheap to create and pass around. For &str specifically, the slice must point to valid UTF-8 data, which Rust guarantees at the type system level. Slices don't own their data, so they don't free anything when they go out of scope. The memory is managed by whatever owns the underlying collection (in this case, the String s). This makes slices extremely efficient for working with subsets of data without allocation or copying.

Slices enforce Rust's borrowing rules - while an immutable slice exists, the underlying collection cannot be mutated. This prevents iterator invalidation bugs where modifying a collection while iterating over it causes undefined behavior. The borrow checker ensures that if you have a slice pointing into a String, you can't modify or move that String until the slice is done being used. This compile-time guarantee eliminates a whole class of bugs common in C and C++ string handling.

Critical gotcha: String slice indices must be on valid UTF-8 character boundaries, not arbitrary byte positions. Since Rust strings are UTF-8 encoded, a single character might be multiple bytes. Slicing in the middle of a multi-byte character will panic. Use methods like chars().nth() or char_indices() when you need to work with Unicode characters rather than bytes. For byte slices (&[u8]), any index is valid. The difference between &str and &[u8] captures this UTF-8 validity requirement in the type system."#,
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
            commentary: r#"Rust's compiler prevents dangling references - references that point to deallocated memory - through its borrow checker's lifetime analysis. In the commented dangle function, the String s is created inside the function and would be dropped when the function returns, deallocating its heap memory. Returning a reference &s would create a pointer to freed memory, a critical bug that causes undefined behavior in C and C++. The Rust compiler detects this at compile time and rejects the code with a clear error message explaining that the borrowed value doesn't live long enough.

The borrow checker's lifetime analysis tracks how long each value lives and ensures no reference can outlive the data it points to. When you try to return a reference to a local variable, the compiler recognizes that the local variable's lifetime ends at the function boundary, but the returned reference would need to live longer. This violation of lifetime rules is caught before your code ever runs. The error message even suggests the solution: return ownership of the value itself rather than a reference to it.

This compile-time prevention of dangling references eliminates an entire class of memory safety bugs that are notoriously difficult to debug in C and C++. Use-after-free bugs are a major source of security vulnerabilities and crashes in systems programming. Tools like Valgrind can catch some of these at runtime, but Rust prevents them entirely at compile time. There's no runtime overhead, no sanitizers needed, no mysterious crashes in production - just code that won't compile if it could create dangling references.

The solution patterns are clear: return owned values to transfer ownership out of a function, return references to data that outlives the function (like references passed in as parameters), or use smart pointers like Box, Rc, or Arc when you need heap-allocated data with flexible ownership. Each approach has different tradeoffs in terms of ownership, performance, and flexibility. The key insight: Rust forces you to be explicit about who owns data and how long it lives, catching ownership bugs at the earliest possible point - during compilation."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
