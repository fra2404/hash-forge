# Hash Forge - Usage Examples

This document provides comprehensive examples of using Hash Forge for various hashing and verification tasks.

## Basic Text Hashing

### SHA-256 (Default)

```bash
./target/release/hash-forge text --input "Hello, World!"
```

### Different Algorithms

```bash
# MD5 (legacy)
./target/release/hash-forge text --input "test" --algorithm md5

# SHA-1 (legacy)
./target/release/hash-forge text --input "test" --algorithm sha1

# SHA-512
./target/release/hash-forge text --input "test" --algorithm sha512

# BLAKE2b (modern, fast)
./target/release/hash-forge text --input "test" --algorithm blake2b

# BLAKE3 (latest, fastest)
./target/release/hash-forge text --input "test" --algorithm blake3
```

### Output Formats

```bash
# Hexadecimal (default)
./target/release/hash-forge text --input "test" --output-format hex

# Base64
./target/release/hash-forge text --input "test" --output-format base64
```

## Password Hashing

### bcrypt (Recommended for passwords)

```bash
./target/release/hash-forge text --input "mypassword" --algorithm bcrypt
```

### scrypt

```bash
./target/release/hash-forge text --input "mypassword" --algorithm scrypt
```

### Argon2 (Most secure, latest standard)

```bash
./target/release/hash-forge text --input "mypassword" --algorithm argon2
```

### With Custom Salt

```bash
./target/release/hash-forge text --input "mypassword" --algorithm bcrypt --salt "customsalt123"
```

## File Hashing

### Single File

```bash
./target/release/hash-forge file --path document.pdf --algorithm sha256
```

### Different Algorithms for Files

```bash
# For integrity checking
./target/release/hash-forge file --path largefile.zip --algorithm blake3

# For compatibility with legacy systems
./target/release/hash-forge file --path data.txt --algorithm md5
```

## Hash Verification

### Verify Text Hash

```bash
# First compute the hash
./target/release/hash-forge text --input "Hello, World!" --algorithm sha256

# Then verify it
./target/release/hash-forge verify --text "Hello, World!" --algorithm sha256 --expected-hash dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f
```

### Verify File Hash

```bash
# Verify a file against a known hash
./target/release/hash-forge verify --file document.pdf --algorithm sha256 --expected-hash a1b2c3d4e5f6...
```

### Verify with Base64 Hash

```bash
./target/release/hash-forge verify --text "test" --algorithm blake3 --expected-hash "base64encodedvalue="
```

## Batch Processing

### Process Directory

```bash
./target/release/hash-forge batch --directory ./documents --algorithm sha256
```

### With Different Output Format

```bash
./target/release/hash-forge batch --directory ./images --algorithm blake3 --output-format base64
```

## GUI Usage

### Start GUI Application

```bash
./target/release/hash-forge-gui
```

The GUI provides an intuitive interface with:

- Text and file input modes
- Algorithm selection dropdown
- Real-time hash computation
- Hash verification mode
- Copy-to-clipboard functionality
- Advanced options for password hashing

## Performance Tips

1. **For large files**: Use BLAKE3 for best performance
2. **For passwords**: Always use bcrypt, scrypt, or Argon2
3. **For compatibility**: SHA-256 is widely supported
4. **For modern applications**: BLAKE3 is recommended over SHA-2

## Security Recommendations

1. **Never use MD5 or SHA-1 for security purposes** - they're cryptographically broken
2. **Use proper password hashing algorithms** (bcrypt/scrypt/Argon2) for passwords
3. **For file integrity**: SHA-256 or BLAKE3 are excellent choices
4. **For digital signatures**: Use SHA-256 or higher

## Integration Examples

### Checksum Generation for Release Files

```bash
# Create checksums for all files in release directory
./target/release/hash-forge batch --directory ./release --algorithm sha256 > checksums.txt
```

### Automated File Integrity Checking

```bash
#!/bin/bash
# Script to verify file integrity
EXPECTED_HASH="a1b2c3d4..."
RESULT=$(./target/release/hash-forge verify --file important.dat --algorithm sha256 --expected-hash $EXPECTED_HASH)
if [ $? -eq 0 ]; then
    echo "File integrity verified ✅"
else
    echo "File integrity check failed ❌"
    exit 1
fi
```

### Password Verification

```bash
# Note: For real password systems, store only the hash, never the password
USER_PASSWORD="userInput123"
STORED_HASH="stored_bcrypt_hash"
./target/release/hash-forge verify --text "$USER_PASSWORD" --algorithm bcrypt --expected-hash "$STORED_HASH"
```
