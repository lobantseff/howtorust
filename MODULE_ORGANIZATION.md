# Module Organization Guide

This document explains how Rust modules are organized in this project, demonstrating both single-file and subdirectory approaches.

## Project Module Structure

```
src/
├── lib.rs                          # Crate root - declares all modules
├── main.rs                         # Binary entry point
│
├── closures/                       # 📁 Directory-based module
│   ├── mod.rs                      # Module root
│   ├── examples.rs                 # Example definitions
│   └── runners.rs                  # Example execution
│
├── ownership.rs                    # 📄 Single-file module
├── packages_crates_modules.rs      # 📄 Single-file module
├── generics.rs                     # 📄 Single-file module
├── traits.rs                       # 📄 Single-file module
├── lifetimes.rs                    # 📄 Single-file module
├── error_handling.rs               # 📄 Single-file module
└── iterators.rs                    # 📄 Single-file module
```

## Two Ways to Organize Modules

### 1. Single-File Module (Simple)

For smaller modules, use a single `.rs` file:

**File:** `src/ownership.rs`
```rust
// Module content goes directly in the file
use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    // ...
}

pub fn run_example(name: &str) {
    // ...
}
```

**Declaration in lib.rs:**
```rust
pub mod ownership;
```

**Usage:**
```rust
use howrust::ownership;
ownership::run_example("basic_ownership");
```

### 2. Directory-Based Module (Organized)

For larger modules with multiple concerns, use a directory:

**Structure:**
```
src/closures/
├── mod.rs       # Public interface
├── examples.rs  # Example data
└── runners.rs   # Example execution
```

**File:** `src/closures/mod.rs`
```rust
// Private submodules
mod examples;
mod runners;

// Re-export public API
pub use examples::get_examples;
pub use runners::run_example;
```

**File:** `src/closures/examples.rs`
```rust
use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        Example {
            name: "basic_closure",
            // ...
        },
    ]
}
```

**File:** `src/closures/runners.rs`
```rust
pub fn run_example(name: &str) {
    match name {
        "basic_closure" => basic_closure(),
        _ => println!("Not found"),
    }
}

fn basic_closure() {
    // Implementation
}
```

**Declaration in lib.rs:**
```rust
pub mod closures;  // Rust looks for closures/mod.rs
```

**Usage:**
```rust
use howrust::closures;
closures::run_example("basic_closure");  // Same API as single-file!
```

## Why Use Directories?

### Advantages of Directory-Based Modules:

1. **Separation of Concerns**
   - `examples.rs` - Data definitions
   - `runners.rs` - Execution logic
   - `mod.rs` - Public interface

2. **Easier Navigation**
   - Related code grouped together
   - Clear file boundaries
   - Better for large modules

3. **Team Collaboration**
   - Multiple people can work on different files
   - Fewer merge conflicts
   - Clear ownership of functionality

4. **Future Extensibility**
   - Easy to add more files (e.g., `helpers.rs`, `tests.rs`)
   - Can split further as needed
   - Maintains same public API

## Migration Pattern

To convert a single-file module to a directory:

### Before:
```
src/
└── closures.rs
```

### After:
```
src/
└── closures/
    ├── mod.rs        # Move public API here
    ├── examples.rs   # Extract example data
    └── runners.rs    # Extract implementation
```

### Steps:

1. **Create directory:**
   ```bash
   mkdir src/closures
   ```

2. **Create `mod.rs`:**
   ```rust
   mod examples;
   mod runners;

   pub use examples::get_examples;
   pub use runners::run_example;
   ```

3. **Move code to submodules:**
   - Example data → `examples.rs`
   - Implementation → `runners.rs`

4. **Remove old file:**
   ```bash
   rm src/closures.rs
   ```

5. **No changes needed in `lib.rs`!**
   ```rust
   pub mod closures;  // Still works!
   ```

## Real Example: Closures Module

### Directory Structure
```
src/closures/
├── mod.rs          # 15 lines - just re-exports
├── examples.rs     # 150+ lines - all Example definitions
└── runners.rs      # 150+ lines - all execution functions
```

### mod.rs (Public Interface)
```rust
// Closures: Anonymous functions that capture their environment
//
// This module demonstrates Rust's subdirectory organization:
// - mod.rs - Module root, re-exports public API
// - examples.rs - Example definitions
// - runners.rs - Example execution functions

mod examples;
mod runners;

pub use examples::get_examples;
pub use runners::run_example;
```

**Key Points:**
- `mod examples;` - Private submodule
- `mod runners;` - Private submodule
- `pub use` - Re-export for clean API
- Users don't know about internal structure!

### examples.rs (Data Layer)
```rust
use crate::{Difficulty, Example};

pub fn get_examples() -> Vec<Example> {
    vec![
        Example {
            name: "basic_closure",
            description: "Creating and calling a simple closure",
            code: r#"..."#,
            difficulty: Difficulty::Beginner,
        },
        // 8 more examples...
    ]
}
```

### runners.rs (Execution Layer)
```rust
pub fn run_example(name: &str) {
    match name {
        "basic_closure" => basic_closure(),
        "move_keyword" => move_keyword(),
        // ...
        _ => println!("Example '{}' not found", name),
    }
}

fn basic_closure() {
    let add = |a, b| a + b;
    println!("{}", add(5, 3));
}

// More runner functions...
```

## Best Practices

### When to Use Single File:
- ✅ Module < 200 lines
- ✅ Single clear purpose
- ✅ No natural subdivisions
- ✅ Rarely modified

### When to Use Directory:
- ✅ Module > 200 lines
- ✅ Multiple concerns
- ✅ Natural subdivisions exist
- ✅ Frequently modified
- ✅ Multiple contributors

## Module Privacy

Both approaches support the same privacy controls:

```rust
// In closures/examples.rs
pub fn get_examples() -> Vec<Example> { }    // Public
fn helper_function() { }                     // Private to examples.rs

// In closures/runners.rs
pub fn run_example(name: &str) { }           // Public
fn basic_closure() { }                       // Private to runners.rs
```

**Important:** Items in `examples.rs` and `runners.rs` are private to the `closures` module unless re-exported in `mod.rs`.

## Testing with Subdirectories

Directory modules can have inline tests:

```rust
// In closures/runners.rs
pub fn run_example(name: &str) { /* ... */ }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner() {
        run_example("basic_closure");
    }
}
```

Or a dedicated test file:

```
src/closures/
├── mod.rs
├── examples.rs
├── runners.rs
└── tests.rs        // Private test module
```

```rust
// In closures/mod.rs
mod examples;
mod runners;

#[cfg(test)]
mod tests;

pub use examples::get_examples;
pub use runners::run_example;
```

## Summary

This project demonstrates both module organization styles:

| Module | Style | Reason |
|--------|-------|--------|
| **closures** | 📁 Directory | Demonstrates subdirectory organization |
| ownership | 📄 Single file | Simpler for this size |
| generics | 📄 Single file | Simpler for this size |
| traits | 📄 Single file | Simpler for this size |
| lifetimes | 📄 Single file | Simpler for this size |
| error_handling | 📄 Single file | Simpler for this size |
| iterators | 📄 Single file | Simpler for this size |
| packages_crates_modules | 📄 Single file | Ironically! |

**Key Takeaway:** The `closures` module shows how to organize larger modules with subdirectories while maintaining the same clean API as single-file modules. Both approaches work seamlessly with Rust's module system.
