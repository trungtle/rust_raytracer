use std::ops;
use std::ops::{Index, IndexMut};
use crate::types::Float;
use super::tuple::{Tuple2d, Tuple3d, Tuple4d};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Vector {
    Vector2d(Tuple2d),
    Vector3d(Tuple3d),
    Vector4d(Tuple4d)
}

// pub fn sqrt(v: Vector) -> Vector {

// }

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0., z: 0. }
    }

    pub fn x(&self) -> Float { self.x }
    pub fn y(&self) -> Float { self.y }
    pub fn z(&self) -> Float { self.z }
    pub fn r(&self) -> Float { self.x }
    pub fn g(&self) -> Float { self.y }
    pub fn b(&self) -> Float { self.z }

    pub fn sqrt(v: Vec3) -> Vec3 {
        Vec3::new(
            Float::sqrt(v.x),
            Float::sqrt(v.y),
            Float::sqrt(v.z)
        )
    }

    pub fn length(&self) -> Float {
        Float::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn length2(&self) -> Float {
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

    pub fn clamp(&self, min: Float, max: Float) -> Vec3 {
        Vec3 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max)
        }
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> Float {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: v1.y() * v2.z() - v1.z() * v2.y(),
            y: -(v1.x() * v2.z() - v1.z() * v2.x()),
            z: v1.x() * v2.y() - v1.y() * v2.x()
        }
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - 2. * Vec3::dot(v, n) * n;
    }
}

// ----------------------------------------------------------------------------
// Conversion from other types
// ----------------------------------------------------------------------------
impl From<Float> for Vec3 {
    fn from(item: Float) -> Self {
        Vec3 {
            x: item, y: item, z: item
        }
    }
}

// ----------------------------------------------------------------------------
// Operator overloading
// ----------------------------------------------------------------------------
impl Index<usize> for Vec3 {
    type Output = Float;
    fn index<'a>(&'a self, i: usize) -> &Float {
        if i == 0 {
            return &self.x;
        } else if i == 1{
            return &self.y;
        } else {
            return &self.z;
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&mut self, i: usize) -> &mut Self::Output {
        if i == 0 {
            return &mut self.x;
        } else if i == 1{
            return &mut self.y;
        } else {
            return &mut self.z;
        }
    }
}

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

impl ops::AddAssign<Float> for Vec3 {
    fn add_assign(&mut self, other: Float) {
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

impl ops::SubAssign<Float> for Vec3 {
    fn sub_assign(&mut self, other: Float) {
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

impl ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Float) -> Self::Output {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for Float {
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

impl ops::MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, other: Float) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Float) -> Self::Output  {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, other: Float) {
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
    pub x: Float,
    pub y: Float
}

impl Vec2 {
    pub fn new(x: Float, y: Float) -> Self {
        Self {
            x, y
        }
    }

    pub fn x(&self) -> Float { self.x }
    pub fn y(&self) -> Float { self.y }

    pub fn length(&self) -> Float {
        Float::sqrt(self.x*self.x + self.y*self.y)
    }

    pub fn length2(&self) -> Float {
        self.x*self.x + self.y*self.y
    }

    pub fn normalize(&self) -> Vec2 {
        let len_inv = 1.0 / self.length();
        Vec2 {
            x: self.x * len_inv,
            y: self.y * len_inv
        }
    }

    pub fn dot(v1: Vec2, v2: Vec2) -> Float {
        v1.x() * v2.x() + v1.y() * v2.y()
    }
}

// ----------------------------------------------------------------------------
// Conversion from other types
// ----------------------------------------------------------------------------
impl From<Float> for Vec2 {
    fn from(item: Float) -> Self {
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

impl ops::AddAssign<Float> for Vec2 {
    fn add_assign(&mut self, other: Float) {
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

impl ops::SubAssign<Float> for Vec2 {
    fn sub_assign(&mut self, other: Float) {
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

impl ops::Mul<Float> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: Float) -> Self::Output {
        Vec2 {
            x: self.x * _rhs,
            y: self.y * _rhs
        }
    }
}

impl ops::Mul<Vec2> for Float {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * _rhs.x,
            y: self * _rhs.y
        }
    }
}

impl ops::MulAssign<Float> for Vec2 {
    fn mul_assign(&mut self, other: Float) {
        *self = Self {
            x: self.x * other,
            y: self.y * other
        };
    }
}


impl ops::DivAssign<Float> for Vec2 {
    fn div_assign(&mut self, other: Float) {
        *self = Self {
            x: self.x / other,
            y: self.y / other
        };
    }
}