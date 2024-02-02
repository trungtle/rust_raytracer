use env_logger;
use tracer::RustracerApp;

fn init_ui(app: Box<RustracerApp>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rustracer",
        options,
        Box::new(|creation_context| {
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            app
        }),
    )
}


fn main() {
    // Set environment variables
    let key = "RUST_LOG";
    std::env::set_var(key, "info");

    env_logger::init();

    let app = Box::<RustracerApp>::default();
    let ui_result = init_ui(app);
    match ui_result {
        Ok(_) => {}
        Err(err) => log::error!("Failed to create app with error {}", err)
    }


}
