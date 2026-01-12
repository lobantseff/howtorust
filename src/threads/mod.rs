// Threads: Concurrent execution with threads
//
// This module demonstrates Rust's thread model:
// - src/threads/mod.rs (this file) - Module root, re-exports public API
// - src/threads/examples.rs - Example definitions
// - src/threads/runners.rs - Example execution functions
//
// Threads in Rust provide safe concurrent execution through
// ownership and type system guarantees.

mod examples;
mod runners;

// Re-export the public API
pub use examples::get_examples;
pub use runners::run_example;
