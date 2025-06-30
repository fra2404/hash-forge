#!/bin/bash

# Hash Forge Simple Release Script
# Quick release automation

set -e

VERSION="1.1.0"

echo "🚀 Hash Forge v$VERSION Release"
echo "==============================="

# Quick checks
echo "📋 Running quick checks..."
cargo clippy --all-targets --all-features --quiet || exit 1
cargo fmt --check --quiet || exit 1
cargo test --quiet || exit 1

# Build
echo "🔨 Building release binaries..."
cargo build --release --quiet

# Git operations
echo "📝 Git operations..."
git add .
git commit -m "Release v$VERSION - Phase 1 Complete

🚀 Major release with new algorithms and GUI enhancements

New Features:
- SHA-3 family (SHA3-224, SHA3-256, SHA3-384, SHA3-512)
- SHAKE functions (SHAKE128, SHAKE256)
- xxHash family (xxHash32, xxHash64, xxHash3)
- Complete HMAC support for all algorithms
- Modern GUI with dark theme and real-time computation
- Modular architecture with clean code organization

Technical Improvements:
- Zero clippy warnings, perfect rustfmt formatting
- Enhanced security with constant-time comparisons
- Optimized performance for large files
- Comprehensive documentation and clean project structure"

git tag -a "v$VERSION" -m "Hash Forge v$VERSION - Phase 1 Release"
git push origin main --tags

echo "✅ Release v$VERSION completed!"
echo "🔗 Create GitHub release: https://github.com/fra2404/hash-forge/releases/new?tag=v$VERSION"
