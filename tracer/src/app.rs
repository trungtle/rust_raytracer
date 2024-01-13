use eframe::egui;

pub struct RustracerApp {
    name: String,
    width: f32,
    height: f32
}

impl Default for RustracerApp {
    fn default() -> Self {
        Self {
            name: "Rustracer".to_owned(),
            width: 400.0,
            height: 400.0
        }
    }
}

impl eframe::App for RustracerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.name.clone());
            ui.label(format!("Hello '{}', width {}", self.name, self.width));
            ui.image(egui::include_image!("../../output/image-12-Jan-2024-23-28-34.png"));

        });
    }
}
