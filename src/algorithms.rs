use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum HashAlgorithm {
    // Fast cryptographic hashes (for files)
    #[value(name = "md5")]
    Md5,
    #[value(name = "sha1")]
    Sha1,
    #[value(name = "sha256")]
    Sha256,
    #[value(name = "sha512")]
    Sha512,
    #[value(name = "blake2b")]
    Blake2b,
    #[value(name = "blake2s")]
    Blake2s,
    #[value(name = "blake3")]
    Blake3,
    
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
            HashAlgorithm::Blake2b => write!(f, "BLAKE2b"),
            HashAlgorithm::Blake2s => write!(f, "BLAKE2s"),
            HashAlgorithm::Blake3 => write!(f, "BLAKE3"),
            HashAlgorithm::Bcrypt => write!(f, "bcrypt"),
            HashAlgorithm::Scrypt => write!(f, "scrypt"),
            HashAlgorithm::Argon2 => write!(f, "Argon2"),
        }
    }
}

impl HashAlgorithm {
    /// Returns true if this algorithm is designed for password hashing (slow)
    pub fn is_password_hash(&self) -> bool {
        matches!(self, HashAlgorithm::Bcrypt | HashAlgorithm::Scrypt | HashAlgorithm::Argon2)
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
            HashAlgorithm::Blake2b => "High performance alternative to SHA-512",
            HashAlgorithm::Blake2s => "High performance alternative to SHA-256",
            HashAlgorithm::Blake3 => "Modern, fastest cryptographic hash",
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
            HashAlgorithm::Blake3 => 32, // Default size, configurable
            HashAlgorithm::Bcrypt => 60, // bcrypt string format
            HashAlgorithm::Scrypt => 32,
            HashAlgorithm::Argon2 => 32,
        }
    }
}
