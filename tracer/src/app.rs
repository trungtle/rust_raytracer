use std::sync::Arc;
use eframe::{egui::{self, ImageSource, TextureOptions, SizeHint, Image, load::ImageLoader}, epaint::ColorImage};
pub struct RustracerApp {
    name: String,
    width: f32,
    height: f32,
    texture: Option<egui::TextureHandle>,
    pub image: Option<Arc<ColorImage>>,
}

impl Default for RustracerApp {
    fn default() -> Self {
        Self {
            name: "Rustracer".to_owned(),
            width: 400.0,
            height: 400.0,
            texture: None,
            image: None,
        }
    }
}

impl eframe::App for RustracerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.name.clone());
            ui.label(format!("Hello '{}', width {}, height: {}", self.name, self.width, self.height));
            if let Some(image) = self.image.take() {
                self.texture = Some(ctx.load_texture("image", image, Default::default()));
            }


            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), texture.size_vec2()));
            }
        });
    }
}
