pub mod direct_lighting;

pub use direct_lighting::DirectLightingIntegrator as DirectLightingIntegrator;
use crate::core::Spectrum;

pub trait Integrator {
    fn render() -> Spectrum;
}