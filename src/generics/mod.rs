// Generics: Writing flexible, reusable code
//
// This module demonstrates generic types and functions:
// - examples.rs - Example definitions
// - runners.rs - Example execution functions

mod examples;
mod runners;

pub use examples::get_examples;
pub use runners::run_example;

// Example generic structures for demonstration
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}
