use eframe::{egui::{self}, epaint::ColorImage};

use math::{Vec2, Vec3};

use std::{f64::consts::FRAC_PI_2, path::Path};
use std::{
    time::Instant,
    sync::Arc
};

use crate::core::{
    film::Film,
    primitive::Primitive,
    sampler,
    scene::Scene,
    shape::Shape,
    spectrum::Spectrum,
    transform::Transform,
    view::View
};

use crate::shapes::{
    mesh::Mesh,
    sphere::Sphere
};

use crate::materials::{
    ConstantMaterial,
    MetalMaterial,
    LambertMaterial
};

use crate::cameras::perspective::PerspectiveCamera;
use crate::integrators::direct_lighting::DirectLightingIntegrator;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 400;
const SAMPLES_PER_PIXEL: u8 = 10;


fn pbrt4_scene() -> Scene
{
    use mesh_loader;

    let mut camera_position: Vec3 = Vec3::new(0.,5.5,-15.5);
    let mut camera_lookat: Vec3 = Vec3::new(0.,0.,-1.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

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
        let mut ply_path = project_root::get_project_root().unwrap();
        ply_path.push(Path::new(pbrt_relative_path));
        let _ = match shape.params {
            pbrt4::types::Shape::PlyMesh { filename } => {
                ply_path.push(Path::new(&filename[1..filename.len()-1]));
                let mesh = Mesh::from_ply(&ply_path);
            },
            _ => {}
        };
    }

    println!("Done");

    scene.environment_light = |ray| -> Spectrum {
        let t = 0.5 * ray.direction.y + 1.0;
        let sky_color = (1. - t) * Vec3::new(1.,1.,1.) + t * Vec3::new(0.5, 0.7, 1.);
        let sky_environment = Spectrum::ColorRGB(sky_color);
        return sky_environment;
    };

    // for shape_entity in pbrt_scene.shapes {
    //     let shape = shape_entity.params;
    //     let mesh = Mesh::from_gltf(g_primitive, g_data)
    //     let mut primitive = Primitive::new(Shape::Mesh(mesh), Option::Some(Arc::new(LambertMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.5, 0.5))))));
    //     let mut transform = Transform::default();
    //     // transform = transform * Transform::scale(Vec3 { x: 3.0, y: 3.0, z: 3.0 });
    //     primitive.apply_transform(transform);
    //     scene.add(primitive);
    // }

    return scene;
}

fn gltf_scene() -> Scene
{
    let camera_position: Vec3 = Vec3::new(0.,5.5,-15.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,-1.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

    scene.environment_light = |ray| -> Spectrum {
        let t = 0.5 * ray.direction.y + 1.0;
        let sky_color = (1. - t) * Vec3::new(1.,1.,1.) + t * Vec3::new(0.5, 0.7, 1.);
        let sky_environment = Spectrum::ColorRGB(sky_color);
        return sky_environment;
    };

    //let g_data = loaders::gltf_loader::load_gltf("assets/glTF/Box/glTF/Box.gltf");
    let g_data = crate::loaders::gltf_loader::load_gltf("assets/glTF/CesiumMilkTruck/glTF/CesiumMilkTruck.gltf");

    // Floor
    scene.add(
        Primitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
                Option::Some(
                    Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.2, 0.2)))))));

    for mesh in g_data.doc.meshes() {
        for primitive in mesh.primitives() {
            let mesh = Mesh::from_gltf(&primitive, &g_data);
            let mut primitive = Primitive::new(Shape::Mesh(mesh), Option::Some(Arc::new(LambertMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.5, 0.5))))));
            //primitive.apply_transform(Transform::translate(Vec3 { x: 0.0, y: 1.0, z: 0.0 }));
            //primitive.apply_transform(Transform::rotate_x(PI));
            let mut transform = Transform::rotate_x(FRAC_PI_2);
            // transform = transform * Transform::scale(Vec3 { x: 3.0, y: 3.0, z: 3.0 });
            primitive.apply_transform(transform);
            scene.add(primitive);
        }
    }

    return scene;
}

fn raytracing_weekend_scene() -> Scene
{
    let camera_position: Vec3 = Vec3::new(0.,0.5,-5.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,-1.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

    scene.environment_light = |ray| -> Spectrum {
        let t = 0.5 * ray.direction.y + 1.0;
        let sky_color = (1. - t) * Vec3::new(1.,1.,1.) + t * Vec3::new(0.5, 0.7, 1.);
        let sky_environment = Spectrum::ColorRGB(sky_color);
        return sky_environment;
    };

    scene.add(
    Primitive::new(
            Shape::Sphere(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
            Option::Some(
                Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(0.5, 0.2, 0.5)))))));

    scene.add(
        Primitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(1., 0., -1.), 0.5)),
                Option::Some(
                    Arc::new(MetalMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.5, 0.5)))))));

    scene.add(
        Primitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(-1., 0., -1.), 0.5)),
                Option::Some(
                    Arc::new(MetalMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.5, 0.5)))))));

    // Ground
    scene.add(
        Primitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
                Option::Some(
                    Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.2, 0.2)))))));

    return scene;
}

