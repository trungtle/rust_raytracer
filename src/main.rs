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
    Sphere
};

use crate::math::Vec3;
use crate::camera::PerspectiveCamera;
use crate::integrators::{DirectLightingIntegrator, Integrator};

fn main() {

    const SCREEN_WIDTH: u32 = 800;
    const SCREEN_HEIGHT: u32 = 800;

    // Create new camera
    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Initialize scene
    let mut scene = Scene::new(cam);
    scene.add(Box::new(Sphere::new(Vec3::new(0., -0., -5.), 0.5)));
    scene.add(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut integrator = DirectLightingIntegrator::new(scene);

    let start = Instant::now();
    integrator.render(&view);
    let duration = start.elapsed();
    println!("Render time: {:?}", duration);
}
