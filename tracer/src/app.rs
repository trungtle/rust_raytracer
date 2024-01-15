use eframe::egui;

pub struct RustracerApp {
    name: String,
    width: f32,
    height: f32,
    pub update_image: bool,
    pub image_filepath: String
}

impl Default for RustracerApp {
    fn default() -> Self {
        Self {
            name: "Rustracer".to_owned(),
            width: 400.0,
            height: 400.0,
            update_image: true,
            image_filepath: "../../output/image-28-Oct-2023-22-28-03.png".to_owned()
        }
    }
}

impl eframe::App for RustracerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.name.clone());
            ui.label(format!("Hello '{}', width {}, height: {}", self.name, self.width, self.height));
            if self.update_image {
                ui.add(
                    egui::Image::new(self.image_filepath.clone()).rounding(10.0),
                );
            }
        });
    }
}
