// Lifetimes: Validating references
//
// This module demonstrates lifetime annotations and borrowing:
// - examples.rs - Example definitions
// - runners.rs - Example execution functions

mod examples;
mod runners;

pub use examples::get_examples;
pub use runners::run_example;
