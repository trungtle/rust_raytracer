pub mod constant;
pub mod matte;
pub mod pdf;

use crate::math::vectors::Vec3;
use crate::core::{spectrum::Spectrum, ray::Ray, interaction::SurfaceInteraction};

#[derive(Clone)]
pub struct MetalMaterial {
    pub color: Spectrum
}

impl MetalMaterial {
    pub fn new(color: Spectrum) -> Self {
        MetalMaterial {
            color
        }
    }

    pub fn scatter(&self, ray: &mut Ray, attenuation: &mut Spectrum, hit_point: &Vec3, hit_normal: &Vec3) -> bool {
        ray.origin = hit_point.clone();
        ray.direction = Vec3::reflect(ray.direction, hit_normal.clone());
        *attenuation = self.color;
        return true;
    }
}