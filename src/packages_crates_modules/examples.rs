// Packages, crates, and modules examples organized in a separate file

use crate::{Difficulty, Example};

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
            commentary: r#"Modules organize code into namespaces. The mod keyword creates a module, and pub makes items public.
By default, all items in a module are private to their parent module. Use the module::item syntax
to access items from outside the module.

From The Rust Book (Chapter 7.2):
"Modules let us organize code within a crate into groups for readability and easy reuse."

Modules form a tree structure, with the crate root (main.rs or lib.rs) at the top."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "nested_modules",
            description: "Nested module paths",
            code: r#"// Using the garden module
use howrust::packages_crates_modules::garden::vegetables::Carrot;

let carrot = Carrot::new(15);
println!("Carrot length: {} cm", carrot.length_cm);"#,
            commentary: r#"Modules can be nested to create hierarchical organization. Access nested items using :: path syntax.
Each level must be marked pub for external access. The full path starts from the crate name and
works down through the module tree.

From The Rust Book (Chapter 7.3):
"Paths can take two forms: absolute paths start from a crate root, relative paths start from the current module."

Nested modules help organize large projects into logical sections and subsections."#,
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
            commentary: r#"The use keyword brings paths into scope, eliminating the need to write full paths repeatedly. This
is similar to import in other languages. You can use absolute or relative paths. Convention is to
use parent module for functions but the type itself for structs/enums.

From The Rust Book (Chapter 7.4):
"We can bring a path into a scope once and then call the items in that path as if they're local items."

Idiomatic: use std::collections::HashMap, not use std::collections::HashMap::new."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "use_as",
            description: "Renaming imports with 'as'",
            code: r#"use std::collections::HashMap as Map;

let mut data = Map::new();
data.insert("key", "value");
println!("{:?}", data);"#,
            commentary: r#"The as keyword renames imports to avoid conflicts or provide shorter names. This is useful when
importing two items with the same name from different modules, or when you want a more convenient
name for frequently used types.

From The Rust Book (Chapter 7.4):
"We can use the as keyword to rename an item when we bring it into scope."

Common use case: Renaming Result types from different libraries that might conflict."#,
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
            commentary: r#"pub use re-exports items, making them available both internally and to external code. This is useful
for creating a convenient public API that differs from internal organization. Libraries use this to
expose a flat, user-friendly API while maintaining complex internal structure.

From The Rust Book (Chapter 7.4):
"We can combine pub and use. This technique is called re-exporting because we're bringing an item
into scope but also making that item available for others to bring into their scope."

Common pattern: Re-export commonly used items at the crate root for easier access."#,
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
            commentary: r#"The glob operator (*) imports all public items from a module. Use sparingly - it makes it unclear
where names come from and can cause name conflicts. Main use cases are preludes and test modules
where many items are needed.

From The Rust Book (Chapter 7.4):
"The glob operator will bring all public items defined in a path into scope."

Be cautious: Glob imports can make code harder to understand and maintain. Prefer explicit imports in production code."#,
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
            commentary: r#"By default, items are private to their parent module. The pub keyword makes them accessible from
outside. Privacy rules apply at module boundaries - code within a module can access all items in
that module and its descendants.

From The Rust Book (Chapter 7.2):
"All items (functions, methods, structs, enums, modules, and constants) are private by default."

This enforces encapsulation - implementation details stay hidden unless explicitly exposed."#,
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
            commentary: r#"The super keyword refers to the parent module, similar to .. in filesystem paths. This is useful
for relative paths when accessing items in parent or sibling modules. It makes paths more maintainable
when modules are reorganized.

From The Rust Book (Chapter 7.3):
"We can use super to construct relative paths that start in the parent module."

super is especially useful in tests to access private functions in the parent module being tested."#,
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
            commentary: r#"Struct fields can have individual visibility. pub struct makes the struct name public, but fields
are private unless marked pub. This allows controlled access - you can expose some fields while
keeping others as implementation details accessible only through methods.

From The Rust Book (Chapter 7.3):
"If we make a struct public, the fields will still be private. We can make each field public or not
on a case-by-case basis."

Common pattern: Keep all fields private and provide public constructor and accessor methods for encapsulation."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
