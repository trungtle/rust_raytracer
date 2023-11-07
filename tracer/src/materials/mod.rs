use math::Vec3;

pub mod matte;
pub mod pdf;

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

#[derive(Clone)]
pub struct LambertMaterial {
    pub color: Spectrum
}

impl LambertMaterial {
    pub fn new(color: Spectrum) -> Self {
        LambertMaterial {
            color
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
