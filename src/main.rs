pub mod camera;
pub mod core;
pub mod integrators;
pub mod materials;
pub mod math;
pub mod ray;
pub mod shapes;

use std::{
    time::{Instant}
};

use crate::core::{
    Scene,
    View,
};
use crate::shapes::{
    Sphere,
    Triangle,
};

use crate::math::Vec3;
use crate::camera::PerspectiveCamera;
use crate::integrators::{DirectLightingIntegrator, Integrator};

fn main() {

    const SCREEN_WIDTH: u32 = 100;
    const SCREEN_HEIGHT: u32 = 100;

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
