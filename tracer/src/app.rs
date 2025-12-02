use eframe::{
    egui::{self},
    epaint::ColorImage,
};

use math::{Float, Vec3};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use std::{
    path::{Path, PathBuf},
};
use std::{sync::Arc, time::Instant};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    core::{
        film::Film, primitive::Primitive, sampler, scene::Scene, shape::Shape, spectrum::Spectrum,
        view::View,
    },
    integrators::direct_lighting::{FrameBuffer, RenderSettings, SCREEN_HEIGHT, SCREEN_WIDTH},
};

use crate::shapes::{mesh::Mesh, sphere::Sphere};

use crate::materials::{ConstantMaterial, DieletricMaterial, LambertMaterial, MetalMaterial};

use crate::cameras::perspective::PerspectiveCamera;
use crate::integrators::direct_lighting::DirectLightingIntegrator;


fn pbrt4_scene() -> Scene {
    let camera_position: Vec3 = Vec3::new(0., 5.5, -30.5);
    let camera_lookat: Vec3 = Vec3::new(0., 0., -1.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::default();
    scene.persp_camera = cam;

    let pbrt_filename = "book.pbrt";
    let pbrt_relative_path = "assets/pbrt4/pbrt-book/";
    let pbrt_filepath = pbrt_relative_path.to_string() + pbrt_filename;
    log::info!("Loading scene: {}", &pbrt_filepath);
    let pbrt_scene = pbrt4::Scene::from_file(&pbrt_filepath).unwrap();

    println!("Global options: {:#?}", pbrt_scene.options);

    if let Some(camera) = pbrt_scene.camera {
        println!("Camera: {:#?}", camera);
        // TODO: Extract camera position from pbrt
        // match camera.params {
        //     Perspective(c) => {}
        // }
        // camera_position = camera.params.
    }

    if let Some(film) = pbrt_scene.film {
        println!("Film: {:#?}", film);
    }

    if let Some(integrator) = pbrt_scene.integrator {
        println!("Integrator: {:#?}", integrator);
    }

    if let Some(accelerator) = pbrt_scene.accelerator {
        println!("Accelerator: {:#?}", accelerator);
    }

    if let Some(sampler) = pbrt_scene.sampler {
        println!("Sampler: {:#?}", sampler);
    }

    println!("World begin");

    for texture in pbrt_scene.textures {
        println!("Texture: {:#?}", texture);
    }

    for material in pbrt_scene.materials {
        println!("Material: {:#?}", material);
    }

    for light in pbrt_scene.lights {
        println!("Light: {:#?}", light);
    }

    for medium in pbrt_scene.mediums {
        println!("Medium: {:#?}", medium);
    }

    for shape in pbrt_scene.shapes {
        println!("Shape: {:#?}", shape);
        let mut ply_path = PathBuf::from("E:/CODES/rust_raytracer/");
        ply_path.push(Path::new(pbrt_relative_path));
        let _ = match shape.params {
            pbrt4::types::Shape::PlyMesh { filename } => {
                ply_path.push(Path::new(&filename[1..filename.len() - 1]));
                let mesh = Mesh::from_ply(&ply_path);
            }
            _ => {}
        };
    }

    println!("Done");

    scene.environment_light = |ray| -> Spectrum {
        let t = 0.5 * ray.direction.y + 1.0;
        let sky_color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.);
        let sky_environment = Spectrum::ColorRGB(sky_color);
        return sky_environment;
    };

    return scene;
}

fn gltf_scene() -> Scene {
    let camera_position: Vec3 = Vec3::new(15., 2.5, 0.0);
    // let mut camera_position: Vec3 = Vec3::new(0.,25.5,10.);
    let camera_lookat: Vec3 = Vec3::new(0., 0., -1.);

    let mut scene = Scene::from("assets/glTF/CesiumMilkTruck/glTF/CesiumMilkTruck.gltf");

    scene.environment_light = |ray| -> Spectrum {
        let t = 0.5 * ray.direction.y + 1.0;
        let sky_color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.);
        let sky_environment = Spectrum::ColorRGB(sky_color);
        return sky_environment;
    };

    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    scene.persp_camera = cam;

    // Floor
    scene.add(Primitive::new(
        Shape::Sphere(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
        Option::Some(Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(
            Vec3::new(0.2, 0.2, 0.2),
        )))),
    ));

    return scene;
}

