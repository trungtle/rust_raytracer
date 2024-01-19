use std::f64::consts::PI;
use math::{Vec2, Vec3};

use crate::core::{
    ray::Ray,
    sampler::Sampler,
};

#[derive(Clone, Copy, PartialEq, Debug)]
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
            vertical: vertical
        }
    }

    pub fn get_ray(&self, uv: &Vec2, sampler: &mut Sampler) -> Ray {
        let rp: Vec2 = self.aperture * sampler.sample_unit_disk();
        let offset: Vec3 = self.right * rp.x + self.up * rp.y;
        Ray::new(self.eye + offset, self.lower_left + uv.x * self.horizontal + uv.y * self.vertical - self.eye - offset)
    }
}