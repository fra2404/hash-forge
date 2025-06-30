# Release Process

This document describes how Hash Forge releases are created and distributed.

## Automated Release Process

Hash Forge uses GitHub Actions to automatically build and release binaries for macOS, Linux, and Windows when a new tag is created.

### Triggering a Release

1. **Create and push a new tag:**

   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **The GitHub Actions workflow will automatically:**
   - Build universal binaries for macOS (Intel + Apple Silicon)
   - Build binaries for Linux (x86_64)
   - Build binaries for Windows (x86_64)
   - Create a macOS `.dmg` installer with the app icon
   - Create archives for CLI tools
   - Upload all artifacts to GitHub Releases

### macOS DMG Creation

The macOS release process includes:

1. **Icon Conversion**: `image.png` → `AppIcon.icns`

   - Uses `sips` to create multiple icon sizes (16x16 to 1024x1024)
   - Uses `iconutil` to create the final `.icns` file

2. **App Bundle Creation**:

   - Creates `Hash Forge.app` with proper bundle structure
   - Includes both GUI and CLI binaries
   - Adds `Info.plist` with app metadata and icon reference

3. **DMG Creation**:
   - Uses `create-dmg` for professional-looking installer
   - Fallback to `hdiutil` if `create-dmg` fails
   - Includes drag-to-Applications shortcut

## Local Testing

To test the macOS DMG creation locally:

```bash
./scripts/build-macos-dmg.sh
# or use the full build script:
./build.sh
```

**Prerequisites:**

- Rust with macOS targets installed:
  ```bash
  rustup target add x86_64-apple-darwin aarch64-apple-darwin
  ```
- Icon file at `assets/image.png`
- (Optional) `create-dmg` for better DMG appearance:
  ```bash
  npm install -g create-dmg
  ```

**Build Output:**
All build artifacts are created in the `build/` directory:

- `build/Hash-Forge.dmg` - macOS installer
- `build/hash-forge-cli-macos.tar.gz` - CLI archive
- `build/Hash Forge.app` - macOS application bundle

## Release Assets

Each release includes:

| Asset                         | Platform | Description                 |
| ----------------------------- | -------- | --------------------------- |
| `Hash-Forge.dmg`              | macOS    | GUI app installer with icon |
| `hash-forge-cli-macos.tar.gz` | macOS    | CLI-only archive            |
| `hash-forge-linux.tar.gz`     | Linux    | GUI + CLI binaries          |
| `hash-forge-windows.zip`      | Windows  | GUI + CLI binaries          |

## Distribution

- **GitHub Releases**: Primary distribution method
- **GitHub Pages**: Project website with download links
- **Future**: Homebrew tap for macOS users

## Icon Requirements

The app icon (`assets/image.png`) should be:

- **Format**: PNG
- **Size**: 1024x1024 pixels (minimum 512x512)
- **Quality**: High resolution, as it will be scaled down
- **Design**: Clear and recognizable at small sizes
- **Location**: Must be placed in the `assets/` directory

The build process automatically creates all required icon sizes for macOS.

## Troubleshooting

### DMG Creation Fails

- Ensure `assets/image.png` exists in the project
- Check that all binaries were built successfully
- Try running the local build script to debug issues

### Icon Not Showing

- Verify `assets/image.png` is at least 512x512 pixels
- Check the `Info.plist` includes `CFBundleIconFile`
- Ensure `AppIcon.icns` is created in the app bundle

### Universal Binary Issues

- Verify both Intel and Apple Silicon targets are installed
- Check that `lipo` command succeeds in combining binaries
- Test on both Intel and Apple Silicon Macs if possible
