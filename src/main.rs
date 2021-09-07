use std::{
    thread,
    time::Duration,
};

mod core;
mod math;
mod ray;
mod camera;
mod integrators;

use crate::core::{
    Film, 
    Spectrum,
    View,
};
use crate::math::Vec3;
use crate::ray::Ray;
use crate::camera::PerspectiveCamera;
use crate::integrators::{DirectLightingIntegrator, Integrator, direct_lighting::Scene};

fn main() {

    const SCREEN_WIDTH: u32 = 240;
    const SCREEN_HEIGHT: u32 = 240;

    let cam = PerspectiveCamera::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let scene = Scene::new(cam);
    let view = View::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut integrator = DirectLightingIntegrator::new(scene);
    integrator.render(&view);    
}
