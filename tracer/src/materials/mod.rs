
pub mod matte;
pub mod pdf;

use core::fmt::Debug;
use math::Vec3;

use crate::materials::{
    pdf::Pdf,
    pdf::UniformPdf
};

use crate::core::{spectrum::Spectrum, ray::Ray, interaction::SurfaceInteraction, sampler:: Sampler};

pub trait Material: Send + Sync {
    fn value(&self) -> Spectrum;
    fn scatter(&self,
        ray: &mut Ray, attenuation: &mut Spectrum, hit_point: &Vec3, hit_normal: &Vec3, sampler: &mut Sampler) -> bool;
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spectrum{{{:?}}}", self.value())
    }
}

impl PartialEq for dyn Material {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ConstantMaterial {
    pub color: Spectrum
}

impl ConstantMaterial {
    pub fn new(color: Spectrum) -> Self {
        ConstantMaterial {
            color
        }
    }
}

impl Material for ConstantMaterial {
    fn value(&self) -> Spectrum {
        self.color
    }
    fn scatter(&self,
        ray: &mut Ray, attenuation: &mut Spectrum, hit_point: &Vec3, hit_normal: &Vec3, sampler: &mut Sampler) -> bool {
        let uniform_pdf = UniformPdf::new(&hit_normal);
        ray.direction = uniform_pdf.sample_wi(sampler);
        *attenuation = self.color;
        return true;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MetalMaterial {
    pub color: Spectrum
}

impl MetalMaterial {
    pub fn new(color: Spectrum) -> Self {
        MetalMaterial {
            color
        }
    }
}

impl Material for MetalMaterial {
    fn value(&self) -> Spectrum {
        self.color
    }

    fn scatter(&self,
        ray: &mut Ray, attenuation: &mut Spectrum, hit_point: &Vec3, hit_normal: &Vec3, sampler: &mut Sampler) -> bool {
        ray.origin = hit_point.clone();
        ray.direction = Vec3::reflect(ray.direction, hit_normal.clone());
        *attenuation = self.color;
        return true;
    }
}

#[derive(Clone, Debug)]
pub struct LambertMaterial {
    pub color: Spectrum,
    pub base_color_texture: Option<image::DynamicImage>
}

impl LambertMaterial {
    pub fn new(color: Spectrum) -> Self {
        LambertMaterial {
            color: color,
            base_color_texture: None
        }
    }
}

impl Material for LambertMaterial {
    fn value(&self) -> Spectrum {
        self.color
    }

    fn scatter(&self,
        ray: &mut Ray, attenuation: &mut Spectrum, hit_point: &Vec3, hit_normal: &Vec3, sampler: &mut Sampler) -> bool {
        ray.origin = hit_point.clone();
        ray.direction = Vec3::reflect(ray.direction, hit_normal.clone());
        *attenuation = self.color;
        return true;
    }
}
