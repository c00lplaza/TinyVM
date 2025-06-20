use eframe::{egui, epi};

pub fn run() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TinyVM GUI",
        options,
        Box::new(|_cc| Box::new(TinyVMApp::default())),
    ).unwrap();
}

struct TinyVMApp { status: String }

impl Default for TinyVMApp {
    fn default() -> Self { Self { status: "Ready".to_owned() } }
}

impl epi::App for TinyVMApp {
    fn name(&self) -> &str { "TinyVM GUI" }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TinyVM");
            if ui.button("Load ISO").clicked() {
                self.status = "ISO Loaded!".to_owned();
            }
            ui.label(format!("Status: {}", self.status));
        });
    }
}