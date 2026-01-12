// Async Programming: Asynchronous execution with async/await
//
// This module demonstrates Rust's async programming model:
// - src/async_prog/mod.rs (this file) - Module root, re-exports public API
// - src/async_prog/examples.rs - Example definitions
// - src/async_prog/runners.rs - Example execution functions
//
// Async in Rust allows writing concurrent code that doesn't block threads,
// enabling efficient I/O-bound operations and high-concurrency scenarios.

mod examples;
mod runners;

// Re-export the public API
pub use examples::get_examples;
pub use runners::run_example;
