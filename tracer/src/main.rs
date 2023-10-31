pub mod cameras;
pub mod core;
pub mod integrators;
pub mod loaders;
pub mod materials;
pub mod math;
pub mod shapes;
pub mod textures;

use env_logger;
use materials::MetalMaterial;

use std::{
    ffi::CString,
    num::NonZeroU32,
    time::Instant,
    env
};

use crate::core::{
    primitive::Primitive,
    primitive::ShapePrimitive,
    material::Material,
    scene::Scene,
    shape::Shape,
    spectrum::Spectrum,
    view::View,
};
use crate::materials::{
    constant::ConstantMaterial,
    matte::MatteMaterial
};
use crate::shapes::{
    mesh::Mesh,
    sphere::Sphere,
    triangle::Triangle,
};

use crate::math::vectors::Vec3;
use crate::cameras::perspective::PerspectiveCamera;
use crate::integrators::direct_lighting::DirectLightingIntegrator;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const SAMPLES_PER_PIXEL: u8 = 100;

fn gltf_scene() -> Scene
{
    let camera_position: Vec3 = Vec3::new(0.,5.,-15.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,10.);

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

    // scene.add(
    //     Primitive::Shape(Box::new(
    //         ShapePrimitive::new(
    //             Shape::Sphere(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
    //             Option::Some(
    //                 Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 0.0, 0.0))))))))));

    // scene.add(
    //     Primitive::Shape(Box::new(
    //         ShapePrimitive::new(
    //             Shape::Sphere(Sphere::new(Vec3::new(-1., 0., -1.), 0.5)),
    //             Option::Some(
    //                 Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 1.0, 0.0))))))))));

    // Floor
    scene.add(
        Primitive::Shape(Box::new(
            ShapePrimitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
                Option::None))));

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
            let material = Option::Some(Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 0.0, 0.0))))));
            let primitive = Primitive::Shape(Box::new(ShapePrimitive::new(Shape::Mesh(mesh), material)));
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
    Primitive::Shape(Box::new(
        ShapePrimitive::new(
            Shape::Sphere(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
            Option::Some(
                Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(0.5, 0.2, 0.5))))))))));

    scene.add(
        Primitive::Shape(Box::new(
            ShapePrimitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(1., 0., -1.), 0.5)),
                Option::Some(
                    Box::new(Material::Metal(MetalMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.5, 0.5))))))))));

    scene.add(
        Primitive::Shape(Box::new(
            ShapePrimitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(-1., 0., -1.), 0.5)),
                Option::Some(
                    Box::new(Material::Metal(MetalMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.5, 0.5))))))))));

    // Ground
    scene.add(
        Primitive::Shape(Box::new(
            ShapePrimitive::new(
                Shape::Sphere(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
                Option::Some(
                    Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(0.2, 0.2, 0.2))))))))));

    return scene;
}

fn furnace_test() -> Scene
{
    let reveal = false;
    let camera_position: Vec3 = Vec3::new(0.,5.,-15.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,10.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

    scene.environment_light = |_ray| -> Spectrum {
        Spectrum::ColorRGB(Vec3 { x: 0.1, y: 0.1, z: 0.1 })
    };

    // scene.environment_light = |ray| -> Spectrum {
    //     let t = 0.5 * ray.direction.y + 1.0;
    //     let sky_color = (1. - t) * Vec3::new(1.,1.,1.) + t * Vec3::new(0.5, 0.7, 1.);
    //     let sky_environment = Spectrum::ColorRGB(sky_color);
    //     return sky_environment;
    // };

    scene.add(
    Primitive::Shape(Box::new(
        ShapePrimitive::new(
            Shape::Sphere(Sphere::new(Vec3::new(0., 0., 0.), 2.)),
            Option::Some(
                Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 1.0, 1.0))))))))));

    if reveal {
        scene.add(
            Primitive::Shape(Box::new(
                ShapePrimitive::new(
                    Shape::Sphere(Sphere::new(Vec3::new(3., 0., 0.), 1.)),
                    Option::Some(
                        Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 0.0, 0.0))))))))));

        scene.add(
            Primitive::Shape(Box::new(
                ShapePrimitive::new(
                    Shape::Sphere(Sphere::new(Vec3::new(0., 3., 0.), 1.)),
                    Option::Some(
                        Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 0.0, 0.0))))))))));
    }

    return scene;
}

fn main() {


    // Set environment variables
    let key = "RUST_LOG";
    env::set_var(key, "info");

    env_logger::init();

    // Initialize scene
    let scene = raytracing_weekend_scene();

    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT, SAMPLES_PER_PIXEL);
    let mut integrator = DirectLightingIntegrator::new(scene);

    let start = Instant::now();
    integrator.render(&view);
    let duration = start.elapsed();
    println!("Render time: {:?}", duration);
}