fn furnace_test() -> Scene
{
    let reveal = true;
    let camera_position: Vec3 = Vec3::new(0.,5.,-15.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,10.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

    scene.environment_light = |_ray| -> Spectrum {
        Spectrum::ColorRGB(Vec3 { x: 0.5, y: 0.5, z: 0.5 })
    };

    // scene.environment_light = |ray| -> Spectrum {
    //     let t = 0.5 * ray.direction.y + 1.0;
    //     let sky_color = (1. - t) * Vec3::new(1.,1.,1.) + t * Vec3::new(0.5, 0.7, 1.);
    //     let sky_environment = Spectrum::ColorRGB(sky_color);
    //     return sky_environment;
    // };

    scene.add(
        Primitive::new(
            Shape::Sphere(Sphere::new(Vec3::new(0., 0., 0.), 2.)),
            Option::Some(
                Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 1.0, 1.0)))))));

    if reveal {
        scene.add(
            Primitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(3., 0., -0.5), 1.)),
                Option::Some(
                    Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(0.8, 0.0, 0.0)))))));

        scene.add(
            Primitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(0., 3., -0.5), 1.)),
                Option::Some(
                    Arc::new(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(0.8, 0.1, 0.02)))))));
    }

    return scene;
}

fn render(view: View, scene: Scene) -> Vec<Spectrum> {
    let mut integrator = DirectLightingIntegrator::default();

    let start = Instant::now();
    let pixels = integrator.render(scene, view);
    let duration = start.elapsed();
    log::info!("Render time: {:?}", duration);

    return pixels;
}

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Display)]
enum SceneOption {
    Spheres,
    Truck,
    FurnaceTest,
    Pbrt4
}

pub struct RustracerApp {
    name: String,
    width: u32,
    height: u32,
    sample_per_pixel: u8,
    texture: Option<egui::TextureHandle>,
    pub image: Option<Arc<ColorImage>>,
    scene_option: SceneOption,
    view: View,
    scene: Scene
}

impl Default for RustracerApp {
    fn default() -> Self {
        Self {
            name: "Rustracer".to_owned(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            sample_per_pixel: SAMPLES_PER_PIXEL,
            texture: None,
            image: None,
            scene_option: SceneOption::Spheres,
            view: View::new(SCREEN_WIDTH, SCREEN_HEIGHT, SAMPLES_PER_PIXEL),
            scene: raytracing_weekend_scene()
        }
    }
}

fn test_samplers(width: u32, height: u32) -> Vec<Spectrum> {
    let num_pixels = width as usize * height as usize;
    let mut pixels = vec![Spectrum::ColorRGB(Vec3::from(0.0)); num_pixels];

    let num_samples = 1000;
    let color = Spectrum::ColorRGB(Vec3::new(1.0, 1.0, 1.0));
    let mut sampler = sampler::Sampler::new();
    for _i in 0..num_samples {
        // Return a point ranges from -1 to 1

        // Uncomment to sample from unit disk
        // let random = sampler.random_vec2_0_1();
        // let mut point = sampler::Sampler::sample_unit_disk_concentric(random);
        let mut point = sampler.sample_from_pixel( Vec2::new( 10., 10.), width, height);
        point.x = point.x * width as f64 / 4.0 + width as f64 / 2.0;
        point.y = point.y * height as f64 / 4.0 + height as f64 / 2.0;

        let linear_coords: usize = (point.y as u32 * width + point.x as u32) as usize;
        pixels[linear_coords] = color;
    }

    return pixels;
}


fn load_image_from_path(path: &std::path::Path) -> Result<eframe::egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(eframe::epaint::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl eframe::App for RustracerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.name.clone());
            ui.label(format!("Width {}, height: {}", self.width, self.height));
            egui::ComboBox::from_label("Select scene")
                .selected_text(format!("{:?}", self.scene_option))
                .show_ui(ui, |ui| {
                    for option in SceneOption::iter() {
                        ui.selectable_value(&mut self.scene_option, option, option.to_string());
                    }
                }
            );

            ui.add(egui::Slider::new(&mut self.sample_per_pixel, 1..=100).text("Samples per pixels"));

            if ui.add(egui::Button::new("Render")).clicked() {
                self.view = View::new(self.width, self.height, self.sample_per_pixel);
                self.scene = match self.scene_option {
                    SceneOption::Spheres => { raytracing_weekend_scene() },
                    SceneOption::Truck => { gltf_scene() },
                    SceneOption::FurnaceTest => { furnace_test() },
                    SceneOption::Pbrt4 => { pbrt4_scene() }
                };

                let pixels = render(self.view, self.scene.clone());
                //let pixels = test_samplers();
                // Write to film
                let mut film = Film::new(SCREEN_WIDTH, SCREEN_HEIGHT, "image");
                film.set_pixels(pixels);
                let path = film.write_image();
                log::info!("Image written to: {:?}", film.file_name);
                self.image = Some(Arc::new(load_image_from_path(std::path::Path::new(&path)).unwrap()));
            }

            if let Some(image) = self.image.take() {
                self.texture = Some(ctx.load_texture("image", image, Default::default()));
            }


            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), texture.size_vec2()));
            }
        });
    }
}
