# AGENTS.md - Excaligrep Development Guide

This document provides guidelines for AI agents working on the excaligrep codebase.

## Project Overview

Excaligrep is a Rust desktop application that provides a GUI for searching Excel files (.xlsx, .xls). It uses:
- **Iced** - Cross-platform GUI framework
- **Calamine** - Excel file reading
- **RFD** - Native file dialogs
- **Tokio** - Async runtime

## Build & Development Commands

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the application
cargo run

# Run Clippy linter
cargo clippy

# Format code
cargo fmt

# Format check (without modifying)
cargo fmt -- --check

# Run all tests (none currently exist)
cargo test

# Run a single test
cargo test <test_name>

# Check for documentation
cargo doc --no-deps

# View dependencies
cargo tree
```

## Code Style Guidelines

### General Principles

- Follow standard Rust idioms and conventions
- Keep functions small and focused
- Use meaningful variable names
- No comments unless explaining complex logic

### Naming Conventions

- **Structs/Enums**: PascalCase (e.g., `Excaligrep`, `SearchResult`)
- **Functions/Variables**: snake_case (e.g., `search_query`, `ensure_indexed`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Modules**: snake_case

### Imports

- Group imports by crate: standard library first, then external crates, then local modules
- Use `use` statements at the module top, not inline
- Prefer absolute paths from crate root for `crate::` imports

Example:
```rust
use std::path::PathBuf;
use calamine::{Data as CellData, Reader, open_workbook_auto};

use crate::app::{Config, SearchResult};
```

### Error Handling

- Use `Result` types for functions that can fail
- Use `?` operator for propagating errors
- Use `unwrap_or_default()` or similar patterns when default is safe
- For operations where failure is acceptable, use `let _ =` to ignore results

### Pattern Conventions Observed

The codebase uses these patterns (maintain consistency):

```rust
// Chained if-let for early returns
if let Some(path) = folder {
    // ...
}

// Iterating with flatten chains
std::fs::read_dir(folder)
    .into_iter()
    .flatten()
    .flatten()
    .filter_map(|entry| { ... })

// Conditional with let-else
if should_write && let Ok(range) = workbook.worksheet_range(sheet_name) {
    // ...
}
```

### Struct Definitions

```rust
#[derive(Default, Clone)]
pub struct Excaligrep {
    pub search_query: String,
    pub selected_folder: Option<PathBuf>,
    pub search_results: Vec<SearchResult>,
    pub files: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub file_name: String,
    pub line: String,
}
```

- Use `#[derive(...)]` for common traits: Default, Clone, Debug, PartialEq
- Keep fields public for simple data structs
- Add doc comments (`///`) for public API documentation

### Iced GUI Patterns

```rust
// Message enum for all UI events
#[derive(Debug, Clone)]
enum Message {
    SearchInputChanged(String),
    SearchPressed,
}

// Update function signature
pub fn update(&mut self, message: Message) -> Task<Message> {
    // Handle messages and return async tasks
}

// View function signature  
pub fn view(&self) -> Element<'_, Message> {
    // Return Iced UI elements
}
```

### File Organization

- `src/main.rs` - Application entry point and message handling
- `src/app.rs` - App state and configuration
- `src/search.rs` - Search logic and Excel indexing

### Testing

- Currently no tests exist in the project
- When adding tests, place them in `tests/` directory or use `#[cfg(test)]` modules
- Use descriptive test names: `test_function_name_when_condition()`

### Dependencies

Only add new dependencies when necessary. Current dependencies:
- `calamine` (0.26) - Excel reading
- `iced` (0.14) - GUI with tokio and highlighter features
- `rfd` (0.15) - Native file dialogs
- `tokio` (1.0) - Async runtime with full features
- `dirs` (5) - Config directory detection

## Working with This Codebase

### Adding New Features

1. For search logic: modify `src/search.rs`
2. For UI changes: modify `src/main.rs` (view/update) and `src/app.rs` (state)
3. Add new dependencies to `Cargo.toml`

### Building for Release

```bash
cargo build --release
# Binary will be in target/release/excaligrep
```

### Common Issues

- The app uses `rg` (ripgrep) for text search - ensure it's installed
- Excel files are converted to CSV in `.csv` folder for searching
- Config is stored in `~/.config/excaligrep/last_folder`
