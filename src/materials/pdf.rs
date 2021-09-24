use crate::math::{Sampler, Vec3};
use crate::materials::ONB;

pub trait Pdf {
    fn value(direction: &Vec3) -> f64;
    fn sample_wi(&self) -> Vec3;
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

    fn sample_wi(&self) -> Vec3 {
        // Pick a random point inside a unity sphere tangle to the xy plane,
        // then generate a new direction from it
        self.uvw.from_local(&(Sampler::sample_from_unit_sphere() + Vec3::new(0.,0.,1.)))
    }
}

