# HowRust - Interactive Rust Tutorial & Cheatsheet

An interactive command-line tool for learning Rust concepts through executable examples. Each chapter covers a fundamental Rust topic with examples progressing from beginner to advanced levels.

## Features

- **8 Comprehensive Chapters** covering core Rust concepts
- **60+ Executable Examples** organized by difficulty
- **Interactive CLI** with syntax-highlighted code display
- **Module-based Architecture** demonstrating Rust's code organization
- **Executable Examples** - see the code and run it instantly

## Installation

### From Source

```bash
# Clone or navigate to the project directory
cd rust_traits

# Install the binary
cargo install --path .
```

This will install the `howrust` binary to your Cargo bin directory (usually `~/.cargo/bin/`), which should already be in your PATH.

### Verify Installation

```bash
howrust --help
```

## Usage

### List All Available Chapters

```bash
howrust --list
```

This displays all 8 chapters:
- **ownership** - Understanding Rust's ownership system
- **modules** - Packages, Crates, and Modules
- **generics** - Generic Types
- **traits** - Defining shared behavior
- **lifetimes** - Validating references
- **errors** - Error Handling with Result and Option
- **closures** - Anonymous functions
- **iterators** - Processing sequences of values

### View a Chapter

```bash
howrust ownership
```

This will:
1. Display all examples in the chapter grouped by difficulty
2. Enter an interactive menu where you can:
   - Type a number to view and run that example
   - Type `list` to see all examples again
   - Type `quit` to exit

### Run a Specific Example

```bash
howrust traits --example basic_trait
```

This directly displays and executes a specific example without the interactive menu.

### Example Workflow

```bash
# 1. See what's available
howrust --list

# 2. Explore ownership chapter
howrust ownership

# 3. In the interactive menu, type a number (e.g., "1") to run an example
# 4. Type "list" to see all examples
# 5. Type "quit" to exit

# Or run a specific example directly
howrust closures --example move_keyword
```

## Chapter Overview

### 1. Understanding Ownership
Learn about Rust's ownership system, move semantics, borrowing, and references.

**Examples**: basic_ownership, clone_vs_move, function_ownership, borrowing_immutable, borrowing_mutable, multiple_references, reference_rules, slice_internals, dangling_reference_prevention

### 2. Packages, Crates, and Modules
Organize code with modules, understand visibility, and use the module system.

**Examples**: basic_module, nested_modules, use_keyword, use_as, pub_use, glob_imports, module_visibility, super_keyword, struct_privacy

### 3. Generic Types
Write flexible, reusable code with generic type parameters.

**Examples**: generic_function, generic_struct, multiple_generic_params, generic_methods, generic_enums, trait_bounds, where_clause, generic_associated_types, const_generics

### 4. Traits
Define shared behavior across types with trait definitions and implementations.

**Examples**: basic_trait, default_implementation, trait_bounds, multiple_trait_bounds, trait_objects, return_trait, derive_traits, associated_types, operator_overloading, supertraits

### 5. Lifetimes
Understand and use lifetime annotations to validate references.

**Examples**: basic_lifetime, lifetime_elision, struct_lifetimes, multiple_lifetimes, lifetime_bounds, method_lifetimes, static_lifetime, lifetime_subtyping, higher_ranked_trait_bounds

### 6. Error Handling
Handle errors gracefully with Result, Option, and custom error types.

**Examples**: basic_result, basic_option, unwrap_and_expect, question_mark_operator, option_combinators, result_combinators, custom_error_types, error_conversion, multiple_error_types, early_return_pattern

### 7. Closures
Use anonymous functions that capture their environment with different capture modes.

**Examples**: basic_closure, closure_type_inference, capturing_environment, fn_traits, move_keyword, closures_as_parameters, returning_closures, closure_caching, closure_composition

### 8. Iterators
Process sequences efficiently with iterator adapters and consumers.

**Examples**: basic_iteration, iterator_methods, consuming_adapters, chaining_iterators, fold_reduce, enumerate_zip, find_any_all, custom_iterator, lazy_evaluation, flat_map_flatten

## Project Structure

```
rust_traits/
├── Cargo.toml              # Project configuration
├── README.md               # This file
└── src/
    ├── lib.rs              # Public API and module declarations
    ├── main.rs             # CLI interface
    ├── ownership.rs        # Ownership examples
    ├── packages_crates_modules.rs  # Module system examples
    ├── generics.rs         # Generic types examples
    ├── traits.rs           # Traits examples
    ├── lifetimes.rs        # Lifetimes examples
    ├── error_handling.rs   # Error handling examples
    ├── closures.rs         # Closures examples
    └── iterators.rs        # Iterators examples
```

