use eframe::egui;

pub struct App {
    name: String
}

impl std::default::Default for App {
    fn default() -> Self {
        Self {
            name: "Default App".to_owned()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My app");
        });
    }
}