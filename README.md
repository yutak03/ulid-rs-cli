# ULID Generator for Rust

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

A command-line tool written in Rust for generating ULIDs.   
This tool allows you to quickly generate one or more ULIDs with configurable options.

## What is a ULID?

A ULID is a 128-bit identifier that combines:
- A timestamp with millisecond precision
- A cryptographically secure random component

ULIDs provide several advantages over UUIDs:
- Lexicographically sortable by time of creation
- Case-insensitive
- No special characters (URL safe)
- Monotonically increasing when created in the same millisecond

This implementation uses the [ulid-rs](https://github.com/dylanhart/ulid-rs) library under the hood.

## Installation

### Using Cargo

```bash
cargo install --git https://github.com/yutak03/ulid-generator-rs.git
```

### Build from Source

1. Clone the repository

```bash
git clone https://github.com/yutak03/ulid-generator-rs.git
cd ulid-generator-rs
```

2. Build and install

```bash
# Using cargo-make
cargo install cargo-make  # If you don't have cargo-make installed
cargo make install
```

Installation will place the binary at `/usr/local/bin/ulid` by default.

## Usage

```bash
ulid [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-C, --count <COUNT>` | Number of ULIDs to generate (default: 1) |
| `-I, --interval <INTERVAL>` | Interval between ULID generation in milliseconds (default: 100) |
| `-N, --nil` | Generate nil ULID (all zeros) |
| `-h, --help` | Show help information |
| `-V, --version` | Show version information |

### Examples

Generate a single ULID:
```bash
ulid
```

Generate 5 ULIDs:
```bash
ulid -C 5
```

Generate 3 ULIDs with a 500ms interval:
```bash
ulid -C 3 -I 500
```

Generate a nil ULID:
```bash
ulid --nil
```

## Development

### Prerequisites

- Rust 1.56.0 or later
- Cargo

### Testing

```bash
cargo test
```

### Using Cargo-make

We support cargo-make for development tasks:

```bash
# Run tests
cargo make test

# Run linting
cargo make clippy

# Generate documentation
cargo make doc

# Clean build artifacts
cargo make clean
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Author

- Yutak (yutak03.dev@gmail.com)