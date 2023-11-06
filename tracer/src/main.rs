pub mod cameras;
pub mod core;
pub mod integrators;
pub mod loaders;
pub mod materials;
pub mod math;
pub mod shapes;
pub mod textures;

use env_logger;
use tracer::App;

use std::{
    time::Instant,
    env,
    sync::Arc
};

use crate::math::vectors:: {
    Vec2,
    Vec3
};
use crate::core::sampler;
use crate::core::{
    film::Film,
    primitive::Primitive,
    scene::Scene,
    shape::Shape,
    spectrum::Spectrum,
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

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const SAMPLES_PER_PIXEL: u8 = 100;

fn pbrt4_scene() -> Scene
{
    let camera_position: Vec3 = Vec3::new(0.,5.5,-15.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,-1.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

    let path = "assets/pbrt4/pbrt-book/book.pbrt";
    log::info!("Loading scene: {}", &path);
    let pbrt_scene = pbrt4::Scene::from_file(&path).unwrap();

    for shape in pbrt_scene.shapes {
        log::info!("Shape {:#?}", shape);
    }

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

    // let g_data = loaders::gltf_loader::load_gltf("assets/glTF/Box/glTF/Box.gltf");
    let g_data = loaders::gltf_loader::load_gltf("assets/glTF/CesiumMilkTruck/glTF/CesiumMilkTruck.gltf");

    // Floor
    scene.add(
        Primitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
                Option::None));

    // scene.add(
    //     Primitive::Shape(Box::new(
    //         ShapePrimitive::new(
    //             Shape::Triangle(
    //                 Triangle::new(
    //                     Vec3::new(-1., 0., -1.),
    //                     Vec3::new(1., 0., -1.),
    //                     Vec3::new(0., 1., -1.))),
    //         Option::Some(
    //             Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 1.0, 0.0))))))))));


    for mesh in g_data.doc.meshes() {
        for primitive in mesh.primitives() {
            let mesh = Mesh::from_gltf(&primitive, &g_data);
            let primitive = Primitive::new(Shape::Mesh(mesh), Option::Some(Arc::new(MetalMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.5, 0.5))))));
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

fn render() {
    // Initialize scene
    let scene = raytracing_weekend_scene();

    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT, SAMPLES_PER_PIXEL);

    let mut integrator = DirectLightingIntegrator::new(scene);

    let start = Instant::now();
    integrator.render(&view);
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
        point.x = point.x * film.width as f64 / 4.0 + film.width as f64 / 2.0;
        point.y = point.y * film.height as f64 / 4.0 + film.height as f64 / 2.0;

        let linear_coords: usize = (point.y as u32 * film.width + point.x as u32) as usize;
        pixels[linear_coords] = color;
    }
    film.set_pixels(pixels);
    film.write_image();

    log::info!("Sampler test: {:?}", film.file_name);
}

fn init_ui() -> Result<(), eframe::Error> {
    let mut ctx = egui::Context::default();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2 { x: 320.0, y: 240.0 }),
        ..Default::default()
    };

    eframe::run_native(
        "My app",
        options,
        Box::new(|creation_context| {
            Box::<App>::default()
        }),
    )
}

fn main() {


    // Set environment variables
    let key = "RUST_LOG";
    env::set_var(key, "info");

    env_logger::init();

    let ui_result = init_ui();
    match ui_result {
        Ok(_) => {}
        Err(err) => log::error!("Failed to create ui with error {}", err)
    }

    render();
    //test_samplers();
}
