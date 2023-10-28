use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
    pub fn r(&self) -> f64 { self.x }
    pub fn g(&self) -> f64 { self.y }
    pub fn b(&self) -> f64 { self.z }

    pub fn sqrt(v: Vec3) -> Vec3 {
        Vec3::new(
            f64::sqrt(v.x),
            f64::sqrt(v.y),
            f64::sqrt(v.z)
        )
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn length2(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn normalize(&self) -> Vec3 {
        let len_inv = 1.0 / self.length();
        Vec3 {
            x: self.x * len_inv,
            y: self.y * len_inv,
            z: self.z * len_inv
        }
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max)
        }
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y() * v2.z() - v1.z() * v2.y(),
            y: -(v1.x() * v2.z() - v1.z() * v2.x()),
            z: v1.x() * v2.y() - v1.y() * v2.x()
        }
    }
}

// ----------------------------------------------------------------------------
// Conversion from other types
// ----------------------------------------------------------------------------
impl From<f64> for Vec3 {
    fn from(item: f64) -> Self {
        Vec3 {
            x: item, y: item, z: item
        }
    }
}

// ----------------------------------------------------------------------------
// Operator overloading
// ----------------------------------------------------------------------------
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        };
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Self::Output  {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
        where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { x: 0.0, y: 0.0, z: 0.0 }, |a, b| Self {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z
        })
    }
}

// ----------------------------------------------------------------------------
// VEC2
// ----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x, y
        }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x*self.x + self.y*self.y)
    }

    pub fn length2(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }

    pub fn normalize(&self) -> Vec2 {
        let len_inv = 1.0 / self.length();
        Vec2 {
            x: self.x * len_inv,
            y: self.y * len_inv
        }
    }

    pub fn dot(v1: Vec2, v2: Vec2) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y()
    }
}

// ----------------------------------------------------------------------------
// Conversion from other types
// ----------------------------------------------------------------------------
impl From<f64> for Vec2 {
    fn from(item: f64) -> Self {
        Vec2 {
            x: item, y: item
        }
    }
}

// ----------------------------------------------------------------------------
// Operator overloading
// ----------------------------------------------------------------------------
impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

impl ops::AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other
        };
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        }
    }
}

impl ops::SubAssign<f64> for Vec2 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other
        };
    }
}

impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y
        };
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * _rhs,
            y: self.y * _rhs
        }
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * _rhs.x,
            y: self * _rhs.y
        }
    }
}

impl ops::MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other
        };
    }
}


impl ops::DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other
        };
    }
}