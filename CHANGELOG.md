# Changelog

## [Updated] - 2026-01-11

### Fixed
1. **Continuous Example Numbering**
   - Fixed interactive menu to show continuous numbering across all difficulty levels
   - Before: Beginner (1-2), Intermediate (1-5), Advanced (1-2)
   - After: Beginner (1-2), Intermediate (3-7), Advanced (8-9)
   - Now matches the numbering shown in the "list" command

2. **Type Annotation in error_handling.rs**
   - Fixed Rust compiler error in `result_combinators()` function
   - Added explicit type annotation for Result type in `.or()` chain

### Added
1. **Subdirectory Module Organization**
   - Restructured `closures` module to use subdirectory organization
   - Created `src/closures/` directory with:
     - `mod.rs` - Public interface
     - `examples.rs` - Example definitions
     - `runners.rs` - Example execution functions
   - Demonstrates both single-file and directory-based module patterns

2. **Documentation**
   - Added `MODULE_ORGANIZATION.md` - Comprehensive guide to module organization
   - Explains when to use single-file vs directory-based modules
   - Provides migration patterns and best practices
   - Updated README.md to reference new module structure

## [Initial Release] - 2026-01-11

### Features
- 8 comprehensive chapters covering core Rust concepts
- 75 executable examples organized by difficulty
- Interactive CLI with syntax-highlighted code display
- Single binary installation via `cargo install`
- Support for three usage modes:
  - `howtorust --list` - Browse chapters
  - `howtorust <chapter>` - Interactive menu
  - `howtorust <chapter> --example <name>` - Direct execution

### Chapters
1. Understanding Ownership (9 examples)
2. Packages, Crates, and Modules (9 examples)
3. Generic Types (9 examples)
4. Traits (10 examples)
5. Lifetimes (9 examples)
6. Error Handling (10 examples)
7. Closures (9 examples)
8. Iterators (10 examples)

### Documentation
- README.md with installation and usage
- QUICKSTART.md for quick reference
- SUMMARY.md with technical details
- Shell completion scripts for bash, zsh, fish
