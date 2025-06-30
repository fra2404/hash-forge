// GUI rendering and UI components
use super::algorithms::{ALL_CATEGORIES, AVAILABLE_OUTPUT_FORMATS};
use super::app_state::{HashForgeApp, InputMode};
use crate::algorithms::HashAlgorithm;

#[cfg(feature = "gui")]
impl eframe::App for HashForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark/Light theme toggle
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ui.separator();
            ui.add_space(10.0);

            // Main content in scroll area
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_mode_selection(ui);
                ui.add_space(10.0);

                self.render_input_section(ui);
                ui.add_space(10.0);

                if self.hmac_mode {
                    self.render_hmac_key_section(ui);
                    ui.add_space(10.0);
                }

                self.render_algorithm_section(ui);
                ui.add_space(10.0);

                self.render_control_buttons(ui);
                ui.add_space(10.0);

                if self.verification_mode && !self.hmac_mode {
                    self.render_verification_section(ui);
                    ui.add_space(10.0);
                }

                if self.hmac_mode {
                    self.render_hmac_verification_section(ui);
                    ui.add_space(10.0);
                }

                if !self.display_hash_result().is_empty() {
                    self.render_results_section(ui);
                }

                ui.add_space(20.0);
            });
        });
    }
}

impl HashForgeApp {
    fn render_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(
                egui::RichText::new("🔧 Hash Forge")
                    .size(32.0)
                    .color(egui::Color32::from_rgb(100, 150, 200)),
            );
            ui.label(
                egui::RichText::new("Professional Hash Generator & Verifier")
                    .size(14.0)
                    .color(egui::Color32::GRAY),
            );
            ui.add_space(15.0);
        });
    }

    fn render_mode_selection(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.strong("🔧 Mode Selection");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.hmac_mode, false, "🔐 Hash Mode");
                ui.selectable_value(&mut self.hmac_mode, true, "🔑 HMAC Mode");
            });
        });
    }

    fn render_input_section(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.strong("📝 Input");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.input_mode, InputMode::Text, "📄 Text");
                ui.selectable_value(&mut self.input_mode, InputMode::File, "📁 File");
            });

            ui.add_space(5.0);

            match self.input_mode {
                InputMode::Text => {
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut self.input_text)
                            .desired_rows(4)
                            .hint_text("Enter text to hash...")
                            .font(egui::TextStyle::Monospace),
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
                            ui.label(
                                egui::RichText::new("No file selected").color(egui::Color32::GRAY),
                            );
                        }

                        if ui.button("📂 Browse...").clicked() {
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
    }

    fn render_hmac_key_section(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.strong("🔑 HMAC Key");
            ui.add_space(5.0);

            let response = ui.add(
                egui::TextEdit::singleline(&mut self.hmac_key)
                    .hint_text("Enter HMAC key...")
                    .password(true),
            );

            if response.changed() && self.auto_compute && self.can_compute() {
                self.clear_results();
                self.compute_hash();
            }

            if !self.is_hmac_supported() {
                ui.colored_label(
                    egui::Color32::YELLOW,
                    "⚠️ HMAC not supported for this algorithm",
                );
            }
        });
    }

    fn render_algorithm_section(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.strong("⚙️ Algorithm & Options");
            ui.add_space(5.0);

            // Category filter
            ui.horizontal(|ui| {
                ui.label("Category:");
                egui::ComboBox::from_id_source("category")
                    .selected_text(format!("{}", self.algorithm_category))
                    .show_ui(ui, |ui| {
                        for &category in &ALL_CATEGORIES {
                            ui.selectable_value(
                                &mut self.algorithm_category,
                                category,
                                format!("{category}"),
                            );
                        }
                    });
            });

            ui.add_space(5.0);

            // Algorithm selection
            ui.horizontal(|ui| {
                ui.label("Algorithm:");
                egui::ComboBox::from_id_source("algorithm")
                    .selected_text(format!("{}", self.selected_algorithm))
                    .show_ui(ui, |ui| {
                        for &algorithm in &self.filtered_algorithms() {
                            let mut text = format!("{algorithm}");

                            // Add emoji indicators
                            if algorithm.is_password_hash() {
                                text = format!("🔒 {text}");
                            } else if matches!(
                                algorithm,
                                HashAlgorithm::Blake3
                                    | HashAlgorithm::Sha3_256
                                    | HashAlgorithm::XxHash3
                            ) {
                                text = format!("⭐ {text}");
                            } else if matches!(algorithm, HashAlgorithm::Md5 | HashAlgorithm::Sha1)
                            {
                                text = format!("⚠️ {text}");
                            }

                            let response =
                                ui.selectable_value(&mut self.selected_algorithm, algorithm, text);
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
                            let response = ui.selectable_value(
                                &mut self.output_format,
                                format,
                                format!("{format}"),
                            );
                            if response.clicked() && self.auto_compute && self.can_compute() {
                                self.clear_results();
                                self.compute_hash();
                            }
                        }
                    });
            });

            ui.add_space(5.0);

            // Algorithm info with color coding
            ui.horizontal(|ui| {
                let (icon, color) = if self.selected_algorithm.is_password_hash() {
                    ("🔒", egui::Color32::from_rgb(100, 200, 100))
                } else if matches!(
                    self.selected_algorithm,
                    HashAlgorithm::Md5 | HashAlgorithm::Sha1
                ) {
                    ("⚠️", egui::Color32::YELLOW)
                } else {
                    ("ℹ️", egui::Color32::LIGHT_BLUE)
                };

                ui.label(icon);
                ui.colored_label(color, self.selected_algorithm.recommended_use());
            });

            // Advanced options for password hashing
            if self.selected_algorithm.is_password_hash() {
                ui.add_space(5.0);
                ui.collapsing("🔧 Advanced Options", |ui| {
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
                        let response =
                            ui.add(egui::DragValue::new(&mut self.iterations).range(1..=100000));
                        if response.changed() && self.auto_compute && self.can_compute() {
                            self.clear_results();
                            self.compute_hash();
                        }
                    });
                });
            }
        });
    }

    fn render_control_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let compute_text = if self.hmac_mode {
                "🔑 Compute HMAC"
            } else {
                "🔨 Compute Hash"
            };

            if ui
                .add(egui::Button::new(compute_text).min_size(egui::vec2(120.0, 30.0)))
                .clicked()
                && self.can_compute()
            {
                self.clear_results();
                self.compute_hash();
            }

            ui.separator();

            ui.checkbox(&mut self.auto_compute, "⚡ Auto-compute");

            if !self.hmac_mode {
                ui.checkbox(&mut self.verification_mode, "🔍 Verification mode");
            }

            if ui.button("🗑️ Clear").clicked() {
                self.input_text.clear();
                self.selected_file = None;
                self.hmac_key.clear();
                self.clear_results();
            }
        });
    }

    fn render_verification_section(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.strong("🔍 Hash Verification");
            ui.add_space(5.0);

            let response = ui.add(
                egui::TextEdit::singleline(&mut self.expected_hash)
                    .hint_text("Enter expected hash (hex or base64)...")
                    .font(egui::TextStyle::Monospace),
            );

            if response.changed() && self.auto_compute && self.can_compute() {
                self.compute_hash(); // This will also verify
            }

            if let Some(is_valid) = self.verification_result {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if is_valid {
                        ui.colored_label(egui::Color32::GREEN, "✅ Hash verification PASSED");
                    } else {
                        ui.colored_label(egui::Color32::RED, "❌ Hash verification FAILED");
                    }
                });
            }
        });
    }

    fn render_hmac_verification_section(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.strong("🔍 HMAC Verification");
            ui.add_space(5.0);

            let response = ui.add(
                egui::TextEdit::singleline(&mut self.hmac_expected)
                    .hint_text("Enter expected HMAC (hex or base64)...")
                    .font(egui::TextStyle::Monospace),
            );

            if response.changed() && self.auto_compute && self.can_compute() {
                self.compute_hash(); // This will also verify
            }

            if let Some(is_valid) = self.hmac_verification_result {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if is_valid {
                        ui.colored_label(egui::Color32::GREEN, "✅ HMAC verification PASSED");
                    } else {
                        ui.colored_label(egui::Color32::RED, "❌ HMAC verification FAILED");
                    }
                });
            }
        });
    }

    fn render_results_section(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.strong("📊 Results");
            ui.add_space(5.0);

            // Hash result with copy button
            ui.horizontal(|ui| {
                let result_label = if self.hmac_mode { "HMAC:" } else { "Hash:" };
                ui.label(result_label);

                let hash_text = self.display_hash_result();
                ui.add(
                    egui::TextEdit::multiline(&mut hash_text.to_string())
                        .desired_rows(2)
                        .desired_width(ui.available_width() - 100.0)
                        .font(egui::TextStyle::Monospace),
                );

                ui.vertical(|ui| {
                    if ui.button("📋 Copy").clicked() {
                        ui.output_mut(|o| o.copied_text = hash_text.to_string());
                    }

                    if ui.button("💾 Save").clicked() {
                        // TODO: Implement save to file
                    }
                });
            });

            // Performance info with icons
            if self.computation_time.is_some() {
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("⏱️ Time:");
                    ui.strong(self.formatted_computation_time());

                    if self.file_size.is_some() {
                        ui.separator();
                        ui.label("📏 Size:");
                        ui.strong(self.formatted_file_size());

                        // Calculate and show speed
                        if let (Some(size), Some(time)) = (self.file_size, self.computation_time) {
                            if time.as_secs_f64() > 0.0 {
                                let speed = size as f64 / time.as_secs_f64();
                                ui.separator();
                                ui.label("⚡ Speed:");
                                ui.strong(crate::utils::format_file_size(speed as u64) + "/s");
                            }
                        }
                    }
                });
            }
        });
    }
}
