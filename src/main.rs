pub mod cameras;
pub mod core;
pub mod integrators;
pub mod materials;
pub mod math;
pub mod ray;
pub mod shapes;
pub mod textures;

use std::{
    time::{Instant}
};

use crate::core::{
    scene::Scene,
    view::View,
};
use crate::shapes::{
    sphere::Sphere,
    triangle::Triangle,
};

use crate::math::vectors::Vec3;
use crate::cameras::perspective::PerspectiveCamera;
use crate::integrators::direct_lighting::DirectLightingIntegrator;

fn main() {

    const SCREEN_WIDTH: u32 = 600;
    const SCREEN_HEIGHT: u32 = 600;

    // Create new camera
    let cam_eye = Vec3::new(0.,0.,-10.);
    let look_at = Vec3::new(0.,0.,10.);
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT, cam_eye, look_at);

    // Initialize scene
    let mut scene = Scene::new(cam);
    //scene.add(Box::new(Sphere::new(Vec3::new(0., 0., -5.), 0.5)));
    scene.add(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));
    scene.add(Box::new(Triangle::new(
        Vec3::new(-2., 0., -2.),
        Vec3::new(2., 0., -2.),
        Vec3::new(0., 2., -2.))));

    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut integrator = DirectLightingIntegrator::new(scene);

    let start = Instant::now();
    integrator.render(&view);
    let duration = start.elapsed();
    println!("Render time: {:?}", duration);
}
