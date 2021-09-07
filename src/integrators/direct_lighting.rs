use crate::integrators::Integrator;
use crate::math::Vec3;
use crate::core::Spectrum;

#[derive(Copy, Clone, Debug)]
pub struct DirectLightingIntegrator {

}

impl DirectLightingIntegrator {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl Integrator for DirectLightingIntegrator {
    fn render() -> Spectrum {
        Spectrum::ColorRGB(Vec3::from(0.))
    }
}