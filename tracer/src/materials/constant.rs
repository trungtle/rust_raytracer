use crate::core::ray::Ray;
use crate::core::spectrum::Spectrum;
use crate::materials::{
    pdf::Pdf,
    pdf::UniformPdf
};
use crate::math::vectors::Vec3;

#[derive(Clone)]
pub struct ConstantMaterial {
    color: Spectrum
}

impl ConstantMaterial {
    pub fn new(color: Spectrum) -> Self {
        ConstantMaterial {
            color
        }
    }

    pub fn scatter(&self, ray: &mut Ray, attenuation: &mut Spectrum, hit_normal: &Vec3) -> bool {
        let uniform_pdf = UniformPdf::new(&hit_normal);
        ray.direction = uniform_pdf.sample_wi();
        *attenuation = self.color;
        return true;
    }
}