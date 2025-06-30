use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum HashAlgorithm {
    // Fast cryptographic hashes (for files)
    #[value(name = "md5")]
    Md5,
    #[value(name = "sha1")]
    Sha1,
    #[default]
    #[value(name = "sha256")]
    Sha256,
    #[value(name = "sha512")]
    Sha512,

    // SHA-3 family (Keccak-based)
    #[value(name = "sha3-224")]
    Sha3_224,
    #[value(name = "sha3-256")]
    Sha3_256,
    #[value(name = "sha3-384")]
    Sha3_384,
    #[value(name = "sha3-512")]
    Sha3_512,

    // SHAKE functions (extendable output)
    #[value(name = "shake128")]
    Shake128,
    #[value(name = "shake256")]
    Shake256,

    // BLAKE family
    #[value(name = "blake2b")]
    Blake2b,
    #[value(name = "blake2s")]
    Blake2s,
    #[value(name = "blake3")]
    Blake3,

    // High performance non-cryptographic hashes
    #[value(name = "xxh32")]
    XxHash32,
    #[value(name = "xxh64")]
    XxHash64,
    #[value(name = "xxh3")]
    XxHash3,

    // Slow password hashing algorithms
    #[value(name = "bcrypt")]
    Bcrypt,
    #[value(name = "scrypt")]
    Scrypt,
    #[value(name = "argon2")]
    Argon2,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Md5 => write!(f, "MD5"),
            HashAlgorithm::Sha1 => write!(f, "SHA-1"),
            HashAlgorithm::Sha256 => write!(f, "SHA-256"),
            HashAlgorithm::Sha512 => write!(f, "SHA-512"),
            HashAlgorithm::Sha3_224 => write!(f, "SHA3-224"),
            HashAlgorithm::Sha3_256 => write!(f, "SHA3-256"),
            HashAlgorithm::Sha3_384 => write!(f, "SHA3-384"),
            HashAlgorithm::Sha3_512 => write!(f, "SHA3-512"),
            HashAlgorithm::Shake128 => write!(f, "SHAKE128"),
            HashAlgorithm::Shake256 => write!(f, "SHAKE256"),
            HashAlgorithm::Blake2b => write!(f, "BLAKE2b"),
            HashAlgorithm::Blake2s => write!(f, "BLAKE2s"),
            HashAlgorithm::Blake3 => write!(f, "BLAKE3"),
            HashAlgorithm::XxHash32 => write!(f, "xxHash32"),
            HashAlgorithm::XxHash64 => write!(f, "xxHash64"),
            HashAlgorithm::XxHash3 => write!(f, "xxHash3"),
            HashAlgorithm::Bcrypt => write!(f, "bcrypt"),
            HashAlgorithm::Scrypt => write!(f, "scrypt"),
            HashAlgorithm::Argon2 => write!(f, "Argon2"),
        }
    }
}

impl HashAlgorithm {
    /// Returns true if this algorithm is designed for password hashing (slow)
    pub fn is_password_hash(&self) -> bool {
        matches!(
            self,
            HashAlgorithm::Bcrypt | HashAlgorithm::Scrypt | HashAlgorithm::Argon2
        )
    }

    /// Returns true if this algorithm is fast and suitable for file hashing
    pub fn is_fast_hash(&self) -> bool {
        !self.is_password_hash()
    }

    /// Returns the recommended use case for this algorithm
    pub fn recommended_use(&self) -> &'static str {
        match self {
            HashAlgorithm::Md5 => "Legacy compatibility (not recommended for security)",
            HashAlgorithm::Sha1 => "Legacy compatibility (not recommended for security)",
            HashAlgorithm::Sha256 => "General purpose, file integrity, digital signatures",
            HashAlgorithm::Sha512 => "High security requirements, large data",
            HashAlgorithm::Sha3_224 => "SHA-3 standard, compact 224-bit output",
            HashAlgorithm::Sha3_256 => "SHA-3 standard, modern alternative to SHA-256",
            HashAlgorithm::Sha3_384 => "SHA-3 standard, medium security level",
            HashAlgorithm::Sha3_512 => "SHA-3 standard, highest security level",
            HashAlgorithm::Shake128 => "Extendable output function, customizable length",
            HashAlgorithm::Shake256 => "Extendable output function, higher security",
            HashAlgorithm::Blake2b => "High performance alternative to SHA-512",
            HashAlgorithm::Blake2s => "High performance alternative to SHA-256",
            HashAlgorithm::Blake3 => "Modern, fastest cryptographic hash",
            HashAlgorithm::XxHash32 => "Ultra-fast non-cryptographic hash, 32-bit",
            HashAlgorithm::XxHash64 => "Ultra-fast non-cryptographic hash, 64-bit",
            HashAlgorithm::XxHash3 => "Ultra-fast non-cryptographic hash, modern",
            HashAlgorithm::Bcrypt => "Password hashing (moderate security)",
            HashAlgorithm::Scrypt => "Password hashing (high memory usage)",
            HashAlgorithm::Argon2 => "Password hashing (recommended, modern)",
        }
    }

    /// Returns the typical output size in bytes
    pub fn output_size(&self) -> usize {
        match self {
            HashAlgorithm::Md5 => 16,
            HashAlgorithm::Sha1 => 20,
            HashAlgorithm::Sha256 | HashAlgorithm::Blake2s => 32,
            HashAlgorithm::Sha512 | HashAlgorithm::Blake2b => 64,
            HashAlgorithm::Sha3_224 => 28,
            HashAlgorithm::Sha3_256 => 32,
            HashAlgorithm::Sha3_384 => 48,
            HashAlgorithm::Sha3_512 => 64,
            HashAlgorithm::Shake128 => 32, // Default size, configurable
            HashAlgorithm::Shake256 => 32, // Default size, configurable
            HashAlgorithm::Blake3 => 32,   // Default size, configurable
            HashAlgorithm::XxHash32 => 4,
            HashAlgorithm::XxHash64 => 8,
            HashAlgorithm::XxHash3 => 8, // Default for XXH3_64
            HashAlgorithm::Bcrypt => 60, // bcrypt string format
            HashAlgorithm::Scrypt => 32,
            HashAlgorithm::Argon2 => 32,
        }
    }

    /// Returns true if this algorithm supports configurable output length
    pub fn is_extendable(&self) -> bool {
        matches!(
            self,
            HashAlgorithm::Shake128 | HashAlgorithm::Shake256 | HashAlgorithm::Blake3
        )
    }

    /// Returns true if this algorithm is cryptographically secure
    pub fn is_cryptographic(&self) -> bool {
        !matches!(
            self,
            HashAlgorithm::XxHash32 | HashAlgorithm::XxHash64 | HashAlgorithm::XxHash3
        )
    }
}
