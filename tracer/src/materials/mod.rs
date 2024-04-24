pub mod matte;
pub mod pdf;

use core::fmt::Debug;
use math::Float;
use math::Vec3;

use crate::materials::{pdf::Pdf, pdf::UniformPdf};

use crate::core::{
    interaction::SurfaceInteraction, ray::Ray, sampler::Sampler, spectrum::Spectrum,
};

pub trait Material: Send + Sync {
    fn value(&self) -> Spectrum;
    fn scatter(
        &self,
        ray: &mut Ray,
        attenuation: &mut Spectrum,
        interaction: &SurfaceInteraction,
        sampler: &mut Sampler,
    ) -> bool;
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
    pub color: Spectrum,
}

impl ConstantMaterial {
    pub fn new(color: Spectrum) -> Self {
        ConstantMaterial { color }
    }
}

impl Material for ConstantMaterial {
    fn value(&self) -> Spectrum {
        self.color
    }
    fn scatter(
        &self,
        ray: &mut Ray,
        attenuation: &mut Spectrum,
        interaction: &SurfaceInteraction,
        sampler: &mut Sampler,
    ) -> bool {
        let uniform_pdf = UniformPdf::new(&interaction.hit_normal);
        ray.direction = uniform_pdf.sample_wi(sampler);
        *attenuation = self.color;
        return true;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MetalMaterial {
    pub color: Spectrum,
}

impl MetalMaterial {
    pub fn new(color: Spectrum) -> Self {
        MetalMaterial { color }
    }
}

impl Material for MetalMaterial {
    fn value(&self) -> Spectrum {
        self.color
    }

    fn scatter(
        &self,
        ray: &mut Ray,
        attenuation: &mut Spectrum,
        interaction: &SurfaceInteraction,
        sampler: &mut Sampler,
    ) -> bool {
        ray.origin = interaction.hit_point.clone();
        ray.direction = Vec3::reflect(ray.direction, interaction.hit_normal.clone());
        *attenuation = self.color;
        return true;
    }
}

#[derive(Clone, Debug)]
pub struct LambertMaterial {
    pub color: Spectrum,
    pub base_color_texture: Option<image::DynamicImage>,
}

impl LambertMaterial {
    pub fn new(color: Spectrum) -> Self {
        LambertMaterial {
            color: color,
            base_color_texture: None,
        }
    }
}

impl Material for LambertMaterial {
    fn value(&self) -> Spectrum {
        self.color
    }

    fn scatter(
        &self,
        ray: &mut Ray,
        attenuation: &mut Spectrum,
        interaction: &SurfaceInteraction,
        sampler: &mut Sampler,
    ) -> bool {
        let uniform_pdf = UniformPdf::new(&interaction.hit_normal);
        ray.direction = uniform_pdf.sample_wi(sampler);
        *attenuation = self.color;
        return true;
    }
}

#[derive(Clone, Debug)]
pub struct DieletricMaterial {
    pub eta: Float,
    pub color: Spectrum,
    pub base_color_texture: Option<image::DynamicImage>,
}

impl DieletricMaterial {
    pub fn new(color: Spectrum) -> Self {
        DieletricMaterial {
            eta: 1.5,
            color: color,
            base_color_texture: None,
        }
    }
}

impl Material for DieletricMaterial {
    fn value(&self) -> Spectrum {
        self.color
    }

    fn scatter(
        &self,
        ray: &mut Ray,
        attenuation: &mut Spectrum,
        interaction: &SurfaceInteraction,
        sampler: &mut Sampler,
    ) -> bool {
        ray.origin = interaction.hit_point.clone() + sampler.sample_from_unit_sphere() * 0.1;

        let ir = 1.5;
        let refraction_ratio = if interaction.hit_front_face {
            1.0 / ir
        } else {
            ir
        };
        let normal = if interaction.hit_front_face {
            interaction.hit_normal.clone()
        } else {
            -interaction.hit_normal.clone()
        };
        //ray.direction = Vec3::refract(ray.direction, normal, refraction_ratio);
        ray.direction = ray.direction + sampler.sample_from_unit_sphere() * 0.1;
        ray.direction = ray.direction.normalize();
        *attenuation = Spectrum::ColorRGB(Vec3::from(1.0)) * *attenuation;
        return true;
    }
}
