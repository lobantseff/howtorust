# Quick Start Guide

## Installation

```bash
cd rust_traits
cargo install --path .
```

The `howrust` binary will be installed to `~/.cargo/bin/` (make sure this is in your PATH).

## Basic Commands

### See all available chapters
```bash
howrust --list
```

### Interactive learning mode
```bash
howrust ownership
```
Then:
- Type a number (e.g., `1`, `2`) to view and run that example
- Type `list` to see all examples again
- Type `quit` to exit

### Run a specific example directly
```bash
howrust traits --example basic_trait
howrust closures --example move_keyword
howrust iterators --example lazy_evaluation
```

### Get help
```bash
howrust --help
```

## Example Session

```bash
# 1. List all topics
$ howrust --list

Available Chapters:
1. ownership - Understanding Ownership
2. modules - Packages, Crates, and Modules
3. generics - Generic Types
...

# 2. Learn about traits
$ howrust traits

Traits
======
Define shared behavior with traits

Beginner (2 examples)
  1. basic_trait - Defining and implementing a basic trait
  2. default_implementation - Traits with default method implementations
...

Choose: 1

# Shows code and runs it

# 3. Run specific example
$ howrust ownership --example borrowing_mutable

# Shows the code with syntax highlighting and output
```

## Learning Path

Recommended order:
1. `howrust ownership` - Foundation of Rust
2. `howrust modules` - Code organization
3. `howrust errors` - Essential for real programs
4. `howrust traits` - Core abstraction
5. `howrust generics` - Flexible code
6. `howrust lifetimes` - Advanced ownership
7. `howrust closures` - Functional programming
8. `howrust iterators` - Efficient data processing

## Tips

- Start with **Beginner** examples in each chapter
- Read the code carefully before running
- Try to understand why the output matches the code
- Use `--example` to quickly reference specific patterns
- The colored output helps distinguish different parts

## All Chapters

- **ownership** - Move semantics, borrowing, references
- **modules** - Organizing code with modules
- **generics** - Generic types and bounds
- **traits** - Shared behavior and abstractions
- **lifetimes** - Reference validation
- **errors** - Result, Option, error handling
- **closures** - Anonymous functions
- **iterators** - Processing sequences

## Example Commands

```bash
# Ownership basics
howrust ownership --example basic_ownership

# Module organization
howrust modules --example nested_modules

# Generic functions
howrust generics --example generic_function

# Trait objects
howrust traits --example trait_objects

# Lifetime annotations
howrust lifetimes --example struct_lifetimes

# Error handling with ?
howrust errors --example question_mark_operator

# Closure capturing
howrust closures --example capturing_environment

# Iterator chains
howrust iterators --example chaining_iterators
```

Enjoy learning Rust! 🦀
