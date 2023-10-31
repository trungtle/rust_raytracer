use std::f64::consts::PI;
use crate::core::{
    film::Film,
    ray::Ray,
    sampler::Sampler,
    spectrum::Spectrum,
};
use crate::math::vectors::{
    Vec2,
    Vec3
};

#[derive(Clone)]
pub struct PerspectiveCamera {
    width: u32, height: u32,
    eye: Vec3,
    look_at: Vec3,
    vfov: f64,
    aspect: f64,
    aperture: f64,
    focus_dist: f64,

    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    up: Vec3,
    forward: Vec3,
    right: Vec3,
    film: Film,
}

impl PerspectiveCamera {
    pub fn new(width: u32, height: u32, eye: Vec3, look_at: Vec3) -> Self {
        let eye = eye;
        let focus_dist = (eye - look_at).length();
        let aperture = 0.02;
        let vfov = 40.;
        let aspect = 1.;

        let theta = vfov * PI / 180.;
        let half_height = (theta * 0.5).tan();
        let half_width = half_height * aspect;

        const WORLD_UP: Vec3 = Vec3 {x: 0.,y: 1.,z: 0.};
        let forward = (look_at - eye).normalize();
        let right = Vec3::cross(WORLD_UP, forward);
        let up = Vec3::cross(forward, right);

        let lower_left = eye - half_width * focus_dist * right - half_height * focus_dist * up + focus_dist * forward;
        let horizontal = 2. * half_width * focus_dist * right;
        let vertical = 2. * half_height * focus_dist * up;

        Self {
            width, height,
            eye: eye,
            look_at: look_at,
            vfov: vfov,
            aspect: aspect,
            aperture: aperture,
            focus_dist: focus_dist,
            forward: forward,
            right: right,
            up: up,
            lower_left: lower_left,
            horizontal: horizontal,
            vertical: vertical,
            film: Film::new(width, height, "image"),
        }
    }

    pub fn get_ray(&self, uv: &Vec2) -> Ray {
        let rp: Vec2 = self.aperture * Sampler::sample_from_unit_disk();
        let offset: Vec3 = self.right * rp.x + self.up * rp.y;
        Ray::new(self.eye + offset, self.lower_left + uv.x * self.horizontal + uv.y * self.vertical - self.eye - offset)
    }

    pub fn set_pixel(&mut self, x:u32, y:u32, color: Spectrum) {
        self.film.set_pixel(x, y, color);
    }

    pub fn set_pixels(&mut self, pixels: Vec<Spectrum> ) {
        self.film.set_pixels(pixels);
    }

    pub fn write_film_to_file(&mut self) {
        self.film.write_image();
    }
}