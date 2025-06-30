use anyhow::Result;
use clap::Parser;
use hash_forge::{
    cli::{Cli, Commands},
    core::HashForge,
    hmac_core::HmacProcessor,
    output::OutputFormat,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Text {
            input,
            algorithm,
            output_format,
            salt,
            iterations,
        } => {
            let forge = HashForge::new();
            let result = forge.hash_text(&input, algorithm, salt.as_deref(), iterations)?;

            let formatted = match output_format {
                OutputFormat::Hex => result.to_hex(),
                OutputFormat::Base64 => result.to_base64(),
            };

            println!("🔧 Hash Forge - Text Hashing");
            println!("Input: {input}");
            println!("Algorithm: {algorithm}");
            if let Some(salt) = salt {
                println!("Salt: {salt}");
            }
            if let Some(iter) = iterations {
                println!("Iterations: {iter}");
            }
            println!("Hash ({output_format}): {formatted}");
        }

        Commands::File {
            path,
            algorithm,
            output_format,
        } => {
            let forge = HashForge::new();

            println!("🔧 Hash Forge - File Hashing");
            println!("File: {}", path.display());
            println!("Algorithm: {algorithm}");

            let result = forge.hash_file(&path, algorithm)?;
            let formatted = match output_format {
                OutputFormat::Hex => result.to_hex(),
                OutputFormat::Base64 => result.to_base64(),
            };

            println!("Hash ({output_format}): {formatted}");
        }

        Commands::Verify {
            text,
            file,
            expected_hash,
            algorithm,
        } => {
            let forge = HashForge::new();

            let computed_hash = if let Some(text_value) = text {
                forge.hash_text(&text_value, algorithm, None, None)?
            } else if let Some(file_path) = file {
                forge.hash_file(&file_path, algorithm)?
            } else {
                anyhow::bail!("Either --text or --file must be specified for verification");
            };

            let is_valid = forge.verify_hash(&computed_hash, &expected_hash)?;

            println!("🔧 Hash Forge - Hash Verification");
            println!("Algorithm: {algorithm}");
            println!("Expected: {expected_hash}");
            println!("Computed: {}", computed_hash.to_hex());

            if is_valid {
                println!("✅ Hash verification PASSED");
            } else {
                println!("❌ Hash verification FAILED");
                std::process::exit(1);
            }
        }

        Commands::Batch {
            directory,
            algorithm,
            output_format,
        } => {
            let forge = HashForge::new();

            println!("🔧 Hash Forge - Batch Processing");
            println!("Directory: {}", directory.display());
            println!("Algorithm: {algorithm}");

            forge.batch_process_directory(&directory, algorithm, output_format)?;
        }

        Commands::Hmac {
            text,
            file,
            key,
            algorithm,
            output_format,
        } => {
            if !HmacProcessor::supports_hmac(algorithm) {
                anyhow::bail!("HMAC is not supported for algorithm: {}", algorithm);
            }

            let data = if let Some(ref text_value) = text {
                text_value.as_bytes().to_vec()
            } else if let Some(ref file_path) = file {
                std::fs::read(&file_path)?
            } else {
                anyhow::bail!("Either --text or --file must be specified for HMAC");
            };

            let result = HmacProcessor::compute_hmac(&data, key.as_bytes(), algorithm)?;
            let formatted = match output_format {
                OutputFormat::Hex => result.to_hex(),
                OutputFormat::Base64 => result.to_base64(),
            };

            println!("🔧 Hash Forge - HMAC Computation");
            println!("Algorithm: {algorithm}");
            println!("Key: <hidden for security>");
            if let Some(ref t) = text {
                println!("Input: {t}");
            } else if let Some(ref f) = file {
                println!("File: {}", f.display());
            }
            println!("HMAC ({output_format}): {formatted}");
        }

        Commands::VerifyHmac {
            text,
            file,
            key,
            expected_hmac,
            algorithm,
        } => {
            if !HmacProcessor::supports_hmac(algorithm) {
                anyhow::bail!("HMAC is not supported for algorithm: {}", algorithm);
            }

            let data = if let Some(ref text_value) = text {
                text_value.as_bytes().to_vec()
            } else if let Some(ref file_path) = file {
                std::fs::read(&file_path)?
            } else {
                anyhow::bail!("Either --text or --file must be specified for HMAC verification");
            };

            // Parse expected HMAC from hex
            let expected_bytes = hex::decode(&expected_hmac)
                .map_err(|_| anyhow::anyhow!("Invalid hex format for expected HMAC"))?;

            let is_valid = HmacProcessor::verify_hmac(&data, key.as_bytes(), &expected_bytes, algorithm)?;

            println!("🔧 Hash Forge - HMAC Verification");
            println!("Algorithm: {algorithm}");
            println!("Expected: {expected_hmac}");
            
            let computed = HmacProcessor::compute_hmac(&data, key.as_bytes(), algorithm)?;
            println!("Computed: {}", computed.to_hex());

            if is_valid {
                println!("✅ HMAC verification PASSED");
            } else {
                println!("❌ HMAC verification FAILED");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
