# Contributing to reverse-ssh

Thank you for your interest in contributing to reverse-ssh! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Testing](#testing)
- [Code Style](#code-style)
- [Submitting Changes](#submitting-changes)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment
4. Create a new branch for your changes
5. Make your changes
6. Test thoroughly
7. Submit a pull request

## How to Contribute

### Types of Contributions

We welcome:
- 🐛 Bug fixes
- ✨ New features
- 📝 Documentation improvements
- 🎨 Code quality improvements
- 🧪 Additional tests
- 🌍 Translation/internationalization
- 💡 Feature suggestions

### Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/rrp.git
cd rrp

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example localhost_run
```

### Prerequisites

- Rust 1.87+ (nightly)
- Git
- SSH client (for testing)
- Optionally: An SSH server for testing

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_config_creation

# Run with debug output
RUST_LOG=debug cargo test
```

### Testing Examples

```bash
# Test localhost_run example
cargo run --example localhost_run

# Test simple_server
cargo run --example simple_server

# Test with custom SSH server
export SSH_HOST=your-server.com
export SSH_USER=your-username
export SSH_KEY=~/.ssh/id_rsa
cargo run --example local_test
```

### Adding Tests

When adding new functionality:
1. Write unit tests in the same file as the code
2. Write integration tests in `tests/` directory
3. Add examples if the feature is user-facing
4. Update documentation

Example test:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Your test here
        assert_eq!(2 + 2, 4);
    }
}
```

## Code Style

### Rust Guidelines

Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/):

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Check everything
cargo fmt && cargo clippy && cargo test
```

### Code Conventions

- Use meaningful variable names
- Add comments for complex logic
- Keep functions small and focused
- Use `Result` and `?` for error handling
- Prefer `async/await` over callbacks
- Use `tracing` for logging, not `println!`

### Documentation

- Add doc comments to public functions
- Include examples in doc comments
- Update README.md for user-facing changes
- Keep CHANGELOG.md up to date

Example:
```rust
/// Connects to the SSH server and authenticates.
///
/// # Arguments
///
/// * `tx` - Channel for forwarding connections
/// * `message_tx` - Channel for server messages
///
/// # Example
///
/// ```no_run
/// let mut client = ReverseSshClient::new(config);
/// client.connect(tx, message_tx).await?;
/// ```
///
/// # Errors
///
/// Returns an error if connection or authentication fails.
pub async fn connect(&mut self, ...) -> Result<()> {
    // Implementation
}
```

## Submitting Changes

### Pull Request Process

1. **Update your fork**
   ```bash
   git remote add upstream https://github.com/aovestdipaperino/rrp.git
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Write clean, documented code
   - Add tests
   - Update documentation
   - Follow code style guidelines

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "Add feature: your feature description"
   ```

   Good commit message format:
   ```
   Add feature: Brief description (50 chars or less)

   More detailed explanation if needed. Wrap at 72 characters.

   - Bullet points are okay
   - Include context and reasoning
   - Reference issues: Fixes #123
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**
   - Go to GitHub and create a PR
   - Fill out the PR template
   - Link related issues
   - Add screenshots for UI changes

### PR Checklist

Before submitting:
- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Examples work if applicable
- [ ] Commit messages are clear

### Review Process

1. Maintainers will review your PR
2. Address any feedback
3. Once approved, your PR will be merged
4. Your contribution will be in the next release!

## Reporting Bugs

### Before Submitting

1. Check if the bug is already reported
2. Try the latest version
3. Gather information about the bug

### Bug Report Template

```markdown
**Description**
A clear description of the bug.

**To Reproduce**
Steps to reproduce:
1. Run `cargo run --example localhost_run`
2. See error

**Expected Behavior**
What you expected to happen.

**Actual Behavior**
What actually happened.

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.87.0]
- reverse-ssh version: [e.g., 0.1.0]

**Logs**
```
Paste debug logs here (RUST_LOG=debug)
```

**Additional Context**
Any other relevant information.
```

## Suggesting Features

### Feature Request Template

```markdown
**Is your feature related to a problem?**
A clear description of the problem.

**Describe the solution**
How you'd like it to work.

**Alternatives considered**
Other solutions you've thought about.

**Additional context**
Mockups, examples, use cases.
```

### Feature Discussion

1. Open an issue to discuss the feature
2. Get feedback from maintainers
3. If approved, implement it following these guidelines
4. Submit a PR

## Areas Needing Help

We especially welcome contributions in:

- 🧪 **Testing**: More tests and edge cases
- 📝 **Documentation**: Improving examples and guides
- 🌐 **Compatibility**: Testing on different platforms
- 🚀 **Performance**: Optimization improvements
- 🔒 **Security**: Security audits and improvements
- 🎨 **Examples**: More real-world use cases
- 🐛 **Bug Fixes**: Fixing reported issues

## Development Tips

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run --example localhost_run

# Enable trace logging (very verbose)
RUST_LOG=trace cargo run --example localhost_run

# Run with backtrace
RUST_BACKTRACE=1 cargo run --example localhost_run
```

### Testing SSH Connections

```bash
# Test with standard SSH first
ssh -R 80:localhost:8080 localhost.run

# Compare with library behavior
RUST_LOG=debug cargo run --example localhost_run
```

### Benchmarking

```bash
# Run benchmarks (if available)
cargo bench

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --example localhost_run
```

## Documentation Guidelines

### README.md

- Keep it concise
- Include quick start
- Link to detailed docs
- Add badges

### Code Comments

```rust
// Good: Explains why
// Use empty string for bind address because localhost.run expects it
handle.tcpip_forward("", port).await?;

// Bad: Explains what (obvious from code)
// Call tcpip_forward with empty string and port
handle.tcpip_forward("", port).await?;
```

### Examples

- Keep examples simple and focused
- Add comments explaining each step
- Include error handling
- Show expected output

## Release Process

Maintainers follow this process for releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run all tests
4. Create git tag
5. Publish to crates.io
6. Create GitHub release

## Getting Help

- 💬 Open an issue for questions
- 📧 Email: enzinol@gmail.com
- 📖 Check the documentation
- 🔍 Search existing issues

## Recognition

Contributors will be:
- Listed in CHANGELOG.md
- Credited in release notes
- Mentioned in README.md (for significant contributions)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

Thank you for contributing to reverse-ssh! 🎉
