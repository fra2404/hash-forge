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
    
    // Password hashing options
    pub custom_salt: String,
    pub use_custom_salt: bool,
    pub iterations: u32,
    
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

impl HashForgeApp {
    pub fn new() -> Self {
        Self {
            iterations: 4096,
            auto_compute: true,
            forge: HashForge::new().with_progress(false), // Disable progress in GUI
            ..Default::default()
        }
    }
    
    /// Compute hash based on current settings
    pub fn compute_hash(&mut self) {
        let start_time = std::time::Instant::now();
        
        let result = match self.input_mode {
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
                
                self.forge.hash_text(&self.input_text, self.selected_algorithm, salt, iterations)
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
        };
        
        self.computation_time = Some(start_time.elapsed());
        
        match result {
            Ok(hash) => {
                self.hash_result = Some(match self.output_format {
                    OutputFormat::Hex => hash.to_hex(),
                    OutputFormat::Base64 => hash.to_base64(),
                });
                
                // If in verification mode, check against expected hash
                if self.verification_mode && !self.expected_hash.is_empty() {
                    self.verification_result = self.forge.verify_hash(&hash, &self.expected_hash).ok();
                }
            }
            Err(e) => {
                self.hash_result = Some(format!("Error: {e}"));
            }
        }
    }
    
