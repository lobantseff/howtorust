// Traits: Defining shared behavior
//
// This module demonstrates traits and trait bounds:
// - examples.rs - Example definitions
// - runners.rs - Example execution functions

mod examples;
mod runners;

pub use examples::get_examples;
pub use runners::run_example;

// Example traits and types for demonstration
pub trait Summary {
    fn summarize(&self) -> String;

    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Article {
    pub headline: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
