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
            commentary: r#"Modules are Rust's primary code organization mechanism, creating namespaces that group related functionality together. The mod keyword declares a module, and pub makes items visible outside their parent scope. By default, everything in Rust is private to its parent module - this privacy-by-default encourages encapsulation and explicit API design. You access module items using the :: path separator, like sound::guitar(), which clearly shows where functionality lives in your code's namespace hierarchy.

Modules form a tree structure rooted at the crate's entry point (main.rs for binaries, lib.rs for libraries). Each module can contain functions, structs, enums, constants, traits, and even nested modules. The module system is purely a compile-time construct - it doesn't affect runtime performance or memory layout. The compiler uses modules to organize symbol names and enforce visibility rules, but after compilation, everything is just machine code. This makes Rust's module system a true zero-cost abstraction.

Privacy rules operate at module boundaries, not file boundaries. Code within a module can access all items in that module and its child modules, regardless of pub annotations. Only code outside the module needs pub to cross the module boundary. This creates clear encapsulation: implementation details stay hidden inside modules, while the public API is explicitly marked. The privacy system prevents you from accidentally depending on internal implementation details that might change.

In practice, modules help you organize code logically, separate concerns, control API surface area, and avoid name collisions. Small projects might have just a few modules in a single file, while large projects organize modules across many files and directories. The module system scales from simple programs to massive codebases. When designing modules, think about what belongs together conceptually, what should be public API versus implementation details, and how to make your code's structure discoverable to other developers."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "nested_modules",
            description: "Nested module paths",
            code: r#"// Using the garden module
use howrust::packages_crates_modules::garden::vegetables::Carrot;

let carrot = Carrot::new(15);
println!("Carrot length: {} cm", carrot.length_cm);"#,
            commentary: r#"Nested modules create hierarchical organization, letting you structure code into logical categories and subcategories. Access nested items using the :: path separator, chaining module names like packages_crates_modules::garden::vegetables::Carrot. Each level in the path must be marked pub for external access - if any intermediate module is private, the entire path becomes inaccessible from outside. This forces you to make conscious decisions about your API boundaries at every level of nesting.

Paths in Rust come in two forms: absolute paths starting from the crate root (use crate::module::item), and relative paths starting from the current module (use super::sibling or self::child). Absolute paths are more explicit and refactor-proof, while relative paths can be more convenient within closely related modules. External crates are accessed by name, as in this example where howrust is the crate name. The compiler resolves these paths at compile time, so there's no runtime lookup cost.

Nested modules map naturally to filesystem hierarchies - a module can be defined in either module_name.rs or module_name/mod.rs with child modules in the directory. This file-based organization scales to large projects with thousands of modules. However, the module structure is independent of file structure - you can define multiple modules in one file, or split a logical module across files. The module tree defines namespacing and visibility, while the filesystem just determines where code is written.

In practice, nested modules help you organize large codebases into understandable chunks. Group related functionality together, separate stable public APIs from volatile internal implementation, and create clear dependency boundaries. Well-organized module hierarchies make code navigable and maintainable. Common patterns include organizing by feature (user, database, network), by layer (models, views, controllers), or by domain concepts. The key is consistency - pick an organization strategy and stick to it throughout your project."#,
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
            commentary: r#"The use keyword brings paths into scope, creating local bindings that eliminate repetitive fully-qualified names. Once you write use std::collections::HashMap, you can refer to the type as just HashMap throughout that scope. This is similar to import in Python or Java but more flexible - use works at any scope level (module, function, even block), and the bindings follow Rust's normal shadowing rules. You can use paths from any visible module, including the standard library, external crates, and your own code.

Rust has idiomatic conventions for what to import. For structs, enums, and traits, import the type itself (use std::collections::HashMap). For functions, import the parent module (use std::collections; then call collections::HashMap::new). This convention makes it clearer where functions come from when reading code - you see the module name at the call site. However, it's just convention; the compiler doesn't enforce it. Some teams or projects establish their own conventions based on readability and preferences.

The use statement is purely a convenience feature - it doesn't change visibility or bring items into your module's public API (unless combined with pub use). It's a compile-time name resolution mechanism that expands to full paths during compilation. This means there's zero runtime cost for using imports. The compiler simply replaces each usage with the full path internally. You can have as many use statements as you want without affecting binary size or performance.

