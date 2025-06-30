use crate::{algorithms::HashAlgorithm, output::OutputFormat};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hash-forge")]
#[command(about = "Professional hash generator and verifier built with Rust")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "Francesco <fra2404@users.noreply.github.com>")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Hash text input
    Text {
        /// Text to hash
        #[arg(short, long)]
        input: String,

        /// Hash algorithm to use
        #[arg(short, long, default_value = "sha256")]
        algorithm: HashAlgorithm,

        /// Output format
        #[arg(short = 'f', long, default_value = "hex")]
        output_format: OutputFormat,

        /// Salt for password hashing algorithms
        #[arg(short, long)]
        salt: Option<String>,

        /// Number of iterations for password hashing
        #[arg(long)]
        iterations: Option<u32>,
    },

    /// Hash file contents
    File {
        /// Path to file
        #[arg(short, long)]
        path: PathBuf,

        /// Hash algorithm to use
        #[arg(short, long, default_value = "sha256")]
        algorithm: HashAlgorithm,

        /// Output format
        #[arg(short = 'f', long, default_value = "hex")]
        output_format: OutputFormat,
    },

    /// Verify hash against expected value
    Verify {
        /// Text to verify (mutually exclusive with --file)
        #[arg(short = 't', long, group = "input")]
        text: Option<String>,

        /// File to verify (mutually exclusive with --text)
        #[arg(short = 'f', long, group = "input")]
        file: Option<PathBuf>,

        /// Expected hash value
        #[arg(short, long)]
        expected_hash: String,

        /// Hash algorithm used
        #[arg(short, long, default_value = "sha256")]
        algorithm: HashAlgorithm,
    },

    /// Batch process directory
    Batch {
        /// Directory to process
        #[arg(short, long)]
        directory: PathBuf,

        /// Hash algorithm to use
        #[arg(short, long, default_value = "sha256")]
        algorithm: HashAlgorithm,

        /// Output format
        #[arg(short = 'f', long, default_value = "hex")]
        output_format: OutputFormat,
    },

    /// Compute HMAC (Keyed-Hash Message Authentication Code)
    Hmac {
        /// Text to compute HMAC for (mutually exclusive with --file)
        #[arg(short = 't', long, group = "input")]
        text: Option<String>,

        /// File to compute HMAC for (mutually exclusive with --text)
        #[arg(short = 'f', long, group = "input")]
        file: Option<PathBuf>,

        /// HMAC key
        #[arg(short, long)]
        key: String,

        /// Hash algorithm to use for HMAC
        #[arg(short, long, default_value = "sha256")]
        algorithm: HashAlgorithm,

        /// Output format
        #[arg(short = 'o', long, default_value = "hex")]
        output_format: OutputFormat,
    },

    /// Verify HMAC against expected value
    VerifyHmac {
        /// Text to verify HMAC for (mutually exclusive with --file)
        #[arg(short = 't', long, group = "input")]
        text: Option<String>,

        /// File to verify HMAC for (mutually exclusive with --text)
        #[arg(short = 'f', long, group = "input")]
        file: Option<PathBuf>,

        /// HMAC key
        #[arg(short, long)]
        key: String,

        /// Expected HMAC value
        #[arg(short, long)]
        expected_hmac: String,

        /// Hash algorithm used for HMAC
        #[arg(short, long, default_value = "sha256")]
        algorithm: HashAlgorithm,
    },
}
