// HowRust: Interactive Rust Tutorial and Cheatsheet
// Main library exposing all tutorial modules

pub mod closures;
pub mod error_handling;
pub mod generics;
pub mod iterators;
pub mod lifetimes;
pub mod ownership;
pub mod packages_crates_modules;
pub mod traits;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::Intermediate => write!(f, "Intermediate"),
            Difficulty::Advanced => write!(f, "Advanced"),
        }
    }
}

pub struct Example {
    pub name: &'static str,
    pub description: &'static str,
    pub code: &'static str,
    pub commentary: &'static str,
    pub difficulty: Difficulty,
}

pub struct Chapter {
    pub name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        name: "ownership",
        title: "Understanding Ownership",
        description: "Learn Rust's ownership system, borrowing, and references",
    },
    Chapter {
        name: "modules",
        title: "Packages, Crates, and Modules",
        description: "Organize code with modules, crates, and packages",
    },
    Chapter {
        name: "generics",
        title: "Generic Types",
        description: "Write flexible code with generic type parameters",
    },
    Chapter {
        name: "traits",
        title: "Traits",
        description: "Define shared behavior with traits",
    },
    Chapter {
        name: "lifetimes",
        title: "Lifetimes",
        description: "Validate references with lifetime annotations",
    },
    Chapter {
        name: "errors",
        title: "Error Handling",
        description: "Handle errors with Result, Option, and panic",
    },
    Chapter {
        name: "closures",
        title: "Closures",
        description: "Use anonymous functions that capture their environment",
    },
    Chapter {
        name: "iterators",
        title: "Iterators",
        description: "Process sequences of values efficiently",
    },
];

pub fn get_chapter_examples(chapter: &str) -> Option<Vec<Example>> {
    match chapter {
        "ownership" => Some(ownership::get_examples()),
        "modules" => Some(packages_crates_modules::get_examples()),
        "generics" => Some(generics::get_examples()),
        "traits" => Some(traits::get_examples()),
        "lifetimes" => Some(lifetimes::get_examples()),
        "errors" => Some(error_handling::get_examples()),
        "closures" => Some(closures::get_examples()),
        "iterators" => Some(iterators::get_examples()),
        _ => None,
    }
}

pub fn run_chapter_example(chapter: &str, example: &str) {
    match chapter {
        "ownership" => ownership::run_example(example),
        "modules" => packages_crates_modules::run_example(example),
        "generics" => generics::run_example(example),
        "traits" => traits::run_example(example),
        "lifetimes" => lifetimes::run_example(example),
        "errors" => error_handling::run_example(example),
        "closures" => closures::run_example(example),
        "iterators" => iterators::run_example(example),
        _ => println!("Chapter '{}' not found", chapter),
    }
}

pub fn find_chapter_by_name(name: &str) -> Option<&'static Chapter> {
    CHAPTERS.iter().find(|ch| ch.name == name)
}
