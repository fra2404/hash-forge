// GUI Application State and Core Logic
use crate::{algorithms::HashAlgorithm, core::HashForge, output::OutputFormat};
use std::path::PathBuf;

#[derive(Default)]
pub struct HashForgeApp {
    // Input options
    pub input_text: String,
    pub selected_file: Option<PathBuf>,
    pub input_mode: InputMode,

    // Algorithm selection
    pub selected_algorithm: HashAlgorithm,
    pub output_format: OutputFormat,
    pub algorithm_category: AlgorithmCategory,

    // Password hashing options
    pub custom_salt: String,
    pub use_custom_salt: bool,
    pub iterations: u32,

    // HMAC options
    pub hmac_mode: bool,
    pub hmac_key: String,
    pub hmac_expected: String,
    pub hmac_verification_result: Option<bool>,

    // Results
    pub hash_result: Option<String>,
    pub computation_time: Option<std::time::Duration>,
    pub file_size: Option<u64>,

    // Verification
    pub verification_mode: bool,
    pub expected_hash: String,
    pub verification_result: Option<bool>,

    // UI state
    pub show_advanced: bool,
    pub auto_compute: bool,

    // Core engine
    pub forge: HashForge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Text,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlgorithmCategory {
    #[default]
    All,
    FastHash,
    PasswordHash,
    Modern,
    Legacy,
}

impl std::fmt::Display for AlgorithmCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmCategory::All => write!(f, "All Algorithms"),
            AlgorithmCategory::FastHash => write!(f, "Fast Hash"),
            AlgorithmCategory::PasswordHash => write!(f, "Password Hash"),
            AlgorithmCategory::Modern => write!(f, "Modern"),
            AlgorithmCategory::Legacy => write!(f, "Legacy"),
        }
    }
}

impl HashForgeApp {
    pub fn new() -> Self {
        Self {
            iterations: 4096,
            auto_compute: true,
            forge: HashForge::new().with_progress(false), // Disable progress in GUI
            ..Default::default()
        }
    }

    /// Check if current configuration is valid for computation
    pub fn can_compute(&self) -> bool {
        match self.input_mode {
            InputMode::Text => !self.input_text.is_empty(),
            InputMode::File => self.selected_file.is_some(),
        }
    }

    /// Get current hash result for display
    pub fn display_hash_result(&self) -> &str {
        self.hash_result.as_deref().unwrap_or("")
    }

    /// Check if HMAC is supported for current algorithm
    pub fn is_hmac_supported(&self) -> bool {
        matches!(
            self.selected_algorithm,
            HashAlgorithm::Sha1
                | HashAlgorithm::Sha256
                | HashAlgorithm::Sha512
                | HashAlgorithm::Sha3_224
                | HashAlgorithm::Sha3_256
                | HashAlgorithm::Sha3_384
                | HashAlgorithm::Sha3_512
        )
    }

    /// Get formatted computation time
    pub fn formatted_computation_time(&self) -> String {
        self.computation_time
            .map(crate::utils::format_duration)
            .unwrap_or_default()
    }

    /// Get formatted file size
    pub fn formatted_file_size(&self) -> String {
        self.file_size
            .map(crate::utils::format_file_size)
            .unwrap_or_default()
    }

    /// Clear current results
    pub fn clear_results(&mut self) {
        self.hash_result = None;
        self.computation_time = None;
        self.file_size = None;
        self.verification_result = None;
        self.hmac_verification_result = None;
    }
}
