pub mod cameras;
pub mod core;
pub mod integrators;
pub mod loaders;
pub mod materials;
pub mod math;
pub mod shapes;
pub mod textures;

use std::{
    time::{Instant}
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

fn main() {

    const SCREEN_WIDTH: u32 = 100;
    const SCREEN_HEIGHT: u32 = 100;

    // Create new camera
    let cam_eye = Vec3::new(0.,1.,-5.5);
    let look_at = Vec3::new(0.,0.,10.);
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, cam_eye, look_at);

    // Initialize scene
    let mut scene = Scene::new(cam);

    let g_data = loaders::gltf_loader::load_gltf("assets/glTF/Box/glTF/Box.gltf");

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


    // let mesh = Mesh::new(vec![v1, v2, v3], vec![0,1,2]);
    for mesh in g_data.doc.meshes() {
        for primitive in mesh.primitives() {
            let mesh = Mesh::from_gltf(&primitive, &g_data);
            scene.add(
                Primitive::Shape(Box::new(
                    ShapePrimitive::new(
                        Shape::Mesh(mesh),
                    Option::Some(
                        Box::new(Material::Constant(ConstantMaterial::new(Spectrum::ColorRGB(Vec3::new(1.0, 0.0, 0.0))))))))));        
        }
    }
    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut integrator = DirectLightingIntegrator::new(scene);

    let start = Instant::now();
    integrator.render(&view);
    let duration = start.elapsed();
    println!("Render time: {:?}", duration);
}
