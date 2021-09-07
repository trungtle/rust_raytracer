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
    Spectrum
};
use crate::math::Vec3;
use crate::ray::Ray;
use crate::camera::PerspectiveCamera;
use crate::integrators::{DirectLightingIntegrator, Integrator};

fn main() {

    const SCREEN_WIDTH: u32 = 240;
    const SCREEN_HEIGHT: u32 = 240;

    let mut film = Film::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "image");

    let ray = Ray::new(Vec3::new(0., 2., -10.), Vec3::new(4.,0.,1.));
    let t = 100.;
    println!("Ray at {} is {:?}", t, ray.point_at(t));

    let cam = PerspectiveCamera::new();
    let integrator = DirectLightingIntegrator::new();
    
    for y in (0..=SCREEN_HEIGHT-1).rev() {
        for x in 0..SCREEN_WIDTH {
            let r: f64 = x as f64 / SCREEN_WIDTH as f64;
            let g: f64 = y as f64 / SCREEN_HEIGHT as f64;
            let b = 0.2;
            film.set_pixel(Spectrum::ColorRGB(Vec3::new(r, b, g)), x, y);
        }
    }

    film.write_image();
    
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