In practice, use statements make code more readable by reducing visual clutter from long paths. They're especially valuable when working with deeply nested modules or external crates with long names. Common pattern: group related imports at the top of files, keep them alphabetically sorted, and remove unused imports (the compiler will warn about these). Modern IDEs and tools like rustfmt can automatically organize imports for consistency across your codebase."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "use_as",
            description: "Renaming imports with 'as'",
            code: r#"use std::collections::HashMap as Map;

let mut data = Map::new();
data.insert("key", "value");
println!("{:?}", data);"#,
            commentary: r#"The as keyword renames imports to avoid name conflicts or provide more convenient names in the current scope. When you write use std::collections::HashMap as Map, you're creating a local alias that only exists in that scope - the original name HashMap is not automatically available unless separately imported. This is particularly useful when two crates define types with the same name, like std::io::Result and a custom Result type, allowing you to import both as IoResult and AppResult without collision.

Renaming works with any item - types, functions, modules, traits, even entire paths. You can rename modules to shorter names for convenience (use very_long_module_name::deeply::nested as short), making subsequent code more readable. The renamed binding follows normal scoping rules - it's visible only within the scope where the use statement appears. If you rename an item in a module, it doesn't affect how that item is referenced elsewhere in your codebase.

The ability to rename is crucial for working with multiple versions of the same crate or similar APIs from different sources. For example, if you're migrating from one database library to another, you might temporarily import both with different names while gradually updating your code. This flexibility makes refactoring and library migration much less painful than in languages where name conflicts require more invasive changes.

Common pattern: Use short, clear names that make sense in context. If you're working extensively with a specific module, renaming it to something brief can significantly improve readability. However, don't overuse renaming - having too many aliases can make code confusing for other developers. Reserve it for resolving actual conflicts or genuinely improving clarity. When in doubt, use the original name unless there's a specific reason to change it."#,
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
            commentary: r#"Re-exporting with pub use brings an item into the current module's public API, making it available to external code as if it were defined in the current module. This decouples your public API structure from your internal code organization. Libraries commonly use this to provide a convenient, flat API at the crate root while organizing implementation details in a deep module hierarchy. For example, users can write use mycrate::Widget instead of use mycrate::internal::ui::components::Widget, even though the latter is where Widget is actually defined.

This pattern is essential for evolving APIs without breaking backward compatibility. You can restructure internal modules, move items between files, and reorganize code freely as long as you maintain the same pub use exports at your public boundary. This gives you flexibility to improve code organization over time while preserving a stable API. Many popular crates use aggressive re-exporting to present a clean, simple API surface despite complex internal architecture.

The prelude pattern takes this further - many crates provide a prelude module that re-exports commonly used items in one place. Users can then import everything they typically need with a single use mycrate::prelude::*. This is considered good practice for libraries because it makes getting started quick and easy while still allowing fine-grained imports for advanced users. The standard library itself uses this pattern with std::prelude.

In practice, use pub use to shape your public API intentionally. Think about what paths users should import from - make common paths short and discoverable. Hide internal implementation details in private modules, exposing only what users need. This creates a separation between interface and implementation that makes your code more maintainable and your API more stable. Document your public exports well, as they form the contract between your crate and its users."#,
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
            commentary: r#"The glob operator (*) imports all public items from a module into the current scope with use std::collections::*. This brings in every public function, type, trait, and constant, making them available without qualification. While convenient, glob imports make code harder to understand because you can't tell where names come from by looking at imports. They also create name collision risks - if two glob-imported modules export items with the same name, you'll get a compile error. The ambiguity increases maintenance burden for anyone reading or modifying the code later.

Glob imports are appropriate in specific contexts. Test modules commonly use them (use super::*) to import everything from the parent module being tested, keeping test code concise. Prelude modules are designed for glob import - they carefully curate a small set of commonly needed items. The standard library's prelude (automatically imported everywhere) is the canonical example. Custom preludes follow this pattern for crates where certain items are used pervasively. Documentation examples sometimes use globs to reduce clutter when the focus is on other concepts.

The compiler tracks which items are imported via glob, so unused imports still generate warnings. However, IDEs struggle to provide good autocomplete and navigation for glob imports since they can't determine statically what's available without analyzing the entire imported module. This degrades the development experience. The readability cost is real - when you see a type or function, you want to quickly identify its origin, and glob imports obscure that information.