fn raytracing_weekend_scene() -> Scene {
    let camera_position: Vec3 = Vec3::new(0., 0.5, -5.5);
    let camera_lookat: Vec3 = Vec3::new(0., 0., -1.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::default();
    scene.persp_camera = cam;

    scene.environment_light = |ray| -> Spectrum {
        let t = 0.5 * ray.direction.y + 1.0;
        //let sky_color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.2, 0.2, 1.);
        //let sky_color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.2, 0.2, 0.2);
        let sky_color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.);
        let sky_environment = Spectrum::ColorRGB(sky_color);
        return sky_environment;
    };

    scene.add(Primitive::new(
        Shape::Sphere(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
        Option::Some(Arc::new(LambertMaterial::new(Spectrum::ColorRGB(
            Vec3::new(0.5, 0.5, 0.5),
        )))),
    ));

    scene.add(Primitive::new(
        Shape::Sphere(Sphere::new(Vec3::new(1., 0., -1.), 0.5)),
        Option::Some(Arc::new(MetalMaterial::new(Spectrum::ColorRGB(Vec3::new(
            0.2, 0.5, 0.5,
        ))))),
    ));

    scene.add(Primitive::new(
        Shape::Sphere(Sphere::new(Vec3::new(-1., 0., -1.), 0.5)),
        Option::Some(Arc::new(DieletricMaterial::new(Spectrum::ColorRGB(
            Vec3::new(1.0, 1.0, 1.0),
        )))),
    ));

    // Ground
    scene.add(Primitive::new(
        Shape::Sphere(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
        Option::Some(Arc::new(LambertMaterial::new(Spectrum::ColorRGB(
            Vec3::new(0.2, 0.2, 0.2),
        )))),
    ));

    return scene;
}

fn furnace_test() -> Scene {
    let reveal = true;
    let camera_position: Vec3 = Vec3::new(0., 5., -15.5);
    let camera_lookat: Vec3 = Vec3::new(0., 0., 10.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::default();
    scene.persp_camera = cam;

    scene.environment_light = |_ray| -> Spectrum {
        Spectrum::ColorRGB(Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        })
    };

    scene.add(Primitive::new(
        Shape::Sphere(Sphere::new(Vec3::zero(), 2.)),
        Option::Some(Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        })))),
    ));

    if reveal {
        scene.add(Primitive::new(
            Shape::Sphere(Sphere::new(
                Vec3 {
                    x: 3.,
                    y: 0.,
                    z: -0.5,
                },
                1.,
            )),
            Option::Some(Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3 {
                x: 0.8,
                y: 0.0,
                z: 0.0,
            })))),
        ));

        scene.add(Primitive::new(
            Shape::Sphere(Sphere::new(
                Vec3 {
                    x: 0.,
                    y: 3.,
                    z: -0.5,
                },
                1.,
            )),
            Option::Some(Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3 {
                x: 0.8,
                y: 0.1,
                z: 0.02,
            })))),
        ));
    }

    return scene;
}

// ------------------------------------------------------------
// Test sampler
// ------------------------------------------------------------
struct TestSampler {}

#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Display)]
enum SamplerTestOption {
    UnitDisk,
    UnitDiskConcentric,
}

impl TestSampler {
    fn test_samplers(width: u32, height: u32) -> Vec<Spectrum> {
        let num_pixels = width as usize * height as usize;
        let mut pixels = vec![Spectrum::ColorRGB(Vec3::from(0.0)); num_pixels];

        let num_samples = 1000;
        let color = Spectrum::ColorRGB(Vec3::from(1.0));
        for _i in 0..num_samples {
            // Return a point ranges from -1 to 1

            // Uncomment to sample from unit disk
            let random = sampler::Sampler::random_vec2_0_1();
            let mut point = sampler::Sampler::sample_unit_disk_concentric(random);
            //let mut point = sampler.sample_from_pixel( Vec2 {0: 10., 1: 10.}, width, height);
            point.0 = point.x() * width as Float / 4.0 + width as Float / 2.0;
            point.1 = point.y() * height as Float / 4.0 + height as Float / 2.0;

            let linear_coords: usize = (point.1 as u32 * width + point.0 as u32) as usize;
            pixels[linear_coords] = color;
        }

        return pixels;
    }
}

