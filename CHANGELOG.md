# Changelog

All notable changes to Hash Forge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-01-01

### Added
- **New Hash Algorithms**:
  - SHA-3 family: SHA3-224, SHA3-256, SHA3-384, SHA3-512
  - SHAKE functions: SHAKE128, SHAKE256 (extendable output functions)
  - xxHash family: xxHash32, xxHash64, xxHash3 (high-performance hashing)
- **HMAC Support**:
  - Complete HMAC implementation for all supported algorithms
  - CLI commands: `hmac` and `verify-hmac`
  - HMAC mode in GUI with key input
  - Constant-time verification for security
- **GUI Enhancements**:
  - Modern dark theme with professional styling
  - Real-time auto-compute functionality
  - Visual algorithm indicators (emoji-based categorization)
  - File drag & drop support
  - Progress tracking for large file operations
  - Algorithm filtering by category (Modern, Fast Hash, Password Hash, Legacy)
- **New Modules**:
  - `hmac_core.rs` - Dedicated HMAC implementation
  - `gui/` directory with modular GUI structure:
    - `app_state.rs` - Application state management
    - `algorithms.rs` - Algorithm filtering and categorization
    - `compute.rs` - Hash computation logic
    - `ui.rs` - User interface rendering
    - `mod.rs` - Module organization

### Changed
- **GUI Architecture**: Complete refactoring into modular components
- **User Experience**: Enhanced visual feedback and performance indicators
- **Code Organization**: Improved separation of concerns between CLI and GUI
- **Algorithm Display**: Added emoji indicators and security warnings
- **File Processing**: Optimized streaming for large files

### Improved
- **Performance**: Faster file processing with progress tracking
- **Security**: Constant-time comparisons for hash verification
- **Code Quality**: Zero clippy warnings, perfect rustfmt formatting
- **Memory Usage**: Reduced memory footprint for large file operations
- **Error Handling**: Enhanced error messages and user feedback

### Fixed
- SHAKE output handling for consistent results
- GUI threading issues during computation
- File processing edge cases for very large files
- Memory cleanup for sensitive data

### Documentation
- Added comprehensive technical documentation
- Created usage examples for all new features
- Updated web documentation with new algorithms
- Added security best practices guide

### Dependencies
- Added `sha3` v0.10.8 for SHA-3 and SHAKE functions
- Added `xxhash-rust` v0.8.15 for xxHash implementations
- Updated existing dependencies for compatibility

## [1.0.0] - 2024-12-01

### Added
- Initial release with core functionality
- Support for basic hash algorithms (SHA-1, SHA-256, SHA-512, BLAKE2, BLAKE3, MD5)
- Password hash algorithms (Argon2, bcrypt, scrypt)
- CLI interface with text and file processing
- Basic GUI interface
- Hash verification functionality
- Multiple output formats (hex, base64)

### Features
- Cross-platform support (macOS, Linux, Windows)
- Progress tracking for large files
- Batch processing capabilities
- Professional CLI with clap argument parsing
- Real-time GUI updates

---

## Release Notes

### v1.1.0 Highlights

This major update transforms Hash Forge into a comprehensive cryptographic tool with modern algorithms and an enhanced user experience. The addition of SHA-3, SHAKE, and xxHash algorithms provides users with cutting-edge cryptographic options, while HMAC support enables message authentication use cases.

The GUI has been completely redesigned with a modern dark theme, real-time computation, and intuitive visual feedback. The modular architecture ensures maintainability and extensibility for future enhancements.

### Breaking Changes
None. This release is fully backward compatible with v1.0.0.

### Migration Guide
No migration steps required. All existing CLI commands and GUI functionality continue to work as before, with new features available immediately.

### Security Notes
- All new algorithms use secure, well-tested implementations
- HMAC verification uses constant-time comparisons to prevent timing attacks
- Secure random salt generation for password hashing
- Clear warnings for deprecated algorithms (MD5, SHA-1)

### Performance Impact
- Overall performance improvement, especially for large files
- New xxHash algorithms provide significant speed improvements for non-cryptographic use cases
- GUI responsiveness enhanced with optimized computation threading
- Reduced memory usage for file processing operations