Each module (`*.rs`) contains:
- Example definitions with code, description, and difficulty level
- Executable functions that demonstrate each concept
- A `run_example()` function to execute specific examples

## Development

### Building

```bash
cargo build --release
```

### Running Without Installing

```bash
cargo run -- ownership
cargo run -- --list
cargo run -- traits --example basic_trait
```

### Running Tests

```bash
cargo test
```

## Shell Completion

To enable shell completion for the `howrust` command, you can set up tab completion for your shell.

### Bash

Add to your `~/.bashrc` or `~/.bash_profile`:

```bash
_howrust_completion() {
    local cur prev chapters
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    chapters="ownership modules generics traits lifetimes errors closures iterators"

    case "${prev}" in
        howrust)
            COMPREPLY=( $(compgen -W "--list --help ${chapters}" -- ${cur}) )
            return 0
            ;;
        --example)
            # Could be enhanced to suggest example names per chapter
            return 0
            ;;
        *)
            if [[ ${cur} == -* ]] ; then
                COMPREPLY=( $(compgen -W "--example --list --help" -- ${cur}) )
            else
                COMPREPLY=( $(compgen -W "${chapters}" -- ${cur}) )
            fi
            return 0
            ;;
    esac
}

complete -F _howrust_completion howrust
```

Then reload your shell:
```bash
source ~/.bashrc
```

### Zsh

Add to your `~/.zshrc`:

```zsh
_howrust() {
    local -a chapters
    chapters=(
        'ownership:Understanding Ownership'
        'modules:Packages, Crates, and Modules'
        'generics:Generic Types'
        'traits:Traits'
        'lifetimes:Lifetimes'
        'errors:Error Handling'
        'closures:Closures'
        'iterators:Iterators'
    )

    _arguments \
        '1: :->chapter' \
        '--list[List all chapters]' \
        '--help[Show help message]' \
        '--example[Run specific example]:example name:'

    case $state in
        chapter)
            _describe 'chapter' chapters
            ;;
    esac
}

compdef _howrust howrust
```

Then reload your shell:
```zsh
source ~/.zshrc
```

### Fish

Create `~/.config/fish/completions/howrust.fish`:

```fish
# Chapter completions
complete -c howrust -n "__fish_is_first_token" -a "ownership" -d "Understanding Ownership"
complete -c howrust -n "__fish_is_first_token" -a "modules" -d "Packages, Crates, and Modules"
complete -c howrust -n "__fish_is_first_token" -a "generics" -d "Generic Types"
complete -c howrust -n "__fish_is_first_token" -a "traits" -d "Traits"
complete -c howrust -n "__fish_is_first_token" -a "lifetimes" -d "Lifetimes"
complete -c howrust -n "__fish_is_first_token" -a "errors" -d "Error Handling"
complete -c howrust -n "__fish_is_first_token" -a "closures" -d "Closures"
complete -c howrust -n "__fish_is_first_token" -a "iterators" -d "Iterators"

# Flag completions
complete -c howrust -n "__fish_is_first_token" -l list -s l -d "List all chapters"
complete -c howrust -n "__fish_is_first_token" -l help -s h -d "Show help message"
complete -c howrust -l example -d "Run specific example"
```

Fish will automatically load this on next shell start.

### Testing Completion

After setting up completion, try:

```bash
howrust <TAB>        # Should show chapter names and flags
howrust own<TAB>     # Should complete to "ownership"
howrust ownership --<TAB>  # Should show --example flag
```

## Dependencies

- **colored** (2.1) - Terminal color output for syntax highlighting and UI

## Learning Path

Recommended order for learning:

1. **ownership** - Start here! Understanding ownership is fundamental
2. **modules** - Learn how to organize code
3. **errors** - Essential for writing robust Rust programs
4. **traits** - Core abstraction mechanism
5. **generics** - Write flexible, reusable code
6. **lifetimes** - Advanced ownership concepts
7. **closures** - Functional programming in Rust
8. **iterators** - Efficient data processing

## Tips

- Start with beginner examples in each chapter
- Run examples to see actual output
- Read the code carefully - it's designed to be instructive
- Experiment by modifying examples in the source code
- Use `--list` frequently to navigate between chapters

## License

This project is designed for educational purposes.

## Contributing

Feel free to:
- Add more examples to existing chapters
- Improve example descriptions
- Enhance syntax highlighting
- Add new chapters for other Rust concepts
- Fix bugs or improve documentation

## Author

An educational project for learning and teaching Rust concepts interactively.
