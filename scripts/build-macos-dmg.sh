#!/bin/bash

# Build macOS DMG locally for testing
# This script mimics what the GitHub Actions workflow does

set -e

echo "🛠️  Building Hash Forge for macOS..."

# Check if image.png exists
if [ ! -f "assets/image.png" ]; then
    echo "❌ Error: assets/image.png not found!"
    echo "   Please place your app icon at assets/image.png"
    exit 1
fi

# Create build directory
BUILD_DIR="build"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"
echo "📁 Using build directory: $BUILD_DIR"

# Build binaries
echo "📦 Building binaries..."
cargo build --release --target x86_64-apple-darwin --bin hash-forge
cargo build --release --target x86_64-apple-darwin --bin hash-forge-gui
cargo build --release --target aarch64-apple-darwin --bin hash-forge
cargo build --release --target aarch64-apple-darwin --bin hash-forge-gui

# Create universal binaries
echo "🔗 Creating universal binaries..."
mkdir -p target/universal-apple-darwin/release
lipo -create -output target/universal-apple-darwin/release/hash-forge \
  target/x86_64-apple-darwin/release/hash-forge \
  target/aarch64-apple-darwin/release/hash-forge
lipo -create -output target/universal-apple-darwin/release/hash-forge-gui \
  target/x86_64-apple-darwin/release/hash-forge-gui \
  target/aarch64-apple-darwin/release/hash-forge-gui

# Create app bundle
echo "📱 Creating app bundle..."
APP_BUNDLE="$BUILD_DIR/Hash Forge.app"
rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Copy binaries
cp target/universal-apple-darwin/release/hash-forge-gui "$APP_BUNDLE/Contents/MacOS/Hash Forge"
cp target/universal-apple-darwin/release/hash-forge "$APP_BUNDLE/Contents/Resources/"

# Convert PNG to ICNS
echo "🎨 Converting icon..."
ICON_SET="$BUILD_DIR/icon.iconset"
rm -rf "$ICON_SET"
mkdir -p "$ICON_SET"
sips -z 16 16 assets/image.png --out "$ICON_SET/icon_16x16.png"
sips -z 32 32 assets/image.png --out "$ICON_SET/icon_16x16@2x.png"
sips -z 32 32 assets/image.png --out "$ICON_SET/icon_32x32.png"
sips -z 64 64 assets/image.png --out "$ICON_SET/icon_32x32@2x.png"
sips -z 128 128 assets/image.png --out "$ICON_SET/icon_128x128.png"
sips -z 256 256 assets/image.png --out "$ICON_SET/icon_128x128@2x.png"
sips -z 256 256 assets/image.png --out "$ICON_SET/icon_256x256.png"
sips -z 512 512 assets/image.png --out "$ICON_SET/icon_256x256@2x.png"
sips -z 512 512 assets/image.png --out "$ICON_SET/icon_512x512.png"
sips -z 1024 1024 assets/image.png --out "$ICON_SET/icon_512x512@2x.png"

iconutil -c icns "$ICON_SET" -o "$APP_BUNDLE/Contents/Resources/AppIcon.icns"

# Create Info.plist
echo "📄 Creating Info.plist..."
cat > "$APP_BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>Hash Forge</string>
    <key>CFBundleIdentifier</key>
    <string>com.hashforge.app</string>
    <key>CFBundleName</key>
    <string>Hash Forge</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>CFBundleDisplayName</key>
    <string>Hash Forge</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# Make binaries executable
chmod +x "$APP_BUNDLE/Contents/MacOS/Hash Forge"
chmod +x "$APP_BUNDLE/Contents/Resources/hash-forge"

# Create DMG
echo "💿 Creating DMG..."
DMG_CONTENTS="$BUILD_DIR/dmg-contents"
DMG_FILE="$BUILD_DIR/Hash-Forge.dmg"

rm -rf "$DMG_CONTENTS" "$DMG_FILE"
mkdir -p "$DMG_CONTENTS"
cp -R "$APP_BUNDLE" "$DMG_CONTENTS/"
ln -s /Applications "$DMG_CONTENTS/Applications"

# Check if create-dmg is available
if command -v create-dmg &> /dev/null; then
    echo "📦 Using create-dmg..."
    create-dmg \
      --volname "Hash Forge" \
      --volicon "$APP_BUNDLE/Contents/Resources/AppIcon.icns" \
      --window-pos 200 120 \
      --window-size 800 400 \
      --icon-size 100 \
      --icon "Hash Forge.app" 200 190 \
      --hide-extension "Hash Forge.app" \
      --app-drop-link 600 185 \
      --hdiutil-quiet \
      "$DMG_FILE" \
      "$DMG_CONTENTS/" || {
        echo "⚠️  create-dmg failed, falling back to hdiutil..."
        hdiutil create -volname "Hash Forge" -srcfolder "$DMG_CONTENTS" -ov -format UDZO "$DMG_FILE"
    }
else
    echo "📦 Using hdiutil (install create-dmg with: npm install -g create-dmg)..."
    hdiutil create -volname "Hash Forge" -srcfolder "$DMG_CONTENTS" -ov -format UDZO "$DMG_FILE"
fi

echo ""
echo "✅ DMG created successfully: $DMG_FILE"
echo "🎯 You can now test the app by opening the DMG file"
echo ""

# Clean up
rm -rf "$ICON_SET" "$DMG_CONTENTS"

echo "🧹 Cleaned up temporary files"
