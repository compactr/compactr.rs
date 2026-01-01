# Contributing to Compactr

Thank you for your interest in contributing to Compactr! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/compactr.rs`
3. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```bash
cargo build --all-features
```

### Running Tests

```bash
cargo test --all-features
```

### Code Quality

Before submitting a PR, please ensure:

1. All tests pass: `cargo test --all-features`
2. Code is formatted: `cargo fmt --all`
3. Clippy passes: `cargo clippy --all-targets --all-features -- -D warnings`
4. Documentation builds: `cargo doc --no-deps --all-features`

## Code Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `rustfmt` for formatting (configuration in `rustfmt.toml`)
- Address all `clippy` warnings (configuration in `clippy.toml`)
- Write comprehensive documentation for public APIs
- Include examples in documentation where helpful

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Add tests for new functionality
3. Ensure all CI checks pass
4. Update CHANGELOG.md with a brief description of changes
5. Request review from maintainers

## Commit Messages

- Use clear, descriptive commit messages
- Follow the [Conventional Commits](https://www.conventionalcommits.org/) format:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `test:` for test additions/changes
  - `refactor:` for code refactoring
  - `perf:` for performance improvements
  - `chore:` for maintenance tasks

Example: `feat: add UUID format encoding support`

## Testing

- Write unit tests for individual components
- Write integration tests for complex workflows
- Add benchmarks for performance-critical code
- Ensure cross-platform compatibility (Linux, macOS, Windows)

## Documentation

- Document all public APIs with rustdoc comments
- Include examples in documentation
- Update README.md for significant changes
- Add examples in the `examples/` directory for new features

## License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as the project.

## Questions?

Feel free to open an issue for questions or discussion!
