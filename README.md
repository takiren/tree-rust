# tree-rust

A Rust implementation of the classic `tree` command for displaying directory structures in a tree format.

## Features

- **Directory Tree Display**: Shows files and directories in a hierarchical tree structure
- **Depth Control**: Limit display depth with `--depth` option
- **Filtering**: Show only files (`--files-only`) or only directories (`--dirs-only`)
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Fast Performance**: Built with Rust for optimal speed and memory usage

## Installation

### From Source

```bash
git clone https://github.com/yourusername/tree-rust.git
cd tree-rust
cargo build --release
# The binary will be available at target/release/tree-rust
```

## Usage

```bash
tree-rust [OPTIONS] [PATH]
```

### Arguments

- `PATH`: Directory path to display (default: current directory)

### Options

- `-d, --depth <DEPTH>`: Maximum depth to display
- `-f, --files-only`: Show files only
- `-D, --dirs-only`: Show directories only
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Examples

```bash
# Display current directory tree
tree-rust

# Display specific directory
tree-rust /path/to/directory

# Limit depth to 2 levels
tree-rust --depth 2

# Show only files
tree-rust --files-only

# Show only directories
tree-rust --dirs-only

# Combine options
tree-rust --depth 3 --files-only src/
```

### Sample Output

```
project/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── tree.rs
├── tests/
│   └── integration_test.rs
├── Cargo.toml
└── README.md
```

## Error Handling

The tool provides clear error messages for common issues:

- **Non-existent paths**: "Path does not exist: /path/to/nowhere"
- **Permission denied**: "Permission denied (os error 13)"
- **Invalid depth values**: "invalid digit found in string"

## Development

### Prerequisites

- Rust 1.70.0 or later
- Cargo (comes with Rust)

### Building

```bash
# Build for development
cargo build

# Build optimized release version
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_tree_display
```

The project includes comprehensive integration tests covering:
- Basic tree display functionality
- Depth limiting
- File/directory filtering
- Error handling
- Help display
- Edge cases (empty directories, invalid inputs)

### Code Quality

```bash
# Run clippy for linting
cargo clippy

# Format code
cargo fmt

# Check without building
cargo check
```

## Project Structure

```
tree-rust/
├── src/
│   ├── main.rs        # Entry point
│   ├── lib.rs         # Library exports
│   ├── tree.rs        # Core tree display logic
│   └── args.rs        # Command-line argument parsing
├── tests/
│   └── integration_tests.rs  # Integration tests
├── docs/
│   └── plan.md        # Development plan
├── Cargo.toml         # Project configuration
├── CLAUDE.md          # Development guidelines
└── README.md          # This file
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run code quality checks (`cargo clippy && cargo fmt`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the classic Unix `tree` command
- Built with the excellent [clap](https://crates.io/crates/clap) crate for argument parsing
- Uses [tempfile](https://crates.io/crates/tempfile) for testing temporary directories