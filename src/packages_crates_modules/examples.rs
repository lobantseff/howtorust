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
            commentary: r#"📚 INTRODUCTION
Modules (mod) are Rust's primary code organization tool, creating
namespaces that group related functionality. The mod keyword declares
a module, pub makes items visible outside it, and :: accesses items
(sound::guitar()). By default, everything is private to its parent
module - this privacy-by-default forces explicit, intentional API design.
Modules form a tree rooted at the crate entry point (lib.rs or main.rs).

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Code organization problems without modules:
• Name collisions: multiple items with same name → conflict
• No encapsulation: everything accessible → tight coupling
• Unclear structure: flat code → hard to navigate
• No API boundary: can't separate public from private

Modules solve all this:
• Namespaces prevent name collisions
• Privacy-by-default enables encapsulation
• Hierarchical structure matches mental models
• Public/private creates clear API boundaries

Unlike languages with package-level or class-based organization,
Rust's modules are language-level, compile-time constructs with
zero runtime cost.

🔍 IMPORTANT DETAILS & INTRICACIES
Zero-Cost Abstraction: Modules are purely compile-time. The compiler
  uses them for namespacing and privacy checking, then generates plain
  machine code. No runtime overhead for module boundaries.

Privacy at Module Boundaries: Everything is private by default. Code
  inside a module can access everything in that module and descendants,
  regardless of pub. Only external code needs pub to cross boundaries.

Module Tree Structure: Modules form a tree:
  crate (root)
  ├── module_a
  │   ├── submodule_a1
  │   └── submodule_a2
  └── module_b

Paths: Access via :: (like sound::guitar). Absolute paths start with
  crate:: (crate::sound::guitar). Relative paths use super:: (parent)
  or self:: (current).

File Organization: Modules can be inline (mod foo { }), in foo.rs,
  or in foo/mod.rs. The compiler looks for these locations automatically.

💼 WHERE IT'S MOST USED
• Code organization: grouping related functions, types, constants
• API design: separating public interface from private implementation
• Name collision prevention: multiple items with same name in different modules
• Testing: mod tests for unit tests in same file
• Feature organization: payment, auth, database as separate modules

Common patterns:
• One file per module for large modules
• Inline modules for small helper code
• Private submodules for implementation details

