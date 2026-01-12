# Quick Start Guide

## Installation

```bash
cd rust_traits
cargo install --path .
```

The `howtorust` binary will be installed to `~/.cargo/bin/` (make sure this is in your PATH).

## Basic Commands

### See all available chapters
```bash
howtorust --list
```

### Interactive learning mode
```bash
howtorust ownership
```
Then:
- Type a number (e.g., `1`, `2`) to view and run that example
- Type `list` to see all examples again
- Type `quit` to exit

### Run a specific example directly
```bash
howtorust traits --example basic_trait
howtorust closures --example move_keyword
howtorust iterators --example lazy_evaluation
```

### Get help
```bash
howtorust --help
```

## Example Session

```bash
# 1. List all topics
$ howtorust --list

Available Chapters:
1. ownership - Understanding Ownership
2. modules - Packages, Crates, and Modules
3. generics - Generic Types
...

# 2. Learn about traits
$ howtorust traits

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
$ howtorust ownership --example borrowing_mutable

# Shows the code with syntax highlighting and output
```

## Learning Path

Recommended order:
1. `howtorust ownership` - Foundation of Rust
2. `howtorust modules` - Code organization
3. `howtorust errors` - Essential for real programs
4. `howtorust traits` - Core abstraction
5. `howtorust generics` - Flexible code
6. `howtorust lifetimes` - Advanced ownership
7. `howtorust closures` - Functional programming
8. `howtorust iterators` - Efficient data processing

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
howtorust ownership --example basic_ownership

# Module organization
howtorust modules --example nested_modules

# Generic functions
howtorust generics --example generic_function

# Trait objects
howtorust traits --example trait_objects

# Lifetime annotations
howtorust lifetimes --example struct_lifetimes

# Error handling with ?
howtorust errors --example question_mark_operator

# Closure capturing
howtorust closures --example capturing_environment

# Iterator chains
howtorust iterators --example chaining_iterators
```

Enjoy learning Rust! 🦀