Best practice: Avoid glob imports in production code except for preludes and test modules. Prefer explicit imports even if it means more lines - the clarity is worth it. Modern editors make adding imports easy with auto-complete and quick fixes. If you find yourself importing many items from one module, consider whether your code might be better organized or whether those items should be grouped differently. Explicit imports are self-documenting and make code reviews easier by showing exactly what dependencies exist."#,
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
            commentary: r#"Rust's privacy system is private-by-default - every function, struct, enum, constant, trait, and module is private to its parent module unless explicitly marked pub. This design philosophy encourages intentional API design by making you consciously decide what to expose. Private items can be freely changed without breaking external code since no one outside the module can depend on them. Public items form a contract that should remain stable across versions to avoid breaking downstream users.

Privacy is enforced at module boundaries, not file boundaries or other units. All code within a module can access everything in that module, regardless of pub annotations. Child modules can access private items in their ancestors. Sibling modules cannot access each other's private items. This creates a clear encapsulation boundary where implementation details remain hidden. The privacy rules are checked at compile time, imposing zero runtime cost - there's no runtime reflection or access control mechanism.

The module system's privacy rules enable fearless refactoring within module boundaries. You can completely restructure private code, rename functions, change data structures, and optimize implementations without worrying about breaking external code. Only changes to public items require consideration of backward compatibility. This separation of public interface from private implementation is fundamental to maintaining large codebases over time.

In practice, keep things private by default and only mark items pub when they're genuinely needed as part of your public API. For libraries, this means carefully curating what you expose - a smaller API surface is easier to maintain and evolve. For applications, privacy helps organize code into modules with clear responsibilities. A common pattern is to have a small public interface at module boundaries with extensive private implementation inside. This creates loose coupling between modules while maintaining tight cohesion within them."#,
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
            commentary: r#"The super keyword creates relative paths that refer to the parent module, working like the .. directory operator in filesystem paths. When you write super::parent_function(), you're accessing an item one level up in the module tree. This is particularly useful when child modules need to access items in their parent or sibling modules. Relative paths with super are more maintainable than absolute paths because they continue working when you move entire module hierarchies around in your codebase.

You can chain super to go up multiple levels: super::super::grandparent_function() goes up two levels. However, excessive chaining suggests your module structure might be too deeply nested. The super keyword works with Rust's privacy rules - a child module can access private items in its parent modules, making super the natural way to reference that functionality. This is more explicit and refactoring-friendly than using absolute paths starting from crate::.

The most common use case for super is in test modules. By convention, tests are often placed in a child module (mod tests) within the same file as the code they test. Tests need to access the parent module's functions, including private ones. Using super::function_to_test makes this clear and automatic. When you move the file or refactor the module hierarchy, these relative paths automatically adjust, unlike absolute paths that might break.

In practice, choose between super (relative) and crate:: (absolute) paths based on context. Use super when accessing closely related modules, especially within the same file or subsystem. Use absolute paths for accessing distinct, loosely coupled modules where you want clarity about exactly what's being referenced. The key insight: super expresses a structural relationship (this is my parent), while absolute paths express a logical dependency (I need this specific module). Both have their place in idiomatic Rust code."#,
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
            commentary: r#"Struct field visibility is independent of struct visibility - pub struct only makes the type name public, while fields remain private by default. You can mark individual fields pub to expose them selectively, creating mixed public/private field structs. This fine-grained control lets you expose some data directly while hiding implementation details or maintaining invariants through methods. For example, you might expose width publicly for direct access while keeping height private with a validated setter that ensures it's never negative.

This visibility model enables proper encapsulation in ways many languages struggle with. In languages where all struct fields are public by default, it's easy to accidentally create dependencies on internal representation. Rust's private-by-default fields prevent this. You can freely change private field types, representations, or organizations without breaking external code. If a field is public, changing it is a breaking API change. This forces you to think carefully about what truly belongs in your public interface.

The common pattern is to keep all fields private and provide public constructors and accessor methods. This gives you complete control over initialization (ensuring invariants are established) and access patterns (allowing validation, logging, or computed properties). Methods can evolve without breaking API - you might start with a simple field access, then add validation or caching later. If the field were public, such evolution would be impossible without breaking changes.

For structs with all public fields, you get automatic struct literal construction from outside the module (Rectangle { width: 10, height: 20 }). With private fields, you must provide a constructor function or builder pattern. This is actually a feature - it centralizes object creation, making it easy to add validation, logging, or complex initialization logic later. The builder pattern is particularly popular for structs with many fields or optional configuration, providing a fluent API while maintaining encapsulation."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