✅ TAKEAWAY
Modules are Rust's fundamental organization unit - namespaces with
privacy boundaries. Use them to structure code logically, hide
implementation details, and prevent name collisions. Everything is
private by default, forcing explicit public APIs. Modules are zero-cost
at runtime - pure compile-time constructs. Think of modules as boxes
that contain related code and control what's visible to the outside
world. Good module structure makes codebases navigable and maintainable."#,
            difficulty: Difficulty::Beginner,
        },
        Example {
            name: "nested_modules",
            description: "Nested module paths",
            code: r#"// Using the garden module
use howrust::packages_crates_modules::garden::vegetables::Carrot;

let carrot = Carrot::new(15);
println!("Carrot length: {} cm", carrot.length_cm);"#,
            commentary: r#"📚 INTRODUCTION
Nested modules create hierarchical organization through multiple levels
of module nesting (module::submodule::item). Access nested items using
:: path separators, chaining module names like garden::vegetables::Carrot.
Each level in the path must be marked pub for external access - if any
intermediate module is private, the entire path breaks. This creates a
tree structure for organizing code into categories and subcategories.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Flat organization problems without nesting:
• Name collisions: utilities.rs becomes dumping ground → conflicts
• No categorization: everything at same level → poor discoverability
• Unclear relationships: can't express "X is part of Y"
• Large files: everything in one module → thousands of lines

Nested modules solve this:
• Hierarchical structure matches mental models (garden > vegetables > Carrot)
• Logical grouping with clear parent-child relationships
• Scalable organization for projects with thousands of items
• Each nesting level creates an encapsulation boundary

Unlike Java packages (reflect directory structure) or Python modules
(always map to files), Rust modules are purely logical - you choose
how to map them to files.

🔍 IMPORTANT DETAILS & INTRICACIES
Path Resolution: Two types of paths work identically after compilation:
  • Absolute: use crate::garden::vegetables::Carrot (from crate root)
  • Relative: use super::vegetables::Carrot (from current module)
  • External: use howrust::garden::vegetables::Carrot (different crate)
All resolve at compile time - zero runtime cost for path lookup.

Privacy Cascading: Every level must be pub for external access. If
  garden is pub but vegetables is private, garden::vegetables::Carrot
  is inaccessible from outside garden. This creates natural API boundaries
  at each nesting level.

File Mapping Options: A module can exist in multiple forms:
  1. Inline: mod vegetables { pub struct Carrot; }
  2. Adjacent file: vegetables.rs
  3. Directory: vegetables/mod.rs (with submodules in vegetables/*.rs)
  The compiler checks these locations in order. Choose based on size
  and complexity - inline for small modules, files for medium, directories
  for modules with submodules.

Module Tree Independence: The module hierarchy is separate from file
  structure. You can define 10 modules in one file, or split one logical
  module across files with re-exports. The module tree defines namespacing;
  files are just storage.

💼 WHERE IT'S MOST USED
• Feature organization: user::profile, user::auth, user::session
• Layer separation: models::user, views::user, controllers::user
• Standard library: std::collections::hash_map::HashMap
• Domain grouping: shop::products::electronics::Laptop
• API versioning: api::v1::endpoints, api::v2::endpoints

Common patterns:
• Organize by feature for small/medium projects (auth, database, api)
• Organize by layer for large projects (presentation, business, data)
• Keep related code together at each nesting level
• Use depth sparingly - 3-4 levels max for readability

✅ TAKEAWAY
Nested modules organize code hierarchically with parent::child::item
paths. Each level must be pub for external access, creating natural
API boundaries at every nesting level. Paths resolve at compile time
with zero runtime cost. Modules are logical constructs - you choose
how to map them to files (inline, .rs file, or directory). Use nesting
to group related functionality, with 2-3 levels being the sweet spot
for most projects. Pick an organization strategy (by feature, by layer,
by domain) and apply it consistently throughout your codebase."#,
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
            commentary: r#"📚 INTRODUCTION
The use keyword brings paths into scope, creating local bindings that
eliminate repetitive fully-qualified names. Write use std::collections::HashMap
once, then use HashMap instead of the full path throughout that scope.
Works at any scope level - module-wide, function-local, even within
a block. These bindings follow normal shadowing rules and can be
renamed with as.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Problems without use:
• Verbosity: std::collections::HashMap::new() repeatedly → visual clutter
• Reduced readability: long paths obscure business logic
• Refactoring friction: changing module paths requires updates everywhere
• Type signatures become unwieldy: Option<std::collections::HashMap<String, Vec<MyLongTypeName>>>

The use keyword solves this:
• Concise names: HashMap instead of std::collections::HashMap
• Clearer code: focus on logic, not navigation
• Single point of change: update one use statement to change path
• Readable signatures: Option<HashMap<String, Vec<MyLongTypeName>>>

Unlike imports in languages where they affect runtime behavior (Python's
import can execute code), Rust's use is purely compile-time name resolution
with zero runtime impact.

🔍 IMPORTANT DETAILS & INTRICACIES
Zero-Cost Abstraction: use is a compile-time convenience. The compiler
  expands each usage to the full path internally, then generates identical
  machine code whether you use imports or not. No lookup tables, no
  runtime resolution, no performance impact.

Scope-Level Flexibility: use works anywhere:
  • Module level: visible to entire module
  • Function level: visible only in that function
  • Block level: use std::fs::File; in an if block
  This lets you minimize scope of imports for clarity.

Idiomatic Conventions (not enforced by compiler):
  • Types (structs, enums, traits): import the type
    use std::collections::HashMap; then HashMap::new()
  • Functions: import parent module
    use std::collections; then collections::hash_map()
  • Methods: always on the type, so import the type
  Reason: seeing the module name at function call sites aids readability.

Visibility vs Import: use doesn't change visibility. A private item
  can be imported via use but remains inaccessible outside its module.
  Only pub use re-exports items as part of public API.

💼 WHERE IT'S MOST USED
• External dependencies: use serde::{Serialize, Deserialize};
• Standard library: use std::fs::File;
• Deep module paths: use crate::database::models::user::User;
• Multiple items: use std::io::{Read, Write, BufReader};
• Type-heavy code: use std::collections::{HashMap, HashSet, BTreeMap};

Common patterns:
• Group imports at file top, organized by external/std/crate
• Alphabetize within groups for consistency (rustfmt does this)
• Use IDE quick-fixes to auto-import (saves manual typing)
• Remove unused imports (compiler warns, clippy errors)

✅ TAKEAWAY
The use keyword creates local name bindings for paths, eliminating
repetitive fully-qualified names with zero runtime cost. Import types
directly (use HashMap), but import parent modules for functions (use
collections, then collections::func). Works at any scope level for
flexibility. Purely a compile-time convenience - doesn't affect visibility,
binary size, or performance. Use liberally to improve readability, and
let tools like rustfmt organize imports automatically. The goal is clear
code where the important logic stands out, not module paths."#,
            difficulty: Difficulty::Intermediate,
        },
        Example {
            name: "use_as",
            description: "Renaming imports with 'as'",
            code: r#"use std::collections::HashMap as Map;

let mut data = Map::new();
data.insert("key", "value");
println!("{:?}", data);"#,
            commentary: r#"📚 INTRODUCTION
The as keyword renames imports within the current scope, creating a local
alias for any item (type, function, module, trait, constant). Write
use std::collections::HashMap as Map to use Map instead of HashMap
in that scope. The original name is not automatically available unless
separately imported. Renamed bindings follow normal scoping rules and
work at any scope level (module, function, block).

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Name conflict problems without renaming:
• Type collision: two crates export Result → can't import both
• Function shadowing: local name conflicts with imported name
• Generic names: importing Error from multiple sources → ambiguous
• Migration pain: can't run old and new APIs side-by-side

The as keyword solves this:
• Import conflicting types: use std::io::Result as IoResult; use crate::Result as AppResult;
• Avoid shadowing: use std::fs::read as read_file; (when you have local read)
• Disambiguate: use db::User as DbUser; use api::User as ApiUser;
• Enable gradual migration: use old_lib::Api as OldApi; use new_lib::Api as NewApi;

Unlike some languages that require complex import mechanics, Rust's
as provides a simple, explicit solution to all naming conflicts.

🔍 IMPORTANT DETAILS & INTRICACIES
Scope Locality: Renaming is local to the scope where the use statement
  appears. Other modules, other files, even other scopes in the same file
  are unaffected. This means renaming is completely safe - you can't
  accidentally break external code by introducing an alias.

Works With Everything: Can rename any item:
  • Types: use HashMap as Map
  • Functions: use calculate_total as total
  • Modules: use std::collections as col
  • Traits: use std::fmt::Display as Show
  • Constants: use MAX_SIZE as SIZE
  • Even re-exports: pub use internal::Api as PublicApi

Original Name Not Available: use HashMap as Map makes only Map available,
  not HashMap. To have both, use two statements: use HashMap; use HashMap as Map;
  This is intentional - forces explicit choice about what names are in scope.

Renaming vs pub use: Combining pub use ... as creates a new public
  name in your API. External code sees the renamed version. This is
  powerful for API design - you can expose third-party types under your
  own names, hiding the underlying dependency.

💼 WHERE IT'S MOST USED
• Resolving conflicts: use tokio::fs::File as TokioFile; use std::fs::File;
• Result types: use std::io::Result as IoResult; (every module defines Result)
• Shortening names: use very_long_crate_name::module as short;
• Migration: use legacy::Database as OldDb; use modern::Database as NewDb;
• API abstraction: pub use external_crate::Type as OurType;

Common patterns:
• Suffix pattern: DbUser, ApiUser, TestUser (shows origin)
• Domain pattern: IoResult, DbResult, ParseResult (shows purpose)
• Shortened modules: col for collections, fs for filesystem
• Avoid generic renames like Map, List unless crystal clear in context

✅ TAKEAWAY
The as keyword renames imports to create local aliases, solving name
conflicts and improving readability. Works with any item (types, functions,
modules, traits). Renamed bindings are local to their scope - safe and
non-invasive. Use it to import conflicting types, shorten verbose names,
or enable gradual migration. Don't overuse - too many aliases confuse
readers. Reserve renaming for genuine conflicts or clarity improvements.
Combine with pub use to create public aliases in your API, hiding
implementation details or third-party dependencies."#,
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
            commentary: r#"📚 INTRODUCTION
Re-exporting with pub use brings an item into the current module's
public API, making it accessible to external code as if defined locally.
Write pub use internal::Widget to expose Widget at the current module's
level, even though it's defined elsewhere. This decouples public API
structure from internal code organization. Users see a clean, flat API
while you organize implementation in deep hierarchies.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
API design problems without re-exporting:
• Leaky abstraction: internal structure exposed → can't refactor
• Deep paths: use mycrate::internal::impl::detail::Widget → tedious
• Breaking changes: moving files breaks external code → fragile
• Discoverability: important types buried deep → hard to find

Re-exporting solves this:
• Stable API: pub use internal::Widget; at root → users use mycrate::Widget
• Refactor freely: move internal modules, maintain re-exports → no breakage
• Flat API surface: hide complexity, expose simplicity
• Explicit design: choose what's public vs implementation detail

Unlike languages where module structure dictates API (Java packages,
Python __init__), Rust separates implementation structure from public
interface through pub use.

🔍 IMPORTANT DETAILS & INTRICACIES
API Independence: Internal organization can change completely without
  breaking users. Move Widget from ui::components to widgets::core,
  just update pub use - external code unchanged. This enables fearless
  refactoring of published libraries.

Visibility Requirements: The re-exported item must be visible from the
  re-exporting location. Can't pub use a private item to make it public
  (would violate privacy). All intermediate modules in the path must be
  pub or at least visible to the re-exporting module.

Prelude Pattern: Many crates provide a prelude module with:
  pub mod prelude {
      pub use crate::{CommonType, UsefulTrait, KeyFunction};
  }
  Users do use mycrate::prelude::*; to get started quickly. Standard
  library's std::prelude is auto-imported in every module.

Semantic Versioning: pub use creates API stability obligations. Adding
  new pub use is minor version bump (backward compatible). Removing pub
  use is major version bump (breaking change). Moving implementation
  while maintaining pub use is patch version (invisible to users).

Re-exporting External Items: Can re-export from dependencies:
  pub use external_crate::Type;
  This makes Type part of your API. If external_crate changes Type, your
  API breaks. Use cautiously - creates tight coupling to dependency versions.

💼 WHERE IT'S MOST USED
• Crate root APIs: pub use internal::Widget; in lib.rs
• Prelude modules: pub use commonly_used::items;
• Facade pattern: pub use impl_detail::concrete as Trait;
• Version compatibility: pub use v2::Api; (while v1 deprecated)
• Framework exports: web frameworks expose Request, Response at root

Common patterns:
• Flat root API for small crates (everything at crate::Item)
• Organized root API for large crates (crate::category::Item)
• Prelude for getting started (crate::prelude::*)
• Re-export traits needed for type functionality
• Hide implementation crates in workspaces

✅ TAKEAWAY
Re-exporting with pub use shapes your public API independently from
internal code organization. Expose items at convenient paths while
organizing implementation in logical hierarchies. This enables refactoring
without breaking external code - move files freely, just maintain re-exports.
Use it to create flat, discoverable APIs and prelude modules for quick
starts. Remember: pub use creates API contracts that follow semantic
versioning - adding is safe, removing is breaking. Design your public
surface intentionally, hide implementation details, and document exports
well since they form your crate's contract with users."#,
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
            commentary: r#"📚 INTRODUCTION
The glob operator (*) imports all public items from a module in one
statement: use std::collections::* brings every public type, function,
trait, and constant into scope. While maximally convenient, glob imports
sacrifice clarity - you can't tell where names originate by looking at
imports. Creates name collision risks when multiple globs export items
with identical names (compile error).

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Problems that tempt glob usage:
• Verbosity: importing 20 items → 20 use statements or one long brace list
• Test boilerplate: tests need most parent module items → repetitive imports
• Prelude modules: carefully curated common items → meant for glob import
• Example code: focusing on concepts, not import management → cleaner demos

Glob provides convenience but at costs:
• Origin obscured: see HashMap, can't tell it's from collections
• Name collisions: two globs with Result → compile error, hard to diagnose
• IDE confusion: autocomplete degraded, can't statically resolve what's available
• Review friction: reviewers can't see dependencies without checking module

Unlike some languages (Python's from x import * is discouraged but common),
Rust community strongly prefers explicit imports except in narrow contexts.

🔍 IMPORTANT DETAILS & INTRICACIES
Compile-Time Resolution: Glob imports are fully resolved at compile time.
  The compiler knows exactly what's imported and generates identical code
  to explicit imports. No runtime penalty, no lookup overhead. Still,
  they obscure the code for humans.

Unused Import Warnings: The compiler tracks glob-imported items and warns
  if nothing from the glob is used. However, it can't warn about individual
  unused items within the glob - it's all-or-nothing. This makes it harder
  to clean up dependencies.

Name Collision Behavior: If two globs import items with the same name,
  using that name is a compile error. Must explicitly import one of them:
  use std::io::Error;  // disambiguates from another glob's Error
  Or qualify it: std::io::Error::new()

Visibility Applies: Only public items are imported by globs. Private
  items remain inaccessible. Within a module, use super::* imports
  private parent items (child can access parent privates).

Prelude Special Case: Rust's std::prelude::v1 is glob-imported automatically
  in every module. This is why Option, Result, String, Vec, etc. are
  available without imports. This is the canonical "good" use of globs
  because the prelude is carefully curated for stability and minimal
  collision risk.

💼 WHERE IT'S MOST USED
• Test modules: use super::* to import everything being tested
• Prelude modules: use mycrate::prelude::* for getting started
• Internal preludes: use crate::common::* for widely-used internal items
• Derive macros: use serde::*; in quick examples
• Documentation: examples focusing on other concepts

Contexts to AVOID:
• Production library code (obscures dependencies)
• Public API modules (unclear what's exposed)
• Multiple globs in same scope (collision risk)
• Large modules with many items (brings in too much)

✅ TAKEAWAY
Glob imports (use module::*) bring all public items into scope for
maximum convenience but minimum clarity. Appropriate for test modules,
prelude modules, and focused examples. Avoid in production code where
explicit imports aid readability, navigation, and code review. Glob
imports have zero runtime cost but real human costs - readers can't
identify where names originate without checking imported modules. The
Rust community strongly prefers explicit imports except in narrow,
well-justified contexts. When you need many items from a module, consider
why - might indicate design issues or opportunities to reorganize code."#,
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
            commentary: r#"📚 INTRODUCTION
Rust's privacy system is private-by-default - every item (function, struct,
enum, constant, trait, module) is private to its parent module unless
marked pub. This forces intentional API design - you must consciously
decide what to expose. Privacy is checked at compile time at module
boundaries. Items within a module can access all other items in that
module, regardless of pub.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Public-by-default problems (C, Go, JavaScript):
• Accidental dependencies: external code uses anything → can't refactor
• No encapsulation: implementation details exposed → fragile APIs
• Breaking changes: any change potentially breaks users → fear of evolution
• Unclear intent: is this API or implementation? → confusion

Private-by-default solves this:
• Explicit APIs: pub marks intentional public interface
• Safe refactoring: private items changeable freely → fearless improvement
• Clear boundaries: public is contract, private is implementation
• Encapsulation enforced: compiler prevents accessing internals

Unlike languages with runtime access control or convention-based privacy
(Python's _prefix), Rust enforces privacy at compile time with zero
runtime overhead.

🔍 IMPORTANT DETAILS & INTRICACIES
Zero-Cost Abstraction: Privacy is purely compile-time. The compiler
  checks privacy rules, then generates code. No runtime access checks,
  no performance penalty. Private and public functions compile to
  identical machine code structure.

Module Boundary Scope: Privacy operates at module boundaries:
  • Within module: all items accessible to each other (even private)
  • Parent to child: parent can access child's private items
  • Child to parent: child can access parent's private items
  • Sibling to sibling: can only access each other's pub items
  This creates hierarchical encapsulation.

Privacy Inheritance: Marking a module pub doesn't make its contents pub.
  Must mark each item individually: pub mod foo { pub fn bar() {} }
  If foo is pub but bar isn't, external code can see foo exists but can't
  call bar. Allows exposing module structure while hiding implementation.

Re-export Privacy: Can pub use a private item to make it public at
  the re-export location. This doesn't change the item's original privacy,
  just provides a public path to it. Enables API design flexibility.

Test Module Exception: #[cfg(test)] modules can access private items
  in their parent module. This is why you can test private functions
  without making them pub. The test configuration makes them visible
  during testing only.

💼 WHERE IT'S MOST USED
• API design: pub fn public_api() + fn helper() keeps helpers hidden
• Library maintenance: change private items freely between versions
• Encapsulation: pub struct with private fields + pub methods
• Module organization: pub mod api, private mod implementation
• Test support: private items testable via #[cfg(test)] mod tests

Common patterns:
• Small pub surface: 5-10 public items, 50+ private items
• Builder pattern: pub builder methods, private construction logic
• Internal modules: pub types, private implementation modules
• Facade pattern: pub interface delegates to private implementations

✅ TAKEAWAY
Rust defaults everything to private, requiring explicit pub for public
API. This enforces intentional design - you choose what to expose.
Privacy is checked at compile time with zero runtime cost. All code
within a module accesses everything; only external code needs pub.
This enables fearless refactoring - change private items freely without
breaking users. Keep most things private, exposing only necessary API
surface. For libraries, smaller public APIs are easier to maintain and
evolve. Think of pub as a promise to users - once public, changes must
follow semantic versioning. Private items are yours to improve freely."#,
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
            commentary: r#"📚 INTRODUCTION
The super keyword creates relative paths referencing the parent module,
like .. in filesystem paths. Write super::parent_function() to access
items one level up the module tree. More maintainable than absolute
paths for closely related modules - when you move the entire hierarchy,
super paths continue working. Can chain (super::super::) to go up
multiple levels.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Absolute path problems in nested modules:
• Fragile: moving module hierarchy breaks crate::old::path references
• Verbose: crate::very::deep::nesting::item repeated everywhere
• Unclear intent: using absolute path for parent → obscures relationship
• Refactoring friction: reorganizing modules requires updating many paths

Relative paths with super solve this:
• Refactor-friendly: move entire module tree, relative paths still work
• Express relationship: super says "my parent" (structural, not logical)
• Concise for nearby code: super::sibling vs crate::module::sibling
• Test pattern: super::function_to_test from #[cfg(test)] mod tests

Unlike languages where relative imports are error-prone (Python's relative
imports have many gotchas), Rust's super is simple and unambiguous - always
means parent module.

🔍 IMPORTANT DETAILS & INTRICACIES
Structural vs Logical Paths: Two philosophies:
  • super::item = "my parent's item" (structural relationship)
  • crate::module::item = "this specific item" (logical dependency)
  Use super for tightly coupled code, crate:: for distinct modules.
  Both compile to identical code, difference is expressiveness.

Privacy Interaction: Child modules can access parent's private items.
  super::private_function() works from child, even if private_function
  isn't pub. This is fundamental to Rust's privacy model - hierarchy
  determines access, not pub keywords within family.

Chaining Allowed: super::super::grandparent_item goes up two levels.
  super::super::super goes up three. Excessive chaining (more than 2)
  often indicates overly deep nesting or poor module organization.
  Consider flattening the hierarchy.

Works Everywhere: super works in use statements, function calls, type
  paths, anywhere a path is valid:
  • use super::Parent;
  • let x = super::create();
  • impl super::Trait for Type {}

Test Module Pattern: The canonical super use case:
  mod tests {
      use super::*;  // import everything from parent
      #[test]
      fn test_private_function() {
          super::private_helper();  // can test private items
      }
  }

💼 WHERE IT'S MOST USED
• Test modules: use super::* to import parent's items for testing
• Sibling access: super::sibling_module::function from child
• Helper utilities: super::common_helper() from multiple submodules
• Type references: impl super::Trait for LocalType
• Constructor patterns: super::Builder::new() from submodule

Common patterns:
• Tests in same file: mod tests { use super::*; }
• Submodule collaboration: super::sibling when siblings need to interact
• One level up: super::item (common), super::super (rare), more is code smell
• Mix with absolute: use super for nearby, crate:: for distant

✅ TAKEAWAY
The super keyword creates relative paths to parent modules, expressing
structural relationships (this is my parent) rather than logical dependencies
(I need that specific module). Particularly useful in test modules for
accessing private items from the parent. Chain super::super:: to go
up multiple levels, though excessive chaining suggests poor organization.
Choose super for closely related modules (especially in same file) and
crate:: for distant modules where clarity about the exact dependency
is valuable. Both compile identically; difference is intent and maintainability.
Relative paths survive refactoring better than absolute paths within
tightly coupled module families."#,
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
            commentary: r#"📚 INTRODUCTION
Struct field visibility is independent of struct visibility. pub struct
Rectangle makes the type name public, but fields remain private by default.
Mark individual fields pub selectively: pub width but private height.
This fine-grained control enables encapsulation - expose safe parts
directly, hide implementation details, and maintain invariants through
controlled access methods.

🎯 WHY IT EXISTS & PROBLEM IT SOLVES
Public fields by default problems (C, Go):
• Broken invariants: Rectangle.width = -10 allowed → invalid state
• Refactoring locked: changing field type/name breaks external code
• No validation: direct field access bypasses checks
• Implementation exposed: users depend on internal representation

Private-by-default fields solve this:
• Controlled access: methods validate (ensure height > 0)
• Free refactoring: change private field representation without breakage
• Invariant maintenance: constructor establishes, methods preserve guarantees
• API evolution: start with direct access, add logic later without breaking

Unlike languages with getter/setter conventions (Java beans), Rust
enforces encapsulation at compile time. Can't access private fields
even via reflection.

🔍 IMPORTANT DETAILS & INTRICACIES
Independence from Type Visibility: pub struct doesn't imply pub fields.
  Three visibility levels combine:
  1. Type visibility: pub struct (external code can name it)
  2. Field visibility: pub width (external code can access it)
  3. Module visibility: determines who "external" is
  This three-way combination enables precise API control.

Construction Implications: Struct literal syntax requires all fields accessible:
  • All pub fields: Rectangle { width: 10, height: 20 } works externally
  • Any private field: constructor required (Rectangle::new(10, 20))
  Private fields force constructor functions, centralizing initialization
  and allowing invariant establishment.

Pattern Matching: Can't destructure private fields externally:
  let Rectangle { width, height } = rect;  // error if height private
  Must use accessors: let h = rect.height(); or destructure with ..:
  let Rectangle { width, .. } = rect;  // ignores private fields

Builder Pattern Enabler: Private fields make builder pattern natural:
  Rectangle::builder()
      .width(10)
      .height(20)
      .build()
  Each setter can validate. build() ensures all required fields set.

Selective Publicity: Common patterns:
  • pub data, private cache: expose data, hide optimization
  • pub id, private mutable state: expose identifier, control changes
  • all private + pub methods: complete encapsulation (most robust)

💼 WHERE IT'S MOST USED
• Invariant enforcement: pub struct Point { pub x: f64, pub y: f64 } is fine,
  but pub struct Temperature { celsius: f64 } keeps invariants safe
• Builder pattern: all fields private, builder methods pub
• Computed properties: private storage, pub methods return computed values
• Representation hiding: private Vec<T>, pub methods expose &[T]
• Migration safety: keep fields private during API evolution

Common patterns:
• Simple data: all pub fields (Point, Color) when no invariants
• Domain objects: private fields + pub methods (User, Account)
• Builders: private fields + pub setters + build()
• Read-only exposure: private field + pub getter, no setter

✅ TAKEAWAY
Struct field visibility is independent from struct visibility. Fields
default to private even in pub structs. Mark fields pub selectively
to expose safe parts while hiding implementation details. Private fields
require constructors (Rectangle::new) instead of literals (Rectangle { }),
centralizing initialization and enabling validation. This fine-grained
control enables proper encapsulation - maintain invariants, refactor
freely, evolve APIs safely. Common pattern: keep all fields private,
provide pub constructors and methods. Only make fields pub when they're
truly part of your public contract and have no invariants to maintain.
Privacy enables fearless refactoring of internal representation."#,
            difficulty: Difficulty::Advanced,
        },
    ]
}
