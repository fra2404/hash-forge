# Contributing to Hash Forge

Thank you for your interest in contributing to Hash Forge! This document provides guidelines and information for contributors.

## 🤝 Ways to Contribute

- **Bug Reports**: Report bugs or issues you encounter
- **Feature Requests**: Suggest new features or improvements
- **Code Contributions**: Submit pull requests with fixes or new features
- **Documentation**: Improve documentation, examples, or guides
- **Testing**: Help test the application on different platforms
- **Translations**: Help translate the interface to other languages (future)

## 🐛 Reporting Bugs

Before creating a bug report, please:

1. **Check existing issues** to avoid duplicates
2. **Use the latest version** to ensure the bug hasn't been fixed
3. **Provide clear steps** to reproduce the issue

### Bug Report Template

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment (please complete the following information):**
- OS: [e.g. macOS 14.0, Windows 11, Ubuntu 22.04]
- Hash Forge Version: [e.g. 1.0.0]
- Installation method: [e.g. DMG, Homebrew, compiled from source]

**Additional context**
Add any other context about the problem here.
```

## 💡 Feature Requests

We welcome feature requests! Please:

1. **Check existing feature requests** first
2. **Describe the use case** clearly
3. **Explain why** this feature would be valuable
4. **Consider implementation complexity**

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
A clear and concise description of what the problem is.

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

## 🔧 Development Setup

### Prerequisites

- **Rust** (latest stable version)
- **Git**
- **macOS**: Xcode Command Line Tools
- **Linux**: Build essentials, pkg-config
- **Windows**: Visual Studio Build Tools

### Setting Up the Development Environment

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/hash-forge.git
   cd hash-forge
   ```

2. **Install Rust dependencies**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Run the CLI version**:
   ```bash
   cargo run --bin hash-forge -- --help
   ```

5. **Run the GUI version**:
   ```bash
   cargo run --bin hash-forge-gui
   ```

### Development Commands

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code formatting
cargo fmt --check

# Apply code formatting
cargo fmt

# Run clippy (linter)
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Build release version
cargo build --release

# Run local build script (macOS)
./build.sh
```

## 📝 Code Guidelines

### Code Style

- **Follow Rust conventions** and idioms
- **Use `cargo fmt`** for consistent formatting
- **Fix all `cargo clippy` warnings**
- **Write clear, self-documenting code**
- **Add comments for complex algorithms**

### Rust-Specific Guidelines

- **Use explicit error handling** with `Result<T, E>`
- **Prefer `?` operator** for error propagation
- **Use `anyhow` for application errors**, `thiserror` for library errors
- **Document public functions** with doc comments
- **Include examples** in documentation when helpful
- **Use `const` for compile-time constants**
- **Prefer `&str` over `String` for function parameters** when possible

### Project Structure

```
src/
├── main.rs          # CLI entry point
├── gui_main.rs      # GUI entry point
├── lib.rs           # Library root
├── core.rs          # Core business logic
├── algorithms.rs    # Hash algorithm implementations
├── cli.rs           # CLI argument parsing and logic
├── gui_core.rs      # GUI application logic
├── output.rs        # Output formatting
└── utils.rs         # Utility functions
```

### Algorithm Implementation

When adding new hash algorithms:

1. **Add to `algorithms.rs`**
2. **Update `HashAlgorithm` enum**
3. **Implement the algorithm function**
4. **Add CLI support** in `cli.rs`
5. **Add GUI support** in `gui_core.rs`
6. **Add tests** for the new algorithm
7. **Update documentation**

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_sha256

# Run tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

### Writing Tests

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test CLI and GUI functionality
- **Property tests**: Use property-based testing for hash properties
- **Cross-platform tests**: Ensure compatibility across platforms

### Test Guidelines

- **Test all public functions**
- **Test error conditions**
- **Use descriptive test names**
- **Include edge cases**
- **Test with different input sizes**

## 📋 Pull Request Process

### Before Submitting

1. **Create an issue** first (for significant changes)
2. **Fork the repository**
3. **Create a feature branch**: `git checkout -b feature/amazing-feature`
4. **Make your changes**
5. **Add tests** for new functionality
6. **Update documentation** if needed
7. **Run all tests**: `cargo test`
8. **Check formatting**: `cargo fmt --check`
9. **Check linting**: `cargo clippy`

### Pull Request Guidelines

- **Use clear, descriptive titles**
- **Reference related issues**: "Fixes #123"
- **Describe what changed** and why
- **Include testing information**
- **Update CHANGELOG.md** if applicable
- **Keep changes focused** (one feature per PR)

### Pull Request Template

```markdown
## Description
Brief description of the changes.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## How Has This Been Tested?
- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing
- [ ] Cross-platform testing

## Checklist:
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
```

## 🏗️ Architecture Guidelines

### Core Principles

- **Separation of Concerns**: Keep CLI, GUI, and core logic separate
- **Shared Logic**: Common functionality in shared modules
- **Error Handling**: Consistent error handling throughout
- **Performance**: Optimize for both memory and speed
- **Security**: Use constant-time operations where appropriate

### Adding New Features

1. **Core Logic**: Implement in `core.rs` or appropriate module
2. **CLI Interface**: Add command-line interface in `cli.rs`
3. **GUI Interface**: Add GUI interface in `gui_core.rs`
4. **Tests**: Add comprehensive tests
5. **Documentation**: Update relevant documentation

## 🔒 Security Considerations

When contributing security-related code:

- **Use well-established cryptographic libraries**
- **Implement constant-time comparisons** for hash verification
- **Clear sensitive data** from memory when possible
- **Follow security best practices** for the algorithms
- **Consider timing attacks** in implementation

## 📚 Documentation Guidelines

### Code Documentation

- **Document all public functions** with rustdoc comments
- **Include examples** in documentation
- **Explain complex algorithms**
- **Document error conditions**
- **Keep documentation up-to-date**

### User Documentation

- **Update README.md** for significant changes
- **Update EXAMPLES.md** for new features
- **Keep installation instructions current**
- **Document breaking changes**

## 🌍 Community Guidelines

### Code of Conduct

- **Be respectful** and inclusive
- **Focus on constructive feedback**
- **Help newcomers** learn and contribute
- **Assume good intentions**
- **Follow GitHub's Community Guidelines**

### Communication

- **Use GitHub Issues** for bug reports and feature requests
- **Use GitHub Discussions** for general questions
- **Use clear, professional language**
- **Provide context** for your contributions

## 🚀 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run full test suite
- [ ] Update documentation
- [ ] Create release tag
- [ ] Build release artifacts
- [ ] Update Homebrew tap (if applicable)

## ❓ Getting Help

If you need help:

1. **Check the documentation** first
2. **Search existing issues** for similar problems
3. **Create a new issue** with details
4. **Use GitHub Discussions** for general questions

## 🙏 Recognition

Contributors will be recognized in:

- **README.md** contributors section
- **Release notes** for significant contributions
- **Git commit history** (always preserved)

Thank you for contributing to Hash Forge! 🔨✨
