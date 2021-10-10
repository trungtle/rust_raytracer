use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::core::spectrum::Spectrum;

#[derive(Clone)]
pub struct ConstantMaterial {
    value: Spectrum
}

impl ConstantMaterial {
    pub fn new(value: Spectrum) -> Self {
        ConstantMaterial {
            value
        }
    }
    pub fn get_value(&self) -> &Spectrum {
        &self.value
    }
}