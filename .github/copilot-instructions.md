# Hash Forge - Copilot Instructions

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

## Project Overview

Hash Forge is a professional hash generator and verifier built with Rust, featuring both CLI and GUI interfaces. The project supports multiple cryptographic algorithms and is designed for cross-platform compatibility.

## Architecture Guidelines

- **Dual Binary Structure**: Two separate binaries (`hash-forge` for CLI, `hash-forge-gui` for GUI)
- **Shared Core Logic**: Common hash algorithms and utilities in shared modules
- **Clean Separation**: CLI logic in `main.rs`, GUI logic in `gui_main.rs`, shared code in modules
- **Error Handling**: Use `anyhow` for application errors, `thiserror` for library errors

## Code Style Preferences

- **Rust Idioms**: Prefer explicit error handling, use `?` operator appropriately
- **Documentation**: Document all public functions and modules with examples
- **Testing**: Include unit tests for hash algorithms and integration tests for CLI
- **Performance**: Optimize for both memory usage and speed, especially for large files

## Dependencies Usage

- **Cryptography**: Use well-established crates (`sha2`, `blake3`, `argon2`)
- **CLI**: Use `clap` with derive macros for clean argument parsing
- **GUI**: Use `egui`/`eframe` for immediate mode GUI with real-time updates
- **File I/O**: Use `indicatif` for progress bars during large file processing

## Security Considerations

- **Constant Time**: Use constant-time comparisons for hash verification
- **Memory Safety**: Ensure sensitive data is properly cleared from memory
- **Algorithm Choice**: Recommend modern algorithms (BLAKE3, Argon2) over legacy ones
- **Salt Handling**: Generate cryptographically secure random salts

## Feature Implementation

- **Hash Algorithms**: Support both fast hashes (for files) and slow hashes (for passwords)
- **Input Methods**: Support text input, file input, and batch processing
- **Output Formats**: Hex, Base64, and raw binary output options
- **Verification**: Allow hash verification against expected values
- **Progress Tracking**: Show progress for large file operations

## GUI Design Principles

- **Real-time Updates**: Update hash results as user types (for text input)
- **File Drag & Drop**: Support dropping files directly into the GUI
- **Copy/Paste**: Easy copying of hash results to clipboard
- **Algorithm Comparison**: Show multiple algorithm results simultaneously
- **Visual Feedback**: Clear success/failure indicators for verification

## Testing Strategy

- **Unit Tests**: Test each hash algorithm implementation
- **Integration Tests**: Test CLI commands and GUI interactions
- **Property Tests**: Use property-based testing for hash properties
- **Benchmark Tests**: Performance tests for different input sizes
- **Cross-Platform**: Ensure tests pass on macOS, Linux, and Windows