    /// Clear current results
    pub fn clear_results(&mut self) {
        self.hash_result = None;
        self.computation_time = None;
        self.file_size = None;
        self.verification_result = None;
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
    
    /// Copy hash result to clipboard
    pub fn copy_to_clipboard(&self) {
        if let Some(ref result) = self.hash_result {
            // Note: Clipboard functionality would require additional dependencies
            // For now, we'll just indicate that the text should be copied
            println!("Copy to clipboard: {result}");
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for HashForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔧 Hash Forge");
            ui.add_space(10.0);
            
            // Input section
            ui.group(|ui| {
                ui.label("📝 Input");
                
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.input_mode, InputMode::Text, "Text");
                    ui.selectable_value(&mut self.input_mode, InputMode::File, "File");
                });
                
                match self.input_mode {
                    InputMode::Text => {
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut self.input_text)
                                .desired_rows(3)
                                .hint_text("Enter text to hash..."),
                        );
                        
                        if response.changed() && self.auto_compute {
                            self.clear_results();
                            self.compute_hash();
                        }
                    }
                    InputMode::File => {
                        ui.horizontal(|ui| {
                            if let Some(ref path) = self.selected_file {
                                ui.label(format!("📄 {}", path.display()));
                            } else {
                                ui.label("No file selected");
                            }
                            
                            #[cfg(feature = "gui")]
                            if ui.button("Browse...").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    self.selected_file = Some(path);
                                    if self.auto_compute {
                                        self.clear_results();
                                        self.compute_hash();
                                    }
                                }
                            }
                        });
                    }
                }
            });
            
            ui.add_space(10.0);
            
            // Algorithm and options section
            ui.group(|ui| {
                ui.label("⚙️ Algorithm & Options");
                
                ui.horizontal(|ui| {
                    ui.label("Algorithm:");
                    egui::ComboBox::from_id_source("algorithm")
                        .selected_text(format!("{}", self.selected_algorithm))
                        .show_ui(ui, |ui| {
                            for &algorithm in &AVAILABLE_ALGORITHMS {
                                let response = ui.selectable_value(&mut self.selected_algorithm, algorithm, format!("{algorithm}"));
                                if response.clicked() && self.auto_compute && self.can_compute() {
                                    self.clear_results();
                                    self.compute_hash();
                                }
                            }
                        });
                    
                    ui.separator();
                    
                    ui.label("Output:");
                    egui::ComboBox::from_id_source("output_format")
                        .selected_text(format!("{}", self.output_format))
                        .show_ui(ui, |ui| {
                            for &format in &AVAILABLE_OUTPUT_FORMATS {
                                let response = ui.selectable_value(&mut self.output_format, format, format!("{format}"));
                                if response.clicked() && self.auto_compute && self.can_compute() {
                                    self.clear_results();
                                    self.compute_hash();
                                }
                            }
                        });
                });
                
                // Algorithm info
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("ℹ️");
                    ui.label(self.selected_algorithm.recommended_use());
                });
                
                // Advanced options for password hashing
                if self.selected_algorithm.is_password_hash() {
                    ui.separator();
                    ui.collapsing("Advanced Options", |ui| {
                        ui.checkbox(&mut self.use_custom_salt, "Use custom salt");
                        if self.use_custom_salt {
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.custom_salt)
                                    .hint_text("Enter salt..."),
                            );
                            if response.changed() && self.auto_compute && self.can_compute() {
                                self.clear_results();
                                self.compute_hash();
                            }
                        }
                        
                        ui.horizontal(|ui| {
                            ui.label("Iterations:");
                            let response = ui.add(egui::DragValue::new(&mut self.iterations).range(1..=100000));
                            if response.changed() && self.auto_compute && self.can_compute() {
                                self.clear_results();
                                self.compute_hash();
                            }
                        });
                    });
                }
            });
            
            ui.add_space(10.0);
            
            // Control buttons
            ui.horizontal(|ui| {
                if ui.button("🔨 Compute Hash").clicked() && self.can_compute() {
                    self.clear_results();
                    self.compute_hash();
                }
                
                ui.separator();
                
                ui.checkbox(&mut self.auto_compute, "Auto-compute");
                ui.checkbox(&mut self.verification_mode, "Verification mode");
                
                if ui.button("🗑️ Clear").clicked() {
                    self.input_text.clear();
                    self.selected_file = None;
                    self.clear_results();
                }
            });
            
            ui.add_space(10.0);
            
            // Verification section
            if self.verification_mode {
                ui.group(|ui| {
                    ui.label("🔍 Hash Verification");
                    
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.expected_hash)
                            .hint_text("Enter expected hash (hex or base64)..."),
                    );
                    
                    if response.changed() && self.auto_compute && self.can_compute() {
                        self.compute_hash(); // This will also verify
                    }
                    
                    if let Some(is_valid) = self.verification_result {
                        ui.horizontal(|ui| {
                            if is_valid {
                                ui.colored_label(egui::Color32::GREEN, "✅ Hash verification PASSED");
                            } else {
                                ui.colored_label(egui::Color32::RED, "❌ Hash verification FAILED");
                            }
                        });
                    }
                });
                
                ui.add_space(10.0);
            }
            
            // Results section
            if !self.display_hash_result().is_empty() {
                ui.group(|ui| {
                    ui.label("📊 Results");
                    
                    // Hash result
                    ui.horizontal(|ui| {
                        ui.label("Hash:");
                        let hash_text = self.display_hash_result();
                        ui.add(
                            egui::TextEdit::multiline(&mut hash_text.to_string())
                                .desired_rows(2)
                                .desired_width(ui.available_width() - 80.0),
                        );
                        
                        if ui.button("📋 Copy").clicked() {
                            ui.output_mut(|o| o.copied_text = hash_text.to_string());
                        }
                    });
                    
                    // Performance info
                    if self.computation_time.is_some() {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("⏱️ Time:");
                            ui.label(self.formatted_computation_time());
                            
                            if self.file_size.is_some() {
                                ui.separator();
                                ui.label("📏 Size:");
                                ui.label(self.formatted_file_size());
                            }
                        });
                    }
                });
            }
        });
    }
}

const AVAILABLE_ALGORITHMS: [HashAlgorithm; 9] = [
    HashAlgorithm::Md5,
    HashAlgorithm::Sha1,
    HashAlgorithm::Sha256,
    HashAlgorithm::Sha512,
    HashAlgorithm::Blake2b,
    HashAlgorithm::Blake2s,
    HashAlgorithm::Blake3,
    HashAlgorithm::Bcrypt,
    HashAlgorithm::Scrypt,
];

const AVAILABLE_OUTPUT_FORMATS: [OutputFormat; 2] = [
    OutputFormat::Hex,
    OutputFormat::Base64,
];
