use math::Vec3;

use crate::core::geometry::ONB;
use crate::core::sampler::Sampler;

pub trait Pdf {
    fn value(direction: &Vec3) -> f64;
    fn sample_wi(&self, sampler: &mut Sampler) -> Vec3;
}

pub struct UniformPdf {
    uvw: ONB
}

impl UniformPdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: ONB::from(w)
        }
    }
}

impl Pdf for UniformPdf {
    fn value(_direction: &Vec3) -> f64 {
        std::f64::consts::FRAC_1_PI
    }

    fn sample_wi(&self, sampler: &mut Sampler) -> Vec3 {
        // Pick a random point inside a unity sphere tangent to the xy plane,
        // then generate a new direction from it
        self.uvw.from_local(&(sampler.sample_cosine_direction()))// + Vec3::new(0.,0.,1.)))
    }
}

