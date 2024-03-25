use std::ops::{self, Index, IndexMut};

use crate::Float;

use crate::{Floating, Numeric};
use crate::tuple::{Tuple2d, Tuple3d, Tuple4d};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Vector {
    V2(Tuple2d),
    V3(Tuple3d),
    V4(Tuple4d)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3<T>
    where T: Numeric {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Default for Vector3<T>
    where T: Numeric {
    fn default() -> Self {
        Vector3::zero()
    }
}

impl<T> From<T> for Vector3<T> 
    where T: Numeric {
    fn from(value: T) -> Self {
        Vector3 {
            x: value,
            y: value,
            z: value
        }
    }
}

impl<T> From<&[T; 3]> for Vector3<T> 
    where T: Numeric {
    fn from(value: &[T; 3]) -> Self {
        Vector3 {
            x: value[0],
            y: value[1],
            z: value[2]
        }
    }
}


impl<T> Vector3<T>
    where T: Numeric {    
    pub fn zero() -> Self {
        Self { x: T::default(), y: T::default(), z: T::default() }
    }

    pub fn x(&self) -> T { self.x }
    pub fn y(&self) -> T { self.y }
    pub fn z(&self) -> T { self.z }
    pub fn r(&self) -> T { self.x }
    pub fn g(&self) -> T { self.y }
    pub fn b(&self) -> T { self.z }
}

impl<T> Vector3<T>
    where T: Floating {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn min3(&self) -> T {
        self.x.min(self.y.min(self.z))
    }

    pub fn sqrt(v: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: T::sqrt(v.x),
            y: T::sqrt(v.y),
            z: T::sqrt(v.z)
        }
    }

    pub fn length(&self) -> T {
        T::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn length2(&self) -> T {
        self.x*self.x + self.y*self.y + self.z*self.z
    }    
}

impl Vector3<Float> {
    pub fn normalize(&self) -> Vector3<Float> {
        let len_inv = 1.0 / self.length();
        Vector3 {
            x: self.x * len_inv,
            y: self.y * len_inv,
            z: self.z * len_inv
        }
    }

    pub fn dot(v1: Vector3<Float>, v2: Vector3<Float>) -> Float {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn clamp(&self, min: Float, max: Float) -> Vector3<Float> {
        Vector3 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max)
        }
    }

    pub fn cross(v1: Vector3<Float>, v2: Vector3<Float>) -> Vector3<Float> {
        Vector3 {
            x: v1.y() * v2.z() - v1.z() * v2.y(),
            y: -(v1.x() * v2.z() - v1.z() * v2.x()),
            z: v1.x() * v2.y() - v1.y() * v2.x()
        }
    }

    pub fn near_zero(&self) -> bool {
        let eps = Float::from(1e-8);
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn reflect(v: Vector3<Float>, n: Vector3<Float>) -> Vector3<Float> {
        return v - n * Float::from(2.) * Vector3::dot(v, n);
    }

    pub fn refract(v: Vector3<Float>, n: Vector3<Float>, etai_over_etat: Float) -> Vector3<Float> {
        let cos_theta = 1.0.min(Vector3::dot(-v, n));
        let r_out_perp =  etai_over_etat * (v + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length2()).abs().sqrt() * n;
        return r_out_perp + r_out_parallel;
    }
}


// ----------------------------------------------------------------------------
// Operator overloading
// ----------------------------------------------------------------------------
impl<T> Index<usize> for Vector3<T>
    where T: Numeric {
    type Output = T;
    fn index<'a>(&'a self, i: usize) -> &T {
        if i == 0 {
            return &self.x;
        } else if i == 1{
            return &self.y;
        } else {
            return &self.z;
        }
    }
}

impl<T> IndexMut<usize> for Vector3<T>
    where T: Numeric {
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

impl ops::Neg for Vector3<Float> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl<T> ops::Add<Vector3<T>> for Vector3<T>
    where T: Numeric {
    type Output = Vector3<T>;

    fn add(self, _rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl<T> ops::AddAssign<Vector3<T>> for Vector3<T>
    where T: Numeric {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl<T> ops::AddAssign<T> for Vector3<T>
    where T: Numeric {
    fn add_assign(&mut self, other: T) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        };
    }
}

impl<T> ops::Sub<Vector3<T>> for Vector3<T>
    where T: Numeric {
    type Output = Vector3<T>;

    fn sub(self, _rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T> ops::SubAssign<T> for Vector3<T>
    where T: Numeric {
    fn sub_assign(&mut self, other: T) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        };
    }
}

impl<T> ops::SubAssign<Vector3<T>> for Vector3<T>
    where T: Numeric {
    fn sub_assign(&mut self, other: Vector3<T>) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}

impl ops::Mul<Vector3<Float>> for Float {
    type Output = Vector3<Float>;

    fn mul(self, _rhs: Vector3<Float>) -> Self::Output {
        Vector3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl<T> ops::Mul<T> for Vector3<T>
    where T: Numeric {
    type Output = Vector3<T>;

    fn mul(self, _rhs: T) -> Self::Output {
        Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl<T> ops::Mul<Vector3<T>> for Vector3<T>
    where T: Numeric {
    type Output = Vector3<T>;

    fn mul(self, _rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl<T> ops::MulAssign<T> for Vector3<T>
    where T: Numeric {
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl<T> ops::Div<T> for Vector3<T>
    where T: Numeric {
    type Output = Vector3<T>;

    fn div(self, _rhs: T) -> Self::Output  {
        Vector3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl<T> ops::DivAssign<T> for Vector3<T>
    where T: Numeric {
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}

impl<T> std::iter::Sum for Vector3<T>
    where T: Numeric {
    fn sum<I>(iter: I) -> Self
        where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { x: T::default(), y: T::default(), z: T::default() }, |a, b| Self {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z
        })
    }
}

// -----------------------------------------------------------------------------
// Format
// -----------------------------------------------------------------------------
impl<T> std::fmt::Display for Vector3<T>
    where T: Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

// =============================================================================
// 
// =============================================================================
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector2<T>(pub T, pub T) where T: Numeric;

impl<T> Vector2<T>
    where T: Floating {
    pub fn length(&self) -> T {
        T::sqrt(self.x()*self.x() + self.y()*self.y())
    }

    pub fn normalize(&self) -> Vector2<T> {
        let len = self.length();
        Vector2 {
            0: self.x() / len,
            1: self.y() / len
        }
    }
}

impl<T> Vector2<T>
    where T: Numeric {
    pub fn new(v: &[T; 2]) -> Self {
        Vector2 { 0: v[0], 1: v[1] }
    }

    pub fn x(&self) -> T { self.0 }
    pub fn y(&self) -> T { self.1 }

    pub fn length2(&self) -> T {
        self.x()*self.x() + self.y()*self.y()
    }

    pub fn dot(v1: Vector2<T>, v2: Vector2<T>) -> T {
        v1.x() * v2.x() + v1.y() * v2.y()
    }
}

// ----------------------------------------------------------------------------
// Conversion from other types
// ----------------------------------------------------------------------------
impl<T> From<T> for Vector2<T>
    where T: Numeric {
    fn from(item: T) -> Self {
        Vector2 { 0: item, 1: item }
    }
}

// ----------------------------------------------------------------------------
// Operator overloading
// ----------------------------------------------------------------------------
impl ops::Mul<Vector2<Float>> for Float {
    type Output = Vector2<Float>;

    fn mul(self, _rhs: Vector2<Float>) -> Self::Output {
        Vector2 {
            0: self * _rhs.0,
            1: self * _rhs.1,
        }
    }
}

impl ops::Neg for Vector2<Float> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2 { 0: -self.x(), 1: -self.y() }
    }
}


impl<T> ops::Add<Vector2<T>> for Vector2<T>
    where T: Numeric {
    type Output = Vector2<T>;

    fn add(self, _rhs: Vector2<T>) -> Self::Output {
        Vector2 { 0: self.x() + _rhs.x(), 1: self.y() + _rhs.y()}
   }
}

impl<T> ops::AddAssign<Vector2<T>> for Vector2<T>
    where T: Numeric {
    fn add_assign(&mut self, other: Self) {
        *self = Vector2 { 0: self.x() + other.x(), 1: self.y() + other.y()};
    }
}

impl<T> ops::AddAssign<T> for Vector2<T>
    where T: Numeric {
    fn add_assign(&mut self, other: T) {
        *self = Vector2 { 0: self.x() + other, 1: self.y() + other};
    }
}

impl<T> ops::Sub<Vector2<T>> for Vector2<T>
    where T: Numeric {
    type Output = Vector2<T>;

    fn sub(self, _rhs: Vector2<T>) -> Self::Output {
        Vector2 { 0: self.x() - _rhs.x(), 1: self.y() - _rhs.y()}
    }
}

impl<T> ops::SubAssign<T> for Vector2<T>
    where T: Numeric {
    fn sub_assign(&mut self, other: T) {
        *self = Vector2 { 0: self.x() - other, 1: self.y() - other};
    }
}

impl<T> ops::SubAssign<Vector2<T>> for Vector2<T>
    where T: Numeric {
    fn sub_assign(&mut self, other: Vector2<T>) {
        *self = Vector2{ 0: self.x() - other.x(), 1: self.y() - other.y() };
    }
}

impl<T> ops::Mul<T> for Vector2<T>
    where T: Numeric {
    type Output = Vector2<T>;

    fn mul(self, _rhs: T) -> Self::Output {
        Vector2 { 0: self.x() * _rhs, 1: self.y() * _rhs }
    }
}

impl<T> ops::MulAssign<T> for Vector2<T>
    where T: Numeric {
    fn mul_assign(&mut self, other: T) {
        *self = Vector2 { 0: self.x() * other, 1: self.y() * other };
    }
}


impl<T> ops::DivAssign<T> for Vector2<T>
    where T: Numeric {
    fn div_assign(&mut self, other: T) {
        *self = Vector2 { 0: self.x() / other, 1: self.y() / other };
    }
}

// -----------------------------------------------------------------------------
// Format
// -----------------------------------------------------------------------------
impl<T> std::fmt::Display for Vector2<T>
    where T: Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}
