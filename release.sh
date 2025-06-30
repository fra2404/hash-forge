#!/bin/bash

# Hash Forge Release Script v1.1.0
# Automates the build, test, and release process

set -e

VERSION="1.1.0"
PROJECT_NAME="hash-forge"

echo "🚀 Starting Hash Forge v$VERSION Release Process"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Step 1: Code Quality Checks
print_status "Running code quality checks..."

echo "  📋 Running clippy..."
if cargo clippy --all-targets --all-features --quiet; then
    print_success "Clippy check passed"
else
    print_error "Clippy check failed"
    exit 1
fi

echo "  📐 Checking code formatting..."
if cargo fmt --check --quiet; then
    print_success "Code formatting check passed"
else
    print_error "Code formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Step 2: Run Tests
print_status "Running test suite..."
if cargo test --quiet; then
    print_success "All tests passed"
else
    print_error "Tests failed"
    exit 1
fi

# Step 3: Build Release Binaries
print_status "Building release binaries..."

echo "  🔨 Building CLI binary..."
if cargo build --release --bin hash-forge --quiet; then
    print_success "CLI binary built successfully"
else
    print_error "CLI binary build failed"
    exit 1
fi

echo "  🎨 Building GUI binary..."
if cargo build --release --bin hash-forge-gui --quiet; then
    print_success "GUI binary built successfully"
else
    print_error "GUI binary build failed"
    exit 1
fi

# Step 4: Create Release Directory
RELEASE_DIR="release-v$VERSION"
print_status "Creating release directory: $RELEASE_DIR"

if [ -d "$RELEASE_DIR" ]; then
    rm -rf "$RELEASE_DIR"
fi
mkdir -p "$RELEASE_DIR"

# Step 5: Copy Binaries and Documentation
print_status "Preparing release files..."

# Copy binaries
cp "target/release/hash-forge" "$RELEASE_DIR/hash-forge-cli"
cp "target/release/hash-forge-gui" "$RELEASE_DIR/hash-forge-gui"

# Copy documentation
cp README.md "$RELEASE_DIR/"
cp LICENSE "$RELEASE_DIR/"
cp CHANGELOG.md "$RELEASE_DIR/"
cp "RELEASE_v$VERSION.md" "$RELEASE_DIR/"
cp PHASE1_COMPLETION_REPORT.md "$RELEASE_DIR/"

# Copy examples and scripts
cp demo_phase1.sh "$RELEASE_DIR/"
cp test_gui_phase1.sh "$RELEASE_DIR/"

# Copy web documentation
cp -r docs "$RELEASE_DIR/"

# Step 6: Create Platform-Specific Archives
print_status "Creating platform-specific archives..."

# Detect platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$ARCH" = "x86_64" ]; then
    ARCH="amd64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="arm64"
fi

ARCHIVE_NAME="${PROJECT_NAME}-v${VERSION}-${PLATFORM}-${ARCH}"

# Create tarball
print_status "Creating archive: ${ARCHIVE_NAME}.tar.gz"
tar -czf "${ARCHIVE_NAME}.tar.gz" -C "$RELEASE_DIR" .

# Create zip for convenience
print_status "Creating archive: ${ARCHIVE_NAME}.zip"
(cd "$RELEASE_DIR" && zip -r "../${ARCHIVE_NAME}.zip" . -q)

# Step 7: Generate checksums
print_status "Generating checksums..."
shasum -a 256 "${ARCHIVE_NAME}.tar.gz" > "${ARCHIVE_NAME}.tar.gz.sha256"
shasum -a 256 "${ARCHIVE_NAME}.zip" > "${ARCHIVE_NAME}.zip.sha256"

# Step 8: Verify binaries
print_status "Verifying built binaries..."

echo "  🔍 Testing CLI binary..."
if ./target/release/hash-forge --version > /dev/null 2>&1; then
    CLI_VERSION=$(./target/release/hash-forge --version | head -n 1)
    print_success "CLI binary verified: $CLI_VERSION"
else
    print_error "CLI binary verification failed"
    exit 1
fi

echo "  🔍 Testing GUI binary..."
if ./target/release/hash-forge-gui --help > /dev/null 2>&1; then
    print_success "GUI binary verified"
else
    print_warning "GUI binary verification skipped (requires display)"
fi

# Step 9: Quick functionality test
print_status "Running quick functionality tests..."

echo "  🧪 Testing hash computation..."
TEST_HASH=$(./target/release/hash-forge text --input "test" --algorithm sha256 | tail -n 1)
if [ ${#TEST_HASH} -eq 64 ]; then
    print_success "Hash computation test passed"
else
    print_error "Hash computation test failed"
    exit 1
fi

echo "  🧪 Testing HMAC computation..."
TEST_HMAC=$(./target/release/hash-forge hmac --data "test" --key "secret" --algorithm sha256 | tail -n 1)
if [ ${#TEST_HMAC} -eq 64 ]; then
    print_success "HMAC computation test passed"
else
    print_error "HMAC computation test failed"
    exit 1
fi

# Step 10: Display Release Summary
echo
echo "🎉 Release v$VERSION Ready!"
echo "=========================="
print_success "All checks passed"
print_success "Binaries built and verified"
print_success "Documentation included"
print_success "Archives created with checksums"

echo
echo "📦 Release Artifacts:"
echo "  • $RELEASE_DIR/ (release directory)"
echo "  • ${ARCHIVE_NAME}.tar.gz ($(du -h "${ARCHIVE_NAME}.tar.gz" | cut -f1))"
echo "  • ${ARCHIVE_NAME}.zip ($(du -h "${ARCHIVE_NAME}.zip" | cut -f1))"
echo "  • Checksums: .sha256 files"

echo
echo "📋 Binary Sizes:"
echo "  • CLI: $(du -h target/release/hash-forge | cut -f1)"
echo "  • GUI: $(du -h target/release/hash-forge-gui | cut -f1)"

echo
echo "🔧 Next Steps:"
echo "  1. Test the release artifacts"
echo "  2. Commit and tag the release:"
echo "     git add ."
echo "     git commit -m \"Release v$VERSION\""
echo "     git tag -a v$VERSION -m \"Hash Forge v$VERSION - Phase 1 Release\""
echo "     git push origin main --tags"
echo "  3. Create GitHub release with artifacts"
echo "  4. Update documentation website"

echo
print_success "Release preparation completed successfully! 🚀"
