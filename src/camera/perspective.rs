use std::f64::consts::PI;
use crate::math::Vec3;
use crate::core::Film;

#[derive(Copy, Clone, Debug)]
pub struct PerspectiveCamera {
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
    right: Vec3
}

impl PerspectiveCamera {
    pub fn new() -> Self {
        let eye = Vec3::from(0.); 
        let look_at = Vec3::new(0.,0.,10.);
        let focus_dist = (eye - look_at).length();
        let aperture = 0.02;
        let vfov = 40.;
        let aspect = 1.;

        let theta = vfov * PI / 180.;
        let half_height = (theta * 0.5).tan();
        let half_width = half_height * aspect;

        const WORLD_UP: Vec3 = Vec3 {x: 0.,y: 1.,z: 0.};
        let forward = (eye - look_at).normalize();
        let right = Vec3::cross(WORLD_UP, forward);
        let up = Vec3::cross(forward, right);

        let lower_left = eye - half_width * focus_dist * right - half_height * focus_dist * up + focus_dist * forward;
        let horizontal = 2. * half_width * focus_dist * right;
        let vertical = 2. * half_height * focus_dist * up;

        Self {
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
        }
    }
}