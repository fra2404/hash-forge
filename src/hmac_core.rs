use crate::algorithms::HashAlgorithm;
use crate::output::HashResult;
use anyhow::{Context, Result};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

/// HMAC implementation for supported hash algorithms
pub struct HmacProcessor;

impl HmacProcessor {
    /// Compute HMAC with the specified algorithm and key
    pub fn compute_hmac(data: &[u8], key: &[u8], algorithm: HashAlgorithm) -> Result<HashResult> {
        match algorithm {
            HashAlgorithm::Sha1 => {
                let mut mac = Hmac::<Sha1>::new_from_slice(key)
                    .context("Invalid key length for HMAC-SHA1")?;
                mac.update(data);
                let result = mac.finalize().into_bytes();
                Ok(HashResult::new(result.to_vec(), algorithm))
            }
            HashAlgorithm::Sha256 => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key)
                    .context("Invalid key length for HMAC-SHA256")?;
                mac.update(data);
                let result = mac.finalize().into_bytes();
                Ok(HashResult::new(result.to_vec(), algorithm))
            }
            HashAlgorithm::Sha512 => {
                let mut mac = Hmac::<Sha512>::new_from_slice(key)
                    .context("Invalid key length for HMAC-SHA512")?;
                mac.update(data);
                let result = mac.finalize().into_bytes();
                Ok(HashResult::new(result.to_vec(), algorithm))
            }
            HashAlgorithm::Sha3_224 => {
                let mut mac = Hmac::<Sha3_224>::new_from_slice(key)
                    .context("Invalid key length for HMAC-SHA3-224")?;
                mac.update(data);
                let result = mac.finalize().into_bytes();
                Ok(HashResult::new(result.to_vec(), algorithm))
            }
            HashAlgorithm::Sha3_256 => {
                let mut mac = Hmac::<Sha3_256>::new_from_slice(key)
                    .context("Invalid key length for HMAC-SHA3-256")?;
                mac.update(data);
                let result = mac.finalize().into_bytes();
                Ok(HashResult::new(result.to_vec(), algorithm))
            }
            HashAlgorithm::Sha3_384 => {
                let mut mac = Hmac::<Sha3_384>::new_from_slice(key)
                    .context("Invalid key length for HMAC-SHA3-384")?;
                mac.update(data);
                let result = mac.finalize().into_bytes();
                Ok(HashResult::new(result.to_vec(), algorithm))
            }
            HashAlgorithm::Sha3_512 => {
                let mut mac = Hmac::<Sha3_512>::new_from_slice(key)
                    .context("Invalid key length for HMAC-SHA3-512")?;
                mac.update(data);
                let result = mac.finalize().into_bytes();
                Ok(HashResult::new(result.to_vec(), algorithm))
            }
            HashAlgorithm::Blake2b => {
                anyhow::bail!("HMAC-BLAKE2b not supported in current implementation")
            }
            HashAlgorithm::Blake2s => {
                anyhow::bail!("HMAC-BLAKE2s not supported in current implementation")
            }
            _ => anyhow::bail!("HMAC not supported for algorithm: {}", algorithm),
        }
    }

    /// Verify HMAC against expected value
    pub fn verify_hmac(
        data: &[u8],
        key: &[u8],
        expected_hmac: &[u8],
        algorithm: HashAlgorithm,
    ) -> Result<bool> {
        let computed = Self::compute_hmac(data, key, algorithm)?;

        // Constant-time comparison to prevent timing attacks
        Ok(constant_time_eq(&computed.bytes, expected_hmac))
    }

    /// Check if algorithm supports HMAC
    pub fn supports_hmac(algorithm: HashAlgorithm) -> bool {
        matches!(
            algorithm,
            HashAlgorithm::Sha1
                | HashAlgorithm::Sha256
                | HashAlgorithm::Sha512
                | HashAlgorithm::Sha3_224
                | HashAlgorithm::Sha3_256
                | HashAlgorithm::Sha3_384
                | HashAlgorithm::Sha3_512
        )
    }
}

/// Constant-time equality check to prevent timing attacks
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_sha256() {
        let data = b"Hello, World!";
        let key = b"secret_key";

        let result = HmacProcessor::compute_hmac(data, key, HashAlgorithm::Sha256).unwrap();
        assert_eq!(result.bytes.len(), 32); // SHA-256 HMAC output size
    }

    #[test]
    fn test_hmac_verification() {
        let data = b"test data";
        let key = b"test_key";

        let hmac_result = HmacProcessor::compute_hmac(data, key, HashAlgorithm::Sha256).unwrap();
        let is_valid =
            HmacProcessor::verify_hmac(data, key, &hmac_result.bytes, HashAlgorithm::Sha256)
                .unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hello!"));
    }
}
