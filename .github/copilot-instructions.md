# GitHub Copilot Instructions for uv-align

## Project Overview

`uva` is a Rust command-line tool that synchronizes Python dependency constraints between `pyproject.toml` and `uv.lock`. It bridges the gap when `uv lock --upgrade` updates the lockfile but leaves `pyproject.toml` constraints unchanged.

**Key Principles:**

- File transformer - reads two files, writes one (no network access by default)
- Preserves user formatting, operators, upper bounds, and environment markers
- Never modifies `uv.lock` directly - that's `uv`'s responsibility
- No virtual environment required

## Architecture

### Module Structure

- **`main.rs`** - Entry point, orchestrates the workflow (CLI → validation → diff → apply)
- **`cli.rs`** - Command-line argument parsing and validation using `clap`
- **`pyproject.rs`** - Parses and updates `pyproject.toml` using `toml_edit` (preserves formatting)
- **`lockfile.rs`** - Parses `uv.lock` to extract resolved versions
- **`lib.rs`** - Core business logic (dependency mapping, diff computation, uv command execution)

### Key Data Structures

```rust
// From pyproject.toml
PyprojectDependency {
    name: String,              // User-written name
    normalised_name: String,   // PEP 503 normalized
    version: Option<String>,   // e.g., "1.2.3"
    operator: Option<String>,  // e.g., ">=", "==", "~="
    suffix: Option<String>,    // e.g., ",<2.0", ",!=1.0.0"
    group: Option<String>,     // dependency group
}

// From uv.lock
LockDependency {
    name: String,
    normalised_name: String,
    version: String,           // Resolved version
}

// After mapping
MappedDependency {
    pyproject: PyprojectDependency,
    lock: LockDependency,
}

// For diff output
DependencyChange {
    name: String,
    operator: Option<String>,
    old: String,               // Version from pyproject.toml
    new: String,               // Version from uv.lock
    suffix: Option<String>,
}
```

## Coding Conventions

### Style Guidelines

1. **Rust Edition 2024** - Use latest stable features
2. **Error Handling** - Use `anyhow::Result` for error propagation
3. **Formatting** - Follow standard `rustfmt` configuration
4. **Naming** - Use snake_case for functions/variables, PascalCase for types
5. **Documentation** - Include doc comments (`///`) for public APIs

### File Operations

- Use `toml_edit` for `pyproject.toml` to preserve formatting and comments
- Use `toml` crate for `uv.lock` parsing (formatting not needed)
- Always validate file existence before operations
- Use `std::path::Path` for cross-platform path handling

### Dependency Normalization

Follow PEP 503 normalization rules:

- Lowercase
- Replace `_`, `.`, `-` with single `-`
- Example: `My_Package.Name` → `my-package-name`

### Terminal UI

- Use `owo-colors` for colored output
- Use `crossterm` for terminal interaction (prompts, key events)
- Color scheme:
  - `.bright_green()` - Success, commands to run
  - `.bright_yellow()` - Warnings
  - `.bright_red()` - Errors, out-of-sync counts
  - `.bright_blue()` - File names
  - `.underline()` - Section headers

### Exit Codes

```rust
0   - Success
1   - General error (anyhow::Error)
2   - Validation error (missing files, invalid args)
126 - Command execution failed (uv lock --upgrade)
127 - Command not found (uv not installed)
```

## Key Workflows

### Standard Flow

1. **Validation** - Check paths, files exist
2. **Optional Upgrade** - Run `uv lock --upgrade` if `--upgrade` flag set
3. **Parse** - Read dependencies from `pyproject.toml` and versions from `uv.lock`
4. **Map** - Match pyproject dependencies to lockfile versions by normalized name
5. **Diff** - Compute version changes
6. **Display** - Show diff with color coding
7. **Confirm** - Prompt user (unless `--yes` or `--check`)
8. **Apply** - Update `pyproject.toml` preserving formatting

### Flag Combinations

- `--check` - Dry run, show diff only (exit code 0 = in sync, 1 = out of sync)
- `-y, --yes` - Auto-approve changes
- `-u, --upgrade` - Run `uv lock --upgrade` first
- `-v, --verbose` - Show detailed uv output
- Conflicting: `--check` and `--yes` cannot be used together

## Dependencies

### Core Dependencies

- **`clap`** (v4.6+) - CLI argument parsing with derive macros
- **`toml_edit`** (v0.25+) - Format-preserving TOML editing
- **`toml`** (v1.1+) - TOML parsing for lockfile
- **`anyhow`** (v1.0+) - Error handling and context
- **`owo-colors`** (v4.3+) - Terminal colors
- **`crossterm`** (v0.29+) - Terminal interaction
- **`tempfile`** (v3.27+) - Testing support

### Build System

- **Rust** - 1.95.0 or later
- **Maturin** - For building Python wheels (v1.9.3+)
- **cargo-dist** - For release builds

## Testing

### Running Tests

```bash
cargo test                 # Run all tests
cargo test --release       # Run optimized tests
cargo build --release      # Build release binary
```

### Test Structure

- Unit tests in module files (e.g., `#[cfg(test)] mod tests`)
- Integration tests should use `tempfile` for temporary directories
- Test both success and error paths
- Mock file operations when testing parsing logic

## Building and Distribution

### Local Development

```bash
cargo build                # Debug build
cargo build --release      # Release build
cargo run -- --help        # Run with args
```

### Python Package

```bash
maturin develop            # Install in dev mode
maturin build              # Build wheel
maturin build --release    # Release wheel
```

### Cross-Platform Considerations

- Use `std::path::Path` not string concatenation
- Test on Linux, macOS, Windows
- Handle different line endings gracefully
- Use `cargo-dist` for release binaries

## Common Patterns

### Reading Dependencies

```rust
// Use toml_edit for format preservation
let content = fs::read_to_string(path)?;
let doc = content.parse::<toml_edit::DocumentMut>()?;
```

### Comparing Versions

- Extract version strings from both files
- Compare as strings (semantic version parsing not needed)
- Preserve original operators and suffixes

### User Interaction

```rust
// Prompt pattern with crossterm
terminal::enable_raw_mode()?;
loop {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('y') | KeyCode::Enter => { /* apply */ },
            KeyCode::Char('n') | KeyCode::Esc => { /* cancel */ },
            _ => continue,
        }
        break;
    }
}
terminal::disable_raw_mode()?;
```

## Future Considerations

- Support for multiple lockfile formats
- Configurable normalization rules
- Batch mode for CI/CD pipelines
- Integration with pre-commit hooks
- Support for workspace dependencies

## References

- [PEP 503 - Simple Repository API](https://peps.python.org/pep-0503/) - Name normalization
- [PEP 621 - Pyproject TOML](https://peps.python.org/pep-0621/) - pyproject.toml format
- [uv documentation](https://docs.astral.sh/uv/) - uv lockfile format
