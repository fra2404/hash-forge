// Hash computation logic for GUI
use super::app_state::{HashForgeApp, InputMode};

impl HashForgeApp {
    /// Compute hash based on current settings
    pub fn compute_hash(&mut self) {
        let start_time = std::time::Instant::now();

        let result = if self.hmac_mode {
            // HMAC computation
            self.compute_hmac()
        } else {
            // Regular hash computation
            match self.input_mode {
                InputMode::Text => {
                    if self.input_text.is_empty() {
                        self.hash_result = None;
                        return;
                    }

                    let salt = if self.use_custom_salt && !self.custom_salt.is_empty() {
                        Some(self.custom_salt.as_str())
                    } else {
                        None
                    };

                    let iterations = if self.selected_algorithm.is_password_hash() {
                        Some(self.iterations)
                    } else {
                        None
                    };

                    self.forge.hash_text(
                        &self.input_text,
                        self.selected_algorithm,
                        salt,
                        iterations,
                    )
                }
                InputMode::File => {
                    if let Some(ref path) = self.selected_file {
                        // Get file size
                        if let Ok(metadata) = std::fs::metadata(path) {
                            self.file_size = Some(metadata.len());
                        }

                        self.forge.hash_file(path, self.selected_algorithm)
                    } else {
                        self.hash_result = None;
                        return;
                    }
                }
            }
        };

        self.computation_time = Some(start_time.elapsed());

        match result {
            Ok(hash) => {
                self.hash_result = Some(match self.output_format {
                    crate::output::OutputFormat::Hex => hash.to_hex(),
                    crate::output::OutputFormat::Base64 => hash.to_base64(),
                });

                // Handle verification
                if self.hmac_mode && !self.hmac_expected.is_empty() {
                    self.hmac_verification_result =
                        self.forge.verify_hash(&hash, &self.hmac_expected).ok();
                } else if self.verification_mode && !self.expected_hash.is_empty() {
                    self.verification_result =
                        self.forge.verify_hash(&hash, &self.expected_hash).ok();
                }
            }
            Err(e) => {
                self.hash_result = Some(format!("Error: {e}"));
            }
        }
    }

    /// Compute HMAC based on current settings
    fn compute_hmac(&mut self) -> anyhow::Result<crate::output::HashResult> {
        if self.hmac_key.is_empty() {
            return Err(anyhow::anyhow!("HMAC key is required"));
        }

        let data = match self.input_mode {
            InputMode::Text => self.input_text.as_bytes(),
            InputMode::File => {
                if self.selected_file.is_some() {
                    return Err(anyhow::anyhow!("File HMAC not yet implemented in GUI"));
                } else {
                    return Err(anyhow::anyhow!("No file selected"));
                }
            }
        };

        crate::hmac_core::HmacProcessor::compute_hmac(
            data,
            self.hmac_key.as_bytes(),
            self.selected_algorithm,
        )
    }
}
