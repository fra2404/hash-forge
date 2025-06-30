# 🔒 Security Information

## Code Signing and Trust

Hash Forge is an open-source project built with security and transparency in mind. Here's what you need to know about running Hash Forge safely on different platforms.

## 🍎 macOS Security

### Why the Warning Appears

When you first run Hash Forge on macOS, you may see this warning:
```
"Hash Forge" cannot be opened because Apple cannot check it for malicious software.
```

This happens because:
- Hash Forge is not distributed through the Mac App Store
- We use **ad-hoc code signing** (standard for open-source projects)
- Apple requires expensive developer certificates for "trusted" status

### ✅ How to Run Safely

#### Method 1: Right-Click to Open (Recommended)
1. **Right-click** the Hash Forge app
2. Select **"Open"** from the context menu
3. Click **"Open"** in the security dialog
4. The app will run and be trusted for future use

#### Method 2: System Preferences
1. Go to **System Preferences → Security & Privacy**
2. Click **"Open Anyway"** next to the Hash Forge warning
3. Enter your password if prompted

#### Method 3: Terminal Command
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine "/Applications/Hash Forge.app"
```

### 🔍 Verification

You can verify Hash Forge's code signature:

```bash
# Check code signature
codesign --verify --verbose "/Applications/Hash Forge.app"

# View signature details
codesign -dv "/Applications/Hash Forge.app"
```

## 🐧 Linux Security

Hash Forge binaries for Linux are:
- Built from public source code
- Compiled with security flags enabled
- Distributed as standard executables

Make the binary executable after download:
```bash
chmod +x hash-forge
chmod +x hash-forge-gui
```

## 🪟 Windows Security

Windows may show a "Windows protected your PC" warning. This is normal for unsigned applications.

**To run:**
1. Click **"More info"**
2. Click **"Run anyway"**

## 🔐 Hash Forge Security Features

### Cryptographic Implementation
- Uses well-audited Rust cryptographic libraries
- Implements constant-time hash comparisons
- Supports modern algorithms (BLAKE3, SHA-3, Argon2)
- Secure random salt generation

### Memory Safety
- Built with Rust for memory safety
- No buffer overflows or memory leaks
- Secure cleanup of sensitive data

### Algorithm Security
- **Recommended**: BLAKE3, SHA-256, SHA-3, Argon2
- **Legacy**: MD5, SHA-1 (included for compatibility, not recommended for security)
- Configurable iterations for password hashing

## 🔍 Source Code Verification

Hash Forge is completely open source:

1. **View the source**: https://github.com/fra2404/hash-forge
2. **Build from source**: Clone the repository and build with `cargo build`
3. **Audit dependencies**: Check `Cargo.toml` for all dependencies
4. **Review builds**: GitHub Actions workflows are public and auditable

## 📞 Reporting Security Issues

If you find a security vulnerability:

1. **DO NOT** open a public issue
2. Email: security@hashforge.dev (or create private security advisory)
3. Include detailed reproduction steps
4. We'll respond within 48 hours

## 🏆 Trust Indicators

- ✅ **Open Source**: Full source code available
- ✅ **Reproducible Builds**: Build scripts are public
- ✅ **Memory Safe**: Written in Rust
- ✅ **Well-Tested**: Comprehensive test suite
- ✅ **Standard Libraries**: Uses established crypto libraries
- ✅ **No Network**: Hash Forge never connects to the internet
- ✅ **No Telemetry**: No usage tracking or data collection

---

*Hash Forge prioritizes your security and privacy. All operations are performed locally on your device.*
