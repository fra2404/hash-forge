#!/bin/bash

# Hash Forge - Release Readiness Check
# This script verifies that all components are ready for release

set -e

echo "🔧 Hash Forge - Release Readiness Check"
echo "========================================"
echo

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Not in Hash Forge project directory"
    exit 1
fi

echo "📂 Project Structure Check"
echo "--------------------------"

# Check essential files
files=(
    "assets/image.png"
    ".github/workflows/release.yml"
    "scripts/build-macos-dmg.sh"
    "docs/index.html"
    "src/main.rs"
    "src/gui_main.rs"
    "README.md"
    "RELEASE.md"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file"
    else
        echo "❌ $file (MISSING)"
        exit 1
    fi
done

echo
echo "🦀 Rust Environment Check"
echo "-------------------------"

# Check Rust installation
if command -v rustc &> /dev/null; then
    echo "✅ Rust installed: $(rustc --version)"
else
    echo "❌ Rust not installed"
    exit 1
fi

# Check required targets
targets=("x86_64-apple-darwin" "aarch64-apple-darwin")
for target in "${targets[@]}"; do
    if rustup target list --installed | grep -q "$target"; then
        echo "✅ Target: $target"
    else
        echo "❌ Target missing: $target"
        echo "   Install with: rustup target add $target"
        exit 1
    fi
done

echo
echo "🛠️ Build Check"
echo "---------------"

# Check if project compiles
if cargo check --quiet; then
    echo "✅ Project compiles successfully"
else
    echo "❌ Project compilation failed"
    exit 1
fi

echo
echo "🎨 Icon Check"
echo "-------------"

# Check icon file
if [ -f "assets/image.png" ]; then
    size=$(file assets/image.png | grep -o '[0-9]* x [0-9]*' | head -1)
    if [ -n "$size" ]; then
        echo "✅ Icon file found: $size"
    else
        echo "⚠️  Icon file found but size unknown"
    fi
else
    echo "❌ Icon file missing"
    exit 1
fi

echo
echo "⚙️ Tools Check"
echo "--------------"

# Check for create-dmg
if command -v create-dmg &> /dev/null; then
    echo "✅ create-dmg available"
else
    echo "⚠️  create-dmg not found (will use hdiutil fallback)"
fi

# Check for sips and iconutil (macOS only)
if command -v sips &> /dev/null && command -v iconutil &> /dev/null; then
    echo "✅ macOS icon tools available"
else
    echo "⚠️  macOS icon tools not available (needed for local testing)"
fi

echo
echo "📄 Documentation Check"
echo "----------------------"

# Check key documentation sections
if grep -q "Release" README.md; then
    echo "✅ README mentions releases"
else
    echo "⚠️  README should mention release process"
fi

if [ -f "RELEASE.md" ]; then
    echo "✅ Release documentation present"
else
    echo "❌ Release documentation missing"
    exit 1
fi

echo
echo "🚀 GitHub Actions Check"
echo "-----------------------"

# Check workflow file
workflow=".github/workflows/release.yml"
if [ -f "$workflow" ]; then
    echo "✅ Release workflow exists"
    
    # Check for key workflow components
    if grep -q "assets/image.png" "$workflow"; then
        echo "✅ Workflow references icon file"
    else
        echo "❌ Workflow doesn't reference icon file"
        exit 1
    fi
    
    if grep -q "AppIcon.icns" "$workflow"; then
        echo "✅ Workflow creates app icon"
    else
        echo "❌ Workflow doesn't create app icon"
        exit 1
    fi
    
    if grep -q "create-dmg\|hdiutil" "$workflow"; then
        echo "✅ Workflow creates DMG"
    else
        echo "❌ Workflow doesn't create DMG"
        exit 1
    fi
else
    echo "❌ Release workflow missing"
    exit 1
fi

echo
echo "🌐 Website Check"
echo "---------------"

if [ -f "docs/index.html" ]; then
    echo "✅ GitHub Pages site ready"
    if grep -q "Hash Forge" "docs/index.html"; then
        echo "✅ Website contains project name"
    else
        echo "⚠️  Website should mention project name"
    fi
else
    echo "❌ GitHub Pages site missing"
    exit 1
fi

echo
echo "🎉 RELEASE READINESS: COMPLETE!"
echo "==============================="
echo
echo "✅ All checks passed!"
echo "✅ Project is ready for release"
echo "✅ GitHub Actions will automatically:"
echo "   • Build universal macOS binaries"
echo "   • Convert image.png to AppIcon.icns"
echo "   • Create professional macOS DMG"
echo "   • Build Linux and Windows binaries"
echo "   • Upload all assets to GitHub Releases"
echo
echo "🚀 To create a release:"
echo "   git tag v1.0.0"
echo "   git push origin v1.0.0"
echo
echo "Happy releasing! 🎊"
