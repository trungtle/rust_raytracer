extern crate math;
use eframe::App;
use eframe::egui::load::ImageLoader;
use math::{Vec2, Vec3};

pub mod cameras;
pub mod core;
pub mod integrators;
pub mod loaders;
pub mod materials;
pub mod shapes;
pub mod textures;

use env_logger;
use tracer::RustracerApp;

use std::f64::consts::FRAC_PI_2;
use std::f64::consts::FRAC_PI_4;
use std::f64::consts::PI;
use std::ops::Deref;
use std::{
    time::Instant,
    env,
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
use crate::materials::{
    ConstantMaterial,
    MetalMaterial,
    LambertMaterial
};
use crate::shapes::{
    mesh::Mesh,
    sphere::Sphere,
    triangle::Triangle,
};

use crate::cameras::perspective::PerspectiveCamera;
use crate::integrators::direct_lighting::DirectLightingIntegrator;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 400;
const SAMPLES_PER_PIXEL: u8 = 10;

fn pbrt4_scene() -> Scene
{
    let mut camera_position: Vec3 = Vec3::new(0.,5.5,-15.5);
    let mut camera_lookat: Vec3 = Vec3::new(0.,0.,-1.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

    let path_pbrt = "assets/pbrt4/pbrt-book/book.pbrt";
    log::info!("Loading scene: {}", &path_pbrt);
    let pbrt_scene = pbrt4::Scene::from_file(&path_pbrt).unwrap();


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
    let g_data = loaders::gltf_loader::load_gltf("assets/glTF/CesiumMilkTruck/glTF/CesiumMilkTruck.gltf");

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

fn render(view: &View, scene: &mut Scene) {
    let mut integrator = DirectLightingIntegrator::default();

    let start = Instant::now();
    integrator.render(view, scene);
    let duration = start.elapsed();
    log::info!("Render time: {:?}", duration);
}

fn test_samplers() {
    let mut film = Film::new(SCREEN_WIDTH, SCREEN_HEIGHT, "image");

    let num_pixels = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;
    let mut pixels = vec![Spectrum::ColorRGB(Vec3::from(0.0)); num_pixels];

    let num_samples = 1000;
    let color = Spectrum::ColorRGB(Vec3::new(1.0, 1.0, 1.0));
    let mut sampler = sampler::Sampler::new();
    for _i in 0..num_samples {
        // Return a point ranges from -1 to 1
        let random = sampler.random_vec2_0_1();
        let mut point = sampler::Sampler::sample_unit_disk_concentric(random);
        point = sampler.sample_from_pixel( Vec2::new( 10., 10.), SCREEN_WIDTH, SCREEN_HEIGHT);
        point.x = point.x * film.width as f64 / 4.0 + film.width as f64 / 2.0;
        point.y = point.y * film.height as f64 / 4.0 + film.height as f64 / 2.0;

        let linear_coords: usize = (point.y as u32 * film.width + point.x as u32) as usize;
        pixels[linear_coords] = color;
    }
    film.set_pixels(pixels);
    film.write_image();

    log::info!("Sampler test: {:?}", film.file_name);
}


fn init_ui(app: Box<RustracerApp>) -> Result<(), eframe::Error> {
    let mut ctx = eframe::egui::Context::default();

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
    env::set_var(key, "info");

    env_logger::init();

    let app = Box::<RustracerApp>::default();
    let ui_result = init_ui(app);
    match ui_result {
        Ok(_) => {}
        Err(err) => log::error!("Failed to create ui with error {}", err)
    }

    // Initialize scene
    let mut scene = pbrt4_scene();


    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT, SAMPLES_PER_PIXEL);
    render(&view, &mut scene);

    //app.update_image = true;
    //app.image_filepath = scene.persp_camera.film.file_path;
    //test_samplers();




}
