// TinyVM with theme switching, program editor, and live state visualization.

use eframe::{egui, epi};
use std::fs;

pub struct TinyVMGuiApp {
    pub output: String,
    pub step_count: usize,
    pub memory: [u8; 16],
    pub editor_content: String,
    pub theme: String,
    pub available_themes: Vec<String>,
}

impl Default for TinyVMGuiApp {
    fn default() -> Self {
        // Load available themes from the themes folder
        let theme_files = fs::read_dir("themes")
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| e.path().file_stem().map(|s| s.to_string_lossy().to_string()))
                    .collect()
            })
            .unwrap_or_else(|_| vec!["default".to_string()]);
        Self {
            output: "Welcome to TinyVM GUI!".to_string(),
            step_count: 0,
            memory: [0; 16],
            editor_content: "// Write your program here\n".to_string(),
            theme: "default".to_string(),
            available_themes: theme_files,
        }
    }
}

impl epi::App for TinyVMGuiApp {
    fn name(&self) -> &str {
        "TinyVM Advanced GUI"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("TinyVM Visualizer");
            ui.horizontal(|ui| {
                ui.label("Theme:");
                for theme in &self.available_themes {
                    if ui.button(theme).clicked() {
                        self.theme = theme.clone();
                        self.output = format!("Theme switched to '{}'", theme);
                        // Here you could load and apply the theme settings
                    }
                }
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Program Editor");
            ui.add(
                egui::TextEdit::multiline(&mut self.editor_content)
                    .desired_rows(10)
                    .desired_width(200.0),
            );
            if ui.button("Load Program").clicked() {
                self.output = "Program loaded into VM (stub)".to_string();
                // Here you would parse and load the program into your VM
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Run Step").clicked() {
                    self.step_count += 1;
                    self.memory[self.step_count % 16] = self.step_count as u8;
                    self.output = format!("Executed step {}", self.step_count);
                }
                if ui.button("Reset").clicked() {
                    self.step_count = 0;
                    self.memory = [0; 16];
                    self.output = "VM reset.".to_string();
                }
            });
            ui.separator();
            ui.label(format!("Output: {}", self.output));
            ui.separator();
            ui.heading("Live Memory State:");
            egui::Grid::new("memory_grid").show(ui, |ui| {
                for (i, val) in self.memory.iter().enumerate() {
                    ui.label(format!("Addr {:02}: {}", i, val));
                    if (i + 1) % 4 == 0 {
                        ui.end_row();
                    }
                }
            });
        });
    }
}

fn main() {
    let app = TinyVMGuiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}