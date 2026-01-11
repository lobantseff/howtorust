// Ownership: Understanding Rust's ownership system
//
// This module demonstrates Rust's ownership, borrowing, and references:
// - examples.rs - Example definitions
// - runners.rs - Example execution functions

mod examples;
mod runners;

pub use examples::get_examples;
pub use runners::run_example;
