// Packages, Crates, and Modules: Code organization in Rust
//
// This module demonstrates Rust's module system:
// - examples.rs - Example definitions
// - runners.rs - Example execution functions

mod examples;
mod runners;

pub use examples::get_examples;
pub use runners::run_example;

// Example module structure for demonstration
pub mod garden {
    pub mod vegetables {
        #[derive(Debug)]
        pub struct Carrot {
            pub length_cm: u32,
        }

        impl Carrot {
            pub fn new(length_cm: u32) -> Self {
                Carrot { length_cm }
            }
        }
    }

    pub mod flowers {
        #[derive(Debug)]
        pub struct Rose {
            pub color: String,
        }
    }
}
