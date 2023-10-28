use std::ops;

use crate::math::vectors::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum Spectrum {
    ColorRGB(Vec3)
}

impl Spectrum {
    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        match self {
            Spectrum::ColorRGB(spectrum ) => spectrum.clamp(min, max)
        }
    }

    pub fn dot(s1: Spectrum, s2: Spectrum) -> f64 {
        match s1 {
            Spectrum::ColorRGB(v1) => {
                match s2 {
                    Spectrum::ColorRGB(v2) => {
                        Vec3::dot(v1, v2)
                    }
                }
            }
        }
    }
}

impl ops::Add<Vec3> for Spectrum {
    type Output = Spectrum;

    fn add(self, _rhs: Vec3) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.add(_rhs))
        }
    }
}

impl ops::Add<Spectrum> for Spectrum {
    type Output = Spectrum;

    fn add(self, _rhs: Spectrum) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => {
                match _rhs {
                    Spectrum::ColorRGB(_rhs) => {
                        Spectrum::ColorRGB(spectrum + _rhs)
                    }
                }
            }
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

impl ops::Div<f64> for Spectrum {
    type Output = Spectrum;

    fn div(self, _rhs: f64) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.div(_rhs))
        }
    }
}