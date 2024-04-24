use math::{Float, Vec3};
use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Spectrum {
    ColorRGB(Vec3),
}

impl Default for Spectrum {
    fn default() -> Self {
        Spectrum::ColorRGB(Vec3::default())
    }
}

impl Spectrum {
    pub fn clamp(&self, min: Float, max: Float) -> Vec3 {
        match self {
            Spectrum::ColorRGB(spectrum) => spectrum.clamp(min, max),
        }
    }

    pub fn dot(s1: Spectrum, s2: Spectrum) -> Float {
        match s1 {
            Spectrum::ColorRGB(v1) => match s2 {
                Spectrum::ColorRGB(v2) => Vec3::dot(v1, v2),
            },
        }
    }
}

impl ops::Add<Vec3> for Spectrum {
    type Output = Spectrum;

    fn add(self, _rhs: Vec3) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.add(_rhs)),
        }
    }
}

impl ops::Add<Spectrum> for Spectrum {
    type Output = Spectrum;

    fn add(self, _rhs: Spectrum) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => match _rhs {
                Spectrum::ColorRGB(_rhs) => Spectrum::ColorRGB(spectrum + _rhs),
            },
        }
    }
}

impl ops::Mul<Vec3> for Spectrum {
    type Output = Spectrum;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.mul(_rhs)),
        }
    }
}

impl ops::Mul<Spectrum> for Spectrum {
    type Output = Spectrum;

    fn mul(self, _rhs: Spectrum) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => match _rhs {
                Spectrum::ColorRGB(_rhs) => Spectrum::ColorRGB(spectrum * _rhs),
            },
        }
    }
}

impl ops::Mul<Float> for Spectrum {
    type Output = Spectrum;

    fn mul(self, _rhs: Float) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.mul(_rhs)),
        }
    }
}

impl ops::Mul<Spectrum> for Float {
    type Output = Spectrum;

    fn mul(self, _rhs: Spectrum) -> Self::Output {
        match _rhs {
            Spectrum::ColorRGB(_rhs) => Spectrum::ColorRGB(self * _rhs),
        }
    }
}

impl ops::Div<Float> for Spectrum {
    type Output = Spectrum;

    fn div(self, _rhs: Float) -> Self::Output {
        match self {
            Spectrum::ColorRGB(spectrum) => Spectrum::ColorRGB(spectrum.div(_rhs)),
        }
    }
}
