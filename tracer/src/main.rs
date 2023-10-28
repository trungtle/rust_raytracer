pub mod cameras;
pub mod core;
pub mod integrators;
pub mod loaders;
pub mod materials;
pub mod math;
pub mod shapes;
pub mod textures;

use env_logger;

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

fn gltf_scene() -> Scene
{
    let camera_position: Vec3 = Vec3::new(0.,5.,-15.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,10.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

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

fn furnace_test() -> Scene
{
    let camera_position: Vec3 = Vec3::new(0.,5.,-15.5);
    let camera_lookat: Vec3 = Vec3::new(0.,0.,10.);

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera_position, camera_lookat);
    let mut scene = Scene::new(cam);

    scene.add(
    Primitive::Shape(Box::new(
        ShapePrimitive::new(
            Shape::Sphere(Sphere::new(Vec3::new(0., 0., 0.), 2.)),
            Option::Some(
                Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 1.0, 1.0))))))))));

    return scene;
}

fn main() {


    // Set environment variables
    let key = "RUST_LOG";
    env::set_var(key, "info");

    env_logger::init();

    // Initialize scene
    let scene = furnace_test();

    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut integrator = DirectLightingIntegrator::new(scene);

    let start = Instant::now();
    integrator.render(&view);
    let duration = start.elapsed();
    println!("Render time: {:?}", duration);
}
