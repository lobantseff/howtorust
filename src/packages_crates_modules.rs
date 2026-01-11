// Packages, Crates, and Modules: Code organization in Rust

use crate::{Difficulty, Example};

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

mod private_module {
    pub fn public_function() {
        println!("This is a public function in a private module");
    }

    fn private_function() {
        println!("This is private");
    }
}

pub fn get_examples() -> Vec<Example> {
    vec![
        // Beginner examples
        Example {
            name: "basic_module",
            description: "Creating and using basic modules",
            code: r#"mod sound {
    pub fn guitar() {
        println!("🎸 Guitar sound!");
    }
}

sound::guitar();"#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "nested_modules",
            description: "Nested module paths",
            code: r#"// Using the garden module
use howrust::packages_crates_modules::garden::vegetables::Carrot;

let carrot = Carrot::new(15);
println!("Carrot length: {} cm", carrot.length_cm);"#,
            difficulty: Difficulty::Beginner,
        },
        // Intermediate examples
        Example {
            name: "use_keyword",
            description: "Bringing paths into scope with 'use'",
            code: r#"use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Blue", 10);
scores.insert("Red", 50);
println!("Scores: {:?}", scores);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "use_as",
            description: "Renaming imports with 'as'",
            code: r#"use std::collections::HashMap as Map;

let mut data = Map::new();
data.insert("key", "value");
println!("{:?}", data);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "pub_use",
            description: "Re-exporting with pub use",
            code: r#"// This module re-exports items for convenience
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

// Re-export for easier access
pub use front_of_house::hosting;

hosting::add_to_waitlist();"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "glob_imports",
            description: "Using glob operator to import all public items",
            code: r#"use std::collections::*;

let mut map = HashMap::new();
map.insert(1, "one");

let mut set = HashSet::new();
set.insert(42);

println!("HashMap: {:?}, HashSet: {:?}", map, set);"#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "module_visibility",
            description: "Public vs private module items",
            code: r#"mod my_mod {
    pub fn public_fn() {
        println!("Public function");
        private_fn();
    }

    fn private_fn() {
        println!("Private function called from within");
    }
}

my_mod::public_fn();
// my_mod::private_fn();  // Error: private!"#,
            difficulty: Difficulty::Intermediate,
        },
        // Advanced examples
        Example {
            name: "super_keyword",
            description: "Using 'super' to access parent module",
            code: r#"mod parent {
    pub fn parent_function() {
        println!("Called from parent");
    }

    pub mod child {
        pub fn child_function() {
            println!("Called from child");
            super::parent_function();
        }
    }
}

parent::child::child_function();"#,
            difficulty: Difficulty::Advanced,
        },
        Example {
            name: "struct_privacy",
            description: "Struct field visibility",
            code: r#"mod shapes {
    pub struct Rectangle {
        pub width: u32,
        height: u32,  // private!
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }
    }
}

let rect = shapes::Rectangle::new(30, 50);
println!("Width: {}, Area: {}", rect.width, rect.area());
// println!("{}", rect.height);  // Error: private field"#,
            difficulty: Difficulty::Advanced,
        },
    ]
}

pub fn run_example(name: &str) {
    match name {
        "basic_module" => basic_module(),
        "nested_modules" => nested_modules(),
        "use_keyword" => use_keyword(),
        "use_as" => use_as(),
        "pub_use" => pub_use(),
        "glob_imports" => glob_imports(),
        "module_visibility" => module_visibility(),
        "super_keyword" => super_keyword(),
        "struct_privacy" => struct_privacy(),
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_module() {
    mod sound {
        pub fn guitar() {
            println!("🎸 Guitar sound!");
        }
    }

    sound::guitar();
}

fn nested_modules() {
    use crate::packages_crates_modules::garden::vegetables::Carrot;

    let carrot = Carrot::new(15);
    println!("Carrot length: {} cm", carrot.length_cm);
}

fn use_keyword() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 50);
    println!("Scores: {:?}", scores);
}

fn use_as() {
    use std::collections::HashMap as Map;

    let mut data = Map::new();
    data.insert("key", "value");
    println!("{:?}", data);
}

fn pub_use() {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("Added to waitlist");
            }
        }
    }

    pub use front_of_house::hosting;

    hosting::add_to_waitlist();
}

fn glob_imports() {
    use std::collections::*;

    let mut map = HashMap::new();
    map.insert(1, "one");

    let mut set = HashSet::new();
    set.insert(42);

    println!("HashMap: {:?}, HashSet: {:?}", map, set);
}

fn module_visibility() {
    mod my_mod {
        pub fn public_fn() {
            println!("Public function");
            private_fn();
        }

        fn private_fn() {
            println!("Private function called from within");
        }
    }

    my_mod::public_fn();
}

fn super_keyword() {
    mod parent {
        pub fn parent_function() {
            println!("Called from parent");
        }

        pub mod child {
            pub fn child_function() {
                println!("Called from child");
                super::parent_function();
            }
        }
    }

    parent::child::child_function();
}

fn struct_privacy() {
    mod shapes {
        pub struct Rectangle {
            pub width: u32,
            height: u32,
        }

        impl Rectangle {
            pub fn new(width: u32, height: u32) -> Self {
                Rectangle { width, height }
            }

            pub fn area(&self) -> u32 {
                self.width * self.height
            }
        }
    }

    let rect = shapes::Rectangle::new(30, 50);
    println!("Width: {}, Area: {}", rect.width, rect.area());
}
