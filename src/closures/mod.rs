// Closures: Anonymous functions that capture their environment
//
// This module demonstrates Rust's subdirectory organization:
// - src/closures/mod.rs (this file) - Module root, re-exports public API
// - src/closures/examples.rs - Example definitions
// - src/closures/runners.rs - Example execution functions
//
// By using a directory structure, we can organize related functionality
// into separate files while keeping a clean public interface.

mod examples;
mod runners;

// Re-export the public API
pub use examples::get_examples;
pub use runners::run_example;
