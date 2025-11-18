# Rust Find

`rust_find` is a simple, cross-platform command-line utility written in Rust to find files by a substring in their name. It serves as a basic alternative to more complex tools like `find`.

## Features

-   Recursively search a directory for files.
-   Filter files by a substring contained in their name.
-   Limit the depth of the directory search.
-   Cross-platform support (Windows, macOS, Linux).

## Installation

You can install `rust_find` directly from the source using `cargo`:

```bash
# Clone the repository
# git clone <repository_url>
# cd rust_find

# Install the binary from the project root
cargo install --path .
```

## Usage

Once installed, you can use the `rust_find` command from anywhere in your terminal.

```bash
rust_find --base-dir <PATH> --contains <SUBSTRING> [--max-depth <DEPTH>]
```

### Arguments

-   `--base-dir <DIR>`: The directory to start the search from.
-   `--contains <STRING>`: The substring to search for in filenames.
-   `--max-depth <DEPTH>`: (Optional) The maximum depth for directory traversal. Defaults to `20`.

### Examples

-   Search for all files containing "main" in the current directory and its subdirectories:
    ```bash
    rust_find --base-dir . --contains "main"
    ```

-   Search for files containing ".toml" in your home directory, up to a depth of 3:
    ```bash
    rust_find --base-dir ~/ --contains ".toml" --max-depth 3
    ```

## Building from Source

To build the project, clone the repository and use the standard Cargo commands.

```bash
# Build in debug mode
cargo build

# Build in release mode (optimized for performance)
cargo build --release
```
The executable will be located in `target/debug/rust_find` or `target/release/rust_find`.

## Running Tests

To run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License.