#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Display)]
enum SceneOption {
    Spheres,
    Truck,
    FurnaceTest,
    Pbrt4,
}

pub struct RustracerApp {
    name: String,
    width: u32,
    height: u32,
    rendering: bool,

    pub image: Option<Arc<ColorImage>>,
    pub image_test: Option<Arc<ColorImage>>,
    pub image_browswer: Option<Arc<ColorImage>>,

    texture: Option<egui::TextureHandle>,
    texture_test: Option<egui::TextureHandle>,
    texture_browser: Option<egui::TextureHandle>,

    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    scene_option: SceneOption,
    view: View,
    scene: Scene,
    render_settings: RenderSettings,
    framebuffer: FrameBuffer
}

impl Default for RustracerApp {
    fn default() -> Self {
        Self {
            name: "Rustracer".to_owned(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            rendering: false,
            
            image: None,
            image_test: None,
            image_browswer: None,

            texture: None,
            texture_test: None,
            texture_browser: None,

            dropped_files: Vec::new(),
            picked_path: None,
            scene_option: SceneOption::Spheres,
            view: View::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            scene: raytracing_weekend_scene(),
            render_settings: RenderSettings::default(),
            framebuffer: FrameBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT)
        }
    }
}

fn load_image_from_path(
    path: &std::path::Path,
) -> Result<eframe::egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(eframe::epaint::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn render_frame(view: &View, scene: &Scene, framebuffer: &FrameBuffer, settings: &RenderSettings) -> FrameBuffer {
    let start = Instant::now();
    let new_frame = DirectLightingIntegrator::render(&scene, &view, &framebuffer, &settings);
    //let duration = start.elapsed();
    // log::info!("Render time: {:?}", duration);

    return new_frame;
}

/// This function initializes the Rustracer application with default settings.
/// It sets up the scene, view, and render settings for the raytracer.
/// The default scene is the "raytracing_weekend_scene" which consists of spheres and a ground plane.
/// The default view is a perspective camera positioned at (0, 0.5, -5.5) and looking towards the origin.
/// The default render settings include multi-threaded rendering.
/// The function returns an instance of the RustracerApp struct.
impl eframe::App for RustracerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("file").show(ctx, |ui| {
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Opened file:");
                    ui.monospace(picked_path);
                });
                self.image_browswer = Some(Arc::new(
                    load_image_from_path(std::path::Path::new(&picked_path)).unwrap(),
                ));
            }

            if let Some(image) = self.image_browswer.take() {
                self.texture_browser =
                    Some(ctx.load_texture("image_browser", image, Default::default()));
            }

            if let Some(texture) = self.texture_browser.as_ref() {
                ui.image((texture.id(), texture.size_vec2()));
            }
        });

        egui::SidePanel::right("test_panel").show(ctx, |ui| {
            if ui.add(egui::Button::new("Test sampler")).clicked() {
                let pixels = TestSampler::test_samplers(self.width, self.height);
                // Write to film
                let mut film = Film::new(SCREEN_WIDTH, SCREEN_HEIGHT, "test samplers");
                film.set_pixels(&pixels);
                let path = film.write_image();
                log::info!("Image written to: {:?}", film.file_name);
                self.image_test = Some(Arc::new(
                    load_image_from_path(std::path::Path::new(&path)).unwrap(),
                ));
            }

            if let Some(image_test) = self.image_test.take() {
                self.texture_test = Some(ctx.load_texture("image", image_test, Default::default()));
            }

            if let Some(texture_test) = self.texture_test.as_ref() {
                ui.image((texture_test.id(), texture_test.size_vec2()));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.name.clone());
            ui.label(format!("Width {}, height: {}", self.width, self.height));
            egui::ComboBox::from_label("Select scene")
                .selected_text(format!("{:?}", self.scene_option))
                .show_ui(ui, |ui| {
                    for option in SceneOption::iter() {
                        ui.selectable_value(&mut self.scene_option, option, option.to_string());
                    }
                });

            ui.add(
                egui::Slider::new(&mut self.render_settings.sample_per_pixel, 1..=100).text("Samples per pixels"),
            );

            egui::color_picker::color_edit_button_rgb(ui, &mut self.render_settings.skycolor_tint)
                .labelled_by(ui.label("Sky color tint").id);

            ui.add(egui::Checkbox::new(
                &mut self.render_settings.single_thread,
                "Single thread",
            ));

            ui.add(egui::Checkbox::new(
                &mut self.render_settings.write_to_file,
                "Write to file",
            ));

            if ui.add(egui::Button::new("Render")).clicked() {
                // Toggle rendering
                self.rendering = !self.rendering;
                if self.rendering == true {
                    // Do we need to reinitialize the view here?
                    self.view = View::new(self.width, self.height);
                    self.scene = match self.scene_option {
                        SceneOption::Spheres => raytracing_weekend_scene(),
                        SceneOption::Truck => gltf_scene(),
                        SceneOption::FurnaceTest => furnace_test(),
                        SceneOption::Pbrt4 => pbrt4_scene(),
                    };
                    self.framebuffer = FrameBuffer::new(self.width, self.height);
                } 
            }

            if self.rendering {
                self.framebuffer.current_sample += 1;

                // RENDER!
                let new_frame = render_frame(&self.view, &self.scene, &self.framebuffer, &self.render_settings);
                self.framebuffer = new_frame;
                
                // Gamma correction
                let mut gamma_corrected_spectrum = self.framebuffer.spectrums.clone();
                gamma_corrected_spectrum.par_iter_mut().for_each(|spectrum| {
                    let Spectrum::ColorRGB(color) = *spectrum;
                    *spectrum = Spectrum::ColorRGB(Vec3::sqrt(color));
                });

                // Convert Vec<Spectrum> to Vec<u8> in RGBA format
                let mut rgba_pixels = Vec::with_capacity((SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize);
                for spectrum in &gamma_corrected_spectrum {
                    let rgb = spectrum.to_rgb(); // Assumes to_rgb() -> Vec3 in [0,1]

                    let r = (rgb.x.clamp(0.0, 1.0) * 255.0) as u8;
                    let g = (rgb.y.clamp(0.0, 1.0) * 255.0) as u8;
                    let b = (rgb.z.clamp(0.0, 1.0) * 255.0) as u8;
                    rgba_pixels.push(r);
                    rgba_pixels.push(g);
                    rgba_pixels.push(b);
                    rgba_pixels.push(255); // Alpha channel
                }
                let color_image = eframe::epaint::ColorImage::from_rgba_unmultiplied(
                    [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                    &rgba_pixels,
                );
                self.image = Some(Arc::new(color_image));

                // Uncomment to load image from file instead of rendering
                //self.image = Some(Arc::new(
                //    load_image_from_path(std::path::Path::new(&path)).unwrap(),
                //));

                if self.render_settings.write_to_file {
                    // Write to film
                    let mut film = Film::new(SCREEN_WIDTH, SCREEN_HEIGHT, "image");
                    film.set_pixels(&gamma_corrected_spectrum);
                    film.file_name = format!("render_{:?}_spp{}_{}x{}.png", self.scene_option, self.render_settings.sample_per_pixel, self.width, self.height);
                    let path = film.write_image();
                    log::info!("Image written to: {:?}", path);
                }

                // End rendering if we have accumulated enough samples or all the rays have terminated
                if self.framebuffer.current_sample >= self.render_settings.sample_per_pixel {
                    self.rendering = false;
                }

            }

            if let Some(image) = self.image.take() {
                self.texture = Some(ctx.load_texture("image", image, Default::default()));
            }

            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), texture.size_vec2()));
            }

            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        });
    }
}
