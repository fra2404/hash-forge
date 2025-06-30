// GUI module organization
pub mod algorithms;
pub mod app_state;
pub mod compute;
pub mod ui;

// Re-export main types
pub use algorithms::{ALL_ALGORITHMS, ALL_CATEGORIES};
pub use app_state::{AlgorithmCategory, HashForgeApp, InputMode};

// Output formats constant
use crate::output::OutputFormat;
pub const AVAILABLE_OUTPUT_FORMATS: [OutputFormat; 2] = [OutputFormat::Hex, OutputFormat::Base64];
