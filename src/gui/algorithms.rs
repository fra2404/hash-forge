// Algorithm filtering and categorization logic
use super::app_state::{AlgorithmCategory, HashForgeApp};
use crate::algorithms::HashAlgorithm;
use crate::output::OutputFormat;

// All available algorithms including Phase 1 additions
pub const ALL_ALGORITHMS: [HashAlgorithm; 19] = [
    // Fast hash algorithms
    HashAlgorithm::Blake3,
    HashAlgorithm::Blake2b,
    HashAlgorithm::Blake2s,
    HashAlgorithm::Sha256,
    HashAlgorithm::Sha512,
    HashAlgorithm::Sha3_224,
    HashAlgorithm::Sha3_256,
    HashAlgorithm::Sha3_384,
    HashAlgorithm::Sha3_512,
    HashAlgorithm::Shake128,
    HashAlgorithm::Shake256,
    HashAlgorithm::XxHash32,
    HashAlgorithm::XxHash64,
    HashAlgorithm::XxHash3,
    HashAlgorithm::Sha1,
    HashAlgorithm::Md5,
    // Password hash algorithms
    HashAlgorithm::Argon2,
    HashAlgorithm::Bcrypt,
    HashAlgorithm::Scrypt,
];

pub const ALL_CATEGORIES: [AlgorithmCategory; 5] = [
    AlgorithmCategory::All,
    AlgorithmCategory::Modern,
    AlgorithmCategory::FastHash,
    AlgorithmCategory::PasswordHash,
    AlgorithmCategory::Legacy,
];

pub const AVAILABLE_OUTPUT_FORMATS: [OutputFormat; 2] = [OutputFormat::Hex, OutputFormat::Base64];

impl HashForgeApp {
    /// Get filtered algorithms based on current category
    pub fn filtered_algorithms(&self) -> Vec<HashAlgorithm> {
        ALL_ALGORITHMS
            .iter()
            .copied()
            .filter(|&alg| self.matches_category(alg))
            .collect()
    }

    /// Check if algorithm matches current category filter
    fn matches_category(&self, algorithm: HashAlgorithm) -> bool {
        match self.algorithm_category {
            AlgorithmCategory::All => true,
            AlgorithmCategory::FastHash => !algorithm.is_password_hash(),
            AlgorithmCategory::PasswordHash => algorithm.is_password_hash(),
            AlgorithmCategory::Modern => matches!(
                algorithm,
                HashAlgorithm::Blake3
                    | HashAlgorithm::Blake2b
                    | HashAlgorithm::Blake2s
                    | HashAlgorithm::Sha3_224
                    | HashAlgorithm::Sha3_256
                    | HashAlgorithm::Sha3_384
                    | HashAlgorithm::Sha3_512
                    | HashAlgorithm::Shake128
                    | HashAlgorithm::Shake256
                    | HashAlgorithm::XxHash3
                    | HashAlgorithm::Argon2
            ),
            AlgorithmCategory::Legacy => matches!(
                algorithm,
                HashAlgorithm::Md5
                    | HashAlgorithm::Sha1
                    | HashAlgorithm::XxHash32
                    | HashAlgorithm::XxHash64
            ),
        }
    }
}
