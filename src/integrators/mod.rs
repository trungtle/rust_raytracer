pub mod direct_lighting;

pub use direct_lighting::DirectLightingIntegrator as DirectLightingIntegrator;

use crate::core::Spectrum;
use crate::core::View;

pub trait Integrator {
    fn render(&mut self, view: &View) -> Spectrum;
}