use crate::{
    algorithms::HashAlgorithm,
    output::{HashResult, OutputFormat},
    utils::generate_salt,
};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs,
    io::{BufReader, Read},
    path::Path,
};

use blake2::{Blake2b512, Blake2s256, Digest as Blake2Digest};
use digest::Digest;
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha256, Sha512};

pub struct HashForge {
    progress_enabled: bool,
}

impl HashForge {
    pub fn new() -> Self {
        Self {
            progress_enabled: true,
        }
    }

    pub fn with_progress(mut self, enabled: bool) -> Self {
        self.progress_enabled = enabled;
        self
    }

    /// Hash text input with the specified algorithm
    pub fn hash_text(
        &self,
        text: &str,
        algorithm: HashAlgorithm,
        salt: Option<&str>,
        _iterations: Option<u32>,
    ) -> Result<HashResult> {
        let bytes = text.as_bytes();

        match algorithm {
            // Fast hash algorithms
            HashAlgorithm::Md5 => {
                let hash = md5::compute(bytes);
                Ok(HashResult::new(hash.as_ref().to_vec(), algorithm))
            }
            HashAlgorithm::Sha1 => {
                let mut hasher = Sha1::new();
                Sha1Digest::update(&mut hasher, bytes);
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                Sha2Digest::update(&mut hasher, bytes);
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                Sha2Digest::update(&mut hasher, bytes);
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Blake2b => {
                let mut hasher = Blake2b512::new();
                Blake2Digest::update(&mut hasher, bytes);
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Blake2s => {
                let mut hasher = Blake2s256::new();
                Blake2Digest::update(&mut hasher, bytes);
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Blake3 => {
                let hash = blake3::hash(bytes);
                Ok(HashResult::new(hash.as_bytes().to_vec(), algorithm))
            }

            // Password hash algorithms
            HashAlgorithm::Bcrypt => {
                let cost = 12; // Default cost
                let default_salt = generate_salt(16);
                let salt = salt.unwrap_or(&default_salt);

                // bcrypt expects a 16-byte salt
                let mut salt_bytes = [0u8; 16];
                let salt_slice = salt.as_bytes();
                let copy_len = std::cmp::min(salt_slice.len(), 16);
                salt_bytes[..copy_len].copy_from_slice(&salt_slice[..copy_len]);

                let hash = bcrypt::hash_with_salt(text, cost, salt_bytes)
                    .context("Failed to compute bcrypt hash")?;
                Ok(HashResult::new(hash.to_string().into_bytes(), algorithm))
            }
            HashAlgorithm::Scrypt => {
                let params =
                    scrypt::Params::new(14, 8, 1, 32).context("Invalid scrypt parameters")?;
                let salt = salt.map(|s| s.as_bytes()).unwrap_or(b"defaultsalt");
                let mut output = [0u8; 32];
                scrypt::scrypt(bytes, salt, &params, &mut output)
                    .context("Failed to compute scrypt hash")?;
                Ok(HashResult::new(output.to_vec(), algorithm))
            }
            HashAlgorithm::Argon2 => {
                let default_salt = generate_salt(16);
                let salt = salt.unwrap_or(&default_salt);
                // Using the new argon2 API (v0.5+)
                let salt_bytes = salt.as_bytes();
                let argon2 = argon2::Argon2::default();
                let mut output = [0u8; 32];
                argon2
                    .hash_password_into(bytes, salt_bytes, &mut output)
                    .map_err(|e| anyhow::anyhow!("Failed to compute Argon2 hash: {}", e))?;
                Ok(HashResult::new(output.to_vec(), algorithm))
            }
        }
    }

    /// Hash file contents with the specified algorithm
    pub fn hash_file(&self, path: &Path, algorithm: HashAlgorithm) -> Result<HashResult> {
        let file = fs::File::open(path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;

        let file_size = file.metadata()?.len();
        let mut reader = BufReader::new(file);

        let progress_bar = if self.progress_enabled && file_size > 1024 * 1024 {
            let pb = ProgressBar::new(file_size);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .unwrap()
                    .progress_chars("#>-"),
            );
            Some(pb)
        } else {
            None
        };

        match algorithm {
            HashAlgorithm::Md5 => {
                let mut hasher = md5::Context::new();
                // Note: md5 doesn't implement the generic Digest trait properly
                let mut buffer = [0; 8192];
                let mut total_read = 0u64;

                loop {
                    let bytes_read = reader.read(&mut buffer)?;
                    if bytes_read == 0 {
                        break;
                    }

                    hasher.consume(&buffer[..bytes_read]);
                    total_read += bytes_read as u64;

                    if let Some(pb) = &progress_bar {
                        pb.set_position(total_read);
                    }
                }

                if let Some(pb) = &progress_bar {
                    pb.finish_with_message("Hash computed");
                }

                Ok(HashResult::new(hasher.compute().0.to_vec(), algorithm))
            }
            HashAlgorithm::Sha1 => {
                let mut hasher = Sha1::new();
                self.update_hasher_with_progress(&mut hasher, &mut reader, &progress_bar)?;
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                self.update_hasher_with_progress(&mut hasher, &mut reader, &progress_bar)?;
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                self.update_hasher_with_progress(&mut hasher, &mut reader, &progress_bar)?;
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Blake2b => {
                let mut hasher = Blake2b512::new();
                self.update_hasher_with_progress(&mut hasher, &mut reader, &progress_bar)?;
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Blake2s => {
                let mut hasher = Blake2s256::new();
                self.update_hasher_with_progress(&mut hasher, &mut reader, &progress_bar)?;
                Ok(HashResult::new(hasher.finalize().to_vec(), algorithm))
            }
            HashAlgorithm::Blake3 => {
                let mut hasher = blake3::Hasher::new();
                let mut buffer = [0; 8192];
                let mut total_read = 0u64;

                loop {
                    let bytes_read = reader.read(&mut buffer)?;
                    if bytes_read == 0 {
                        break;
                    }

                    hasher.update(&buffer[..bytes_read]);
                    total_read += bytes_read as u64;

                    if let Some(pb) = &progress_bar {
                        pb.set_position(total_read);
                    }
                }

                if let Some(pb) = &progress_bar {
                    pb.finish_with_message("Hash computed");
                }

                Ok(HashResult::new(
                    hasher.finalize().as_bytes().to_vec(),
                    algorithm,
                ))
            }

            // Password algorithms don't make sense for files
            HashAlgorithm::Bcrypt | HashAlgorithm::Scrypt | HashAlgorithm::Argon2 => {
                anyhow::bail!("Password hashing algorithms cannot be used for file hashing")
            }
        }
    }

    /// Helper method to update hashers with progress tracking
    fn update_hasher_with_progress<D: Digest>(
        &self,
        hasher: &mut D,
        reader: &mut BufReader<fs::File>,
        progress_bar: &Option<ProgressBar>,
    ) -> Result<()> {
        let mut buffer = [0; 8192];
        let mut total_read = 0u64;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            Digest::update(hasher, &buffer[..bytes_read]);
            total_read += bytes_read as u64;

            if let Some(pb) = progress_bar {
                pb.set_position(total_read);
            }
        }

        if let Some(pb) = progress_bar {
            pb.finish_with_message("Hash computed");
        }

        Ok(())
    }

    /// Verify hash against expected value
    pub fn verify_hash(&self, computed: &HashResult, expected: &str) -> Result<bool> {
        // Try to decode expected hash as hex first, then base64
        if let Ok(expected_bytes) = hex::decode(expected) {
            Ok(computed.constant_time_eq(&expected_bytes))
        } else {
            use base64::Engine;
            if let Ok(expected_bytes) = base64::engine::general_purpose::STANDARD.decode(expected) {
                Ok(computed.constant_time_eq(&expected_bytes))
            } else {
                anyhow::bail!("Expected hash is not valid hex or base64")
            }
        }
    }

    /// Batch process directory
    pub fn batch_process_directory(
        &self,
        directory: &Path,
        algorithm: HashAlgorithm,
        output_format: OutputFormat,
    ) -> Result<()> {
        let entries = fs::read_dir(directory)
            .with_context(|| format!("Failed to read directory: {}", directory.display()))?;

        println!("📁 Processing files in: {}", directory.display());
        println!("Algorithm: {algorithm}");
        println!("Output format: {output_format}");
        println!();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                match self.hash_file(&path, algorithm) {
                    Ok(result) => {
                        let formatted = match output_format {
                            OutputFormat::Hex => result.to_hex(),
                            OutputFormat::Base64 => result.to_base64(),
                        };
                        println!(
                            "{} {}",
                            formatted,
                            path.file_name().unwrap().to_string_lossy()
                        );
                    }
                    Err(e) => {
                        eprintln!("❌ Error processing {}: {e}", path.display());
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for HashForge {
    fn default() -> Self {
        Self::new()
    }
}
