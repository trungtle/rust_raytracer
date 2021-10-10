use std::ops;

use crate::math::vectors::Vec3;

#[derive(Clone, Debug)]
pub enum Spectrum {
    ColorRGB(Vec3)
}

impl Spectrum {
    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        match self {
            Spectrum::ColorRGB(spectrum ) => spectrum.clamp(min, max)
        }
    }
}

impl ops::Mul<Vec3> for Spectrum {
    type Output = Spectrum;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.mul(_rhs))
        }
    }
}

impl ops::Mul<Spectrum> for Spectrum {
    type Output = Spectrum;

    fn mul(self, _rhs: Spectrum) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => {
                match _rhs {
                    Spectrum::ColorRGB(_rhs) => {
                        Spectrum::ColorRGB(spectrum * _rhs)
                    }
                }
            }
        }
    }
}

impl ops::Mul<f64> for Spectrum {
    type Output = Spectrum;

    fn mul(self, _rhs: f64) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.mul(_rhs))
        }
    }
}

impl ops::Mul<Spectrum> for f64 {
    type Output = Spectrum;

    fn mul(self, _rhs: Spectrum) -> Self::Output {
        match _rhs {
            Spectrum::ColorRGB(_rhs) => Spectrum::ColorRGB(self * _rhs)
        }
    }
}