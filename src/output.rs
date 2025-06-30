use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    #[value(name = "hex")]
    Hex,
    #[value(name = "base64")]
    Base64,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Hex => write!(f, "hex"),
            OutputFormat::Base64 => write!(f, "base64"),
        }
    }
}

/// Represents a hash result that can be formatted in different ways
#[derive(Debug, Clone)]
pub struct HashResult {
    pub bytes: Vec<u8>,
    pub algorithm: crate::algorithms::HashAlgorithm,
}

impl HashResult {
    pub fn new(bytes: Vec<u8>, algorithm: crate::algorithms::HashAlgorithm) -> Self {
        Self { bytes, algorithm }
    }
    
    /// Convert hash to hexadecimal string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.bytes)
    }
    
    /// Convert hash to base64 string
    pub fn to_base64(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(&self.bytes)
    }
    
    /// Get raw bytes
    pub fn to_bytes(&self) -> &[u8] {
        &self.bytes
    }
    
    /// Compare with another hash result in constant time
    pub fn constant_time_eq(&self, other: &[u8]) -> bool {
        if self.bytes.len() != other.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (a, b) in self.bytes.iter().zip(other.iter()) {
            result |= a ^ b;
        }
        result == 0
    }
}

impl fmt::Display for HashResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
