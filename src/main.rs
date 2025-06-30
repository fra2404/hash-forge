use anyhow::Result;
use clap::Parser;
use hash_forge::{
    cli::{Cli, Commands},
    core::HashForge,
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
            println!("Input: {}", input);
            println!("Algorithm: {}", algorithm);
            if let Some(salt) = salt {
                println!("Salt: {}", salt);
            }
            if let Some(iter) = iterations {
                println!("Iterations: {}", iter);
            }
            println!("Hash ({}): {}", output_format, formatted);
        }
        
        Commands::File {
            path,
            algorithm,
            output_format,
        } => {
            let forge = HashForge::new();
            
            println!("🔧 Hash Forge - File Hashing");
            println!("File: {}", path.display());
            println!("Algorithm: {}", algorithm);
            
            let result = forge.hash_file(&path, algorithm)?;
            let formatted = match output_format {
                OutputFormat::Hex => result.to_hex(),
                OutputFormat::Base64 => result.to_base64(),
            };
            
            println!("Hash ({}): {}", output_format, formatted);
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
            println!("Algorithm: {}", algorithm);
            println!("Expected: {}", expected_hash);
            println!("Computed: {}", computed_hash.to_hex());
            
            if is_valid {
                println!("✅ Hash verification PASSED");
            } else {
                println!("❌ Hash verification FAILED");
                std::process::exit(1);
            }
        }
        
        Commands::Batch { directory, algorithm, output_format } => {
            let forge = HashForge::new();
            
            println!("🔧 Hash Forge - Batch Processing");
            println!("Directory: {}", directory.display());
            println!("Algorithm: {}", algorithm);
            
            forge.batch_process_directory(&directory, algorithm, output_format)?;
        }
    }

    Ok(())
}
