# 🔧 Hash Forge

> **Professional hash generator and verifier built with Rust** - Multiple algorithms, dual interfaces, and enterprise-grade security

[![CI](https://github.com/fra2404/hash-forge/actions/workflows/ci.yml/badge.svg)](https://github.com/fra2404/hash-forge/actions/workflows/ci.yml)
[![Release](https://github.com/fra2404/hash-forge/actions/workflows/release.yml/badge.svg)](https://github.com/fra2404/hash-forge/actions/workflows/release.yml)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Hash Forge** is a powerful hash generator and verifier written in Rust that supports multiple cryptographic algorithms with both CLI and GUI interfaces.

## ✨ Features

- 🔐 **Multiple algorithms**: SHA-256/512, BLAKE2/3, MD5, SHA-1, Argon2, bcrypt, scrypt
- 📱 **Dual interface**: Both CLI and GUI modes available
- ⚡ **High performance**: Rust implementation with progress tracking
- 🔍 **Hash verification**: Compare computed hashes against expected values
- 📁 **Batch processing**: Process entire directories
- 🛡️ **Security focused**: Constant-time comparisons, secure salt generation
- 🎨 **User friendly**: Real-time GUI updates, file drag & drop
- 🌍 **Cross-platform**: Works on macOS, Linux, and Windows

## 🚀 Quick Start

### 🍺 Install with Homebrew (macOS/Linux - Recommended)

```bash
# Add the Hash Forge tap (coming soon)
brew tap fra2404/hash-forge

# Install Hash Forge CLI and GUI
brew install hash-forge
```

### 📦 Install with Cargo

```bash
# Install from crates.io (coming soon)
cargo install hash-forge

# Or install from source
git clone https://github.com/fra2404/hash-forge.git
cd hash-forge
cargo install --path .
```

### 📥 Download Pre-built Binaries

Get the latest release from [GitHub Releases](https://github.com/fra2404/hash-forge/releases/latest):

- **macOS**: Download the `.dmg` installer (includes app with icon) or CLI-only `.tar.gz`
- **Linux**: Download the `.tar.gz` file (GUI + CLI)
- **Windows**: Download the `.zip` file (GUI + CLI)

> **Automatic Releases**: New releases are automatically created when tags are pushed to GitHub. The macOS `.dmg` includes a beautifully packaged app with icon and drag-to-Applications installer.

### 🔒 macOS Security Notice

On macOS, you may see a warning: *"Apple could not verify 'Hash Forge' is free of malware"*. This is normal for open-source applications that aren't distributed through the Mac App Store.

**To run Hash Forge safely:**

1. **Right-click** the app and select "Open" (instead of double-clicking)
2. Click "Open" in the dialog that appears
3. The app will run and be remembered as safe for future use

**Alternative method:**
```bash
# Allow the app to run via terminal
xattr -d com.apple.quarantine "/Applications/Hash Forge.app"
```

> This is a standard procedure for all open-source macOS applications. Hash Forge is built from public source code and includes ad-hoc code signing for additional security.

## 💻 Usage

### CLI Interface

```bash
# Hash text
hash-forge text --input "Hello, World!" --algorithm sha256

# Hash file
hash-forge file --path document.pdf --algorithm blake3

# Verify hash
hash-forge verify text --value "Hello, World!" --expected-hash "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e" --algorithm sha256

# Batch process directory
hash-forge batch --directory ./files --algorithm sha256

# Password hashing with custom salt
hash-forge text --input "mypassword" --algorithm argon2 --salt "customsalt" --iterations 4096
```

### GUI Interface

```bash
# Launch interactive GUI
hash-forge-gui
```

The GUI provides:

- 📝 **Text input** with real-time hashing
- 📄 **File selection** with drag & drop support
- ⚙️ **Algorithm selection** with recommendations
- 🔍 **Hash verification** mode
- 📊 **Performance metrics** (time, file size)
- 📋 **Copy to clipboard** functionality

## 🎯 Project Status

✅ **CLI Implementation**: Complete and fully functional  
✅ **GUI Implementation**: Complete with egui/eframe  
✅ **All Hash Algorithms**: MD5, SHA-1, SHA-256, SHA-512, BLAKE2, BLAKE3, bcrypt, scrypt, Argon2  
✅ **File & Text Hashing**: Both supported with progress indicators  
✅ **Hash Verification**: Full verification against expected values  
✅ **Batch Processing**: Directory processing implemented  
✅ **Cross-platform**: macOS, Linux, and Windows support  
✅ **Documentation**: Comprehensive examples and usage guides

🚧 **In Progress**: Homebrew tap setup, crates.io publication

## 🔐 Supported Algorithms

### Fast Hash Algorithms (Files & Data)

- **SHA-256** - General purpose, widely supported
- **SHA-512** - High security, larger output
- **BLAKE3** - Modern, fastest performance ⭐ **Recommended**
- **BLAKE2b/2s** - High performance alternatives
- **SHA-1** - Legacy compatibility (not recommended for security)
- **MD5** - Legacy compatibility (not recommended for security)

### Password Hash Algorithms (Slow by Design)

- **Argon2** - Modern, memory-hard ⭐ **Recommended for passwords**
- **bcrypt** - Widely supported, moderate security
- **scrypt** - Memory-hard, good security

## 🔧 Algorithm Recommendations

| Use Case                 | Recommended Algorithm | Why                                  |
| ------------------------ | --------------------- | ------------------------------------ |
| **File integrity**       | BLAKE3 or SHA-256     | Fast, secure, widely supported       |
| **Password hashing**     | Argon2                | Modern, memory-hard, configurable    |
| **Digital signatures**   | SHA-256 or SHA-512    | Standard for cryptographic protocols |
| **High performance**     | BLAKE3                | Fastest modern cryptographic hash    |
| **Legacy compatibility** | SHA-1 or MD5          | Only when required by old systems    |

## ⚡ Performance

Hash Forge is optimized for performance:

- **BLAKE3**: ~3 GB/s (fastest)
- **SHA-256**: ~1.5 GB/s
- **SHA-512**: ~2 GB/s
- **Progress tracking** for large files
- **Constant-time** hash verification
- **Memory efficient** streaming for large files

## 🛡️ Security Features

- **Constant-time comparisons** prevent timing attacks
- **Secure salt generation** using cryptographically secure RNG
- **Memory safety** through Rust's ownership system
- **Clear error messages** without exposing sensitive data
- **Configurable iterations** for password hashing algorithms

## 🔧 Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/fra2404/hash-forge.git
cd hash-forge

# Build CLI only
cargo build --release --bin hash-forge

# Build GUI (requires GUI feature)
cargo build --release --bin hash-forge-gui --features gui

# Build everything and create macOS DMG (macOS only)
./build.sh

# Run tests
cargo test

# Run CLI
cargo run --bin hash-forge -- text --input "test"

# Run GUI
cargo run --bin hash-forge-gui --features gui
```

### Project Structure

```
hash-forge/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── gui_main.rs      # GUI entry point
│   ├── lib.rs           # Library exports
│   ├── algorithms.rs    # Hash algorithm definitions
│   ├── cli.rs           # CLI argument parsing
│   ├── core.rs          # Core hashing engine
│   ├── gui_core.rs      # GUI application logic
│   ├── output.rs        # Output formatting
│   └── utils.rs         # Utility functions
├── assets/              # Project assets
│   └── image.png        # App icon (1024x1024)
├── build/               # Build output directory (ignored by git)
│   ├── Hash-Forge.dmg   # macOS installer
│   └── *.tar.gz         # Distribution archives
├── docs/                # GitHub Pages website
├── scripts/             # Build and utility scripts
└── tests/               # Integration tests
```

## 📋 Examples

### Text Hashing

```bash
# Basic SHA-256
hash-forge text -i "Hello, World!" -a sha256
# Output: a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e

# BLAKE3 with base64 output
hash-forge text -i "Hello, World!" -a blake3 -f base64
# Output: 4/W8p8yoEQJ7TUJTPAj7Xk/8Z1Lj0EWw8BjSXGJNZyg=

# Password hashing with Argon2
hash-forge text -i "mypassword" -a argon2 --salt "randomsalt" --iterations 4096
```

### File Hashing

```bash
# Hash a document
hash-forge file -p document.pdf -a sha256

# Hash with BLAKE3 for speed
hash-forge file -p largefile.zip -a blake3

# Hash with progress tracking (automatic for large files)
hash-forge file -p video.mp4 -a sha256
```

### Hash Verification

```bash
# Verify file integrity
hash-forge verify file -p document.pdf -e "expected_hash_here" -a sha256

# Verify text hash
hash-forge verify text -v "Hello, World!" -e "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e" -a sha256
```

### Batch Processing

```bash
# Hash all files in directory
hash-forge batch -d ./documents -a sha256

# Process with different algorithm
hash-forge batch -d ./images -a blake3 -f base64
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- 🔐 Additional hash algorithms
- 🎨 GUI improvements and themes
- 📱 Mobile/web interfaces
- 🧪 More comprehensive tests
- 📚 Documentation improvements
- 🌍 Internationalization

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Homepage**: [https://fra2404.github.io/hash-forge/](https://fra2404.github.io/hash-forge/)
- **Repository**: [https://github.com/fra2404/hash-forge](https://github.com/fra2404/hash-forge)
- **Issues**: [https://github.com/fra2404/hash-forge/issues](https://github.com/fra2404/hash-forge/issues)
- **Releases**: [https://github.com/fra2404/hash-forge/releases](https://github.com/fra2404/hash-forge/releases)

## 💝 Support

If you find Hash Forge useful, please consider:

- ⭐ **Starring** the repository
- 🐛 **Reporting bugs** or requesting features
- 📝 **Contributing** code or documentation
- ☕ **Supporting development** via [Ko-fi](https://ko-fi.com/fra2404)

---

**Made with ❤️ and ⚡ by [Francesco](https://github.com/fra2404)**
